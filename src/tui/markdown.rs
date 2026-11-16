//! Markdown rendered for the terminal rather than shown as source: no `#`
//! heading markers or code fences, Catppuccin colours (as in Helix), and code
//! highlighted with the matching Catppuccin TextMate theme.

use std::sync::LazyLock;

use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span, Text};
use tui_markdown::{BuiltinCodeTheme, CodeTheme, Options, StyleSheet};

use super::theme::{self, Background};

// The official Catppuccin TextMate themes (github.com/catppuccin/bat, MIT).
static LATTE: LazyLock<CodeTheme> = LazyLock::new(|| {
    code_theme(include_str!("../../assets/themes/catppuccin-latte.tmTheme"), BuiltinCodeTheme::InspiredGitHub)
});
static MOCHA: LazyLock<CodeTheme> = LazyLock::new(|| {
    code_theme(include_str!("../../assets/themes/catppuccin-mocha.tmTheme"), BuiltinCodeTheme::Base16OceanDark)
});

fn code_theme(source: &str, fallback: BuiltinCodeTheme) -> CodeTheme {
    CodeTheme::from_textmate(source).unwrap_or_else(|e| {
        tracing::warn!("couldn't load the Catppuccin code theme ({e}); using {fallback:?}");
        fallback.into()
    })
}

/// Markdown styles, after Catppuccin's Helix theme.
#[derive(Clone, Copy)]
struct Sheet;

impl StyleSheet for Sheet {
    fn heading(&self, level: u8) -> Style {
        let f = theme::flavor();
        let color = match level {
            1 => f.lavender,
            2 => f.mauve,
            3 => f.green,
            4 => f.yellow,
            5 => f.pink,
            _ => f.teal,
        };
        Style::new().fg(theme::ink(color)).add_modifier(Modifier::BOLD)
    }

    fn heading_marker(&self, _level: u8) -> &str {
        ""
    }

    /// A sentinel, not shown: [`render`] replaces fenced blocks with a gutter.
    fn code_block_fence(&self) -> &str {
        FENCE
    }

    fn code(&self) -> Style {
        Style::new().fg(theme::good())
    }

    fn link(&self) -> Style {
        Style::new().fg(theme::ink(theme::flavor().rosewater)).add_modifier(Modifier::UNDERLINED)
    }

    fn blockquote(&self) -> Style {
        Style::new().fg(theme::subtext())
    }

    fn table_header(&self) -> Style {
        Style::new().fg(theme::text()).add_modifier(Modifier::BOLD)
    }

    fn table_border(&self) -> Style {
        Style::new().fg(theme::border())
    }

    fn list_marker(&self) -> Style {
        Style::new().fg(theme::accent())
    }
}

/// Marks the fence lines around code blocks so [`render`] can find them.
const FENCE: &str = "\u{1}";
const GUTTER: &str = "  \u{2502} ";

/// Render a markdown document. The result owns its text, so it can be
/// cached and drawn every frame without re-parsing.
pub fn render(src: &str) -> Text<'static> {
    let code_theme = match theme::background() {
        Background::Light => &*LATTE,
        Background::Dark => &*MOCHA,
    };
    let options = Options::new(Sheet).code_theme(code_theme.clone());
    let text = tui_markdown::from_str_with_options(src, &options);
    let mut lines = Vec::with_capacity(text.lines.len());
    let mut in_code = false;
    for line in text.lines {
        if line.spans.first().is_some_and(|s| s.content.starts_with(FENCE)) {
            in_code = !in_code;
            continue;
        }
        let gutter = in_code.then(|| Span::styled(GUTTER, Style::new().fg(theme::border())));
        let spans = gutter
            .into_iter()
            .chain(line.spans.into_iter().map(|s| Span::styled(s.content.into_owned(), s.style)))
            .collect();
        lines.push(Line { spans, ..line });
    }
    Text { lines, ..text }
}

/// Render source code as a highlighted block.
pub fn code(src: &str) -> Text<'static> {
    render(&format!("```rust\n{}\n```\n", src.trim_end()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hides_markdown_syntax() {
        let src = "# Title\n\nSome `code`.\n\n```rust\nlet x = 1;\n```\n\nAfter.\n";
        let lines: Vec<String> = render(src).lines.iter().map(ToString::to_string).collect();
        assert_eq!(lines[0], "Title");
        assert!(lines.contains(&format!("{GUTTER}let x = 1;")), "{lines:?}");
        assert!(lines.iter().all(|l| !l.contains("```") && !l.contains(FENCE)), "{lines:?}");
        assert_eq!(lines.last().unwrap(), "After.");
    }

    #[test]
    fn code_is_highlighted_with_catppuccin() {
        use ratatui::style::Color;
        // Keywords are Catppuccin's mauve. Each theme is checked directly,
        // since the flavour for a run is chosen once.
        for (theme, mauve) in [(&*MOCHA, Color::Rgb(0xcb, 0xa6, 0xf7)), (&*LATTE, Color::Rgb(0x88, 0x39, 0xef))] {
            let options = Options::new(Sheet).code_theme(theme.clone());
            let text = tui_markdown::from_str_with_options("```rust\nfn main() {}\n```\n", &options);
            let fn_kw = text.lines.iter().flat_map(|l| &l.spans).find(|s| s.content.trim() == "fn");
            assert_eq!(fn_kw.and_then(|s| s.style.fg), Some(mauve), "{:?}", text.lines);
        }
    }
}
