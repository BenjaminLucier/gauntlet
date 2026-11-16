//! Catppuccin everywhere: Latte on light terminals, Mocha on dark ones.
//! Colours follow the Catppuccin style guide (mauve accent, lavender for the
//! focused border, overlay for anything secondary), deepened where Latte's
//! pastels are too faint to read on white. With `NO_COLOR` set, every frame is
//! stripped of colour ([`monochrome`]). Also small layout helpers.

use std::sync::{LazyLock, OnceLock};

use catppuccin::{FlavorColors, PALETTE};
use ratatui::Frame;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Clear};
use terminal_colorsaurus::{QueryOptions, ThemeMode};

use crate::core::Status;
use crate::problem::Difficulty;

/// Whether the terminal has a light or a dark background.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Background {
    Light,
    Dark,
}

impl Background {
    /// `GAUNTLET_THEME=light|dark` if set, otherwise ask the terminal.
    /// Call before the input thread starts, since the terminal answers on stdin.
    pub fn detect() -> Background {
        match std::env::var("GAUNTLET_THEME").as_deref() {
            Ok("light" | "latte") => return Background::Light,
            Ok("dark" | "mocha") => return Background::Dark,
            _ => {}
        }
        match terminal_colorsaurus::theme_mode(QueryOptions::default()) {
            Ok(ThemeMode::Light) => Background::Light,
            Ok(ThemeMode::Dark) => Background::Dark,
            Err(e) => {
                tracing::info!("couldn't detect the terminal background ({e}); assuming dark");
                Background::Dark
            }
        }
    }
}

static BACKGROUND: OnceLock<Background> = OnceLock::new();
static NO_COLOR: LazyLock<bool> = LazyLock::new(crate::ui::no_color);

/// Choose the flavour for this run. Later calls are ignored.
pub fn init(background: Background) {
    let _ = BACKGROUND.set(background);
}

pub fn background() -> Background {
    BACKGROUND.get().copied().unwrap_or(Background::Dark)
}

/// Whether to draw without colour: `NO_COLOR` is set and not empty.
pub fn no_color() -> bool {
    *NO_COLOR
}

pub fn flavor() -> &'static FlavorColors {
    match background() {
        Background::Light => &PALETTE.latte.colors,
        Background::Dark => &PALETTE.mocha.colors,
    }
}

// ── roles ────────────────────────────────────────────────────────────────

/// Contrast every role used for text reaches on the base and popup
/// backgrounds (WCAG AA for body text).
const TEXT_CONTRAST: f64 = 4.5;
/// The focused pane's border is a UI component: WCAG asks 3:1 of those.
const BORDER_CONTRAST: f64 = 3.0;

/// The colour of each role in one flavour.
pub struct Palette {
    base: Color,
    mantle: Color,
    surface: Color,
    selection: Color,
    text: Color,
    subtext: Color,
    muted: Color,
    border: Color,
    focus: Color,
    accent: Color,
    good: Color,
    warn: Color,
    bad: Color,
    star: Color,
    hint: Color,
    cool: Color,
}

impl Palette {
    fn new(f: &FlavorColors) -> Palette {
        let ink = |c| ink_on(f, c, TEXT_CONTRAST);
        Palette {
            base: f.base.into(),
            mantle: f.mantle.into(),
            surface: f.surface0.into(),
            selection: f.surface1.into(),
            text: f.text.into(),
            subtext: ink(f.subtext0),
            muted: ink(f.overlay1),
            border: f.overlay0.into(),
            focus: ink_on(f, f.lavender, BORDER_CONTRAST),
            accent: ink(f.mauve),
            good: ink(f.green),
            warn: ink(f.yellow),
            bad: ink(f.red),
            star: ink(f.pink),
            hint: ink(f.peach),
            cool: ink(f.teal),
        }
    }
}

static LATTE: LazyLock<Palette> = LazyLock::new(|| Palette::new(&PALETTE.latte.colors));
static MOCHA: LazyLock<Palette> = LazyLock::new(|| Palette::new(&PALETTE.mocha.colors));

fn palette() -> &'static Palette {
    match background() {
        Background::Light => &LATTE,
        Background::Dark => &MOCHA,
    }
}

/// `c` blended toward the flavour's text colour until it has `ratio` contrast
/// on both the base and the mantle (popups). Mocha's colours mostly already
/// do; Latte's yellow, peach, pink and overlays are too faint on white.
fn ink_on(f: &FlavorColors, c: catppuccin::Color, ratio: f64) -> Color {
    let rgb = |c: catppuccin::Color| [c.rgb.r, c.rgb.g, c.rgb.b].map(f64::from);
    let (from, to) = (rgb(c), rgb(f.text));
    (0..=20)
        .map(|step| {
            let t = f64::from(step) / 20.0;
            [0, 1, 2].map(|i| from[i] + (to[i] - from[i]) * t)
        })
        .find(|&mix| contrast(mix, rgb(f.base)).min(contrast(mix, rgb(f.mantle))) >= ratio)
        .map_or(f.text.into(), |[r, g, b]| Color::Rgb(r.round() as u8, g.round() as u8, b.round() as u8))
}

/// `c` made readable as text in the current flavour.
pub fn ink(c: catppuccin::Color) -> Color {
    ink_on(flavor(), c, TEXT_CONTRAST)
}

/// WCAG contrast ratio of two sRGB colours (channels 0–255).
fn contrast(a: [f64; 3], b: [f64; 3]) -> f64 {
    let luminance = |c: [f64; 3]| {
        let [r, g, b] = c.map(|v| {
            let v = v / 255.0;
            if v <= 0.039_28 { v / 12.92 } else { ((v + 0.055) / 1.055).powf(2.4) }
        });
        0.2126 * r + 0.7152 * g + 0.0722 * b
    };
    let (la, lb) = (luminance(a), luminance(b));
    (la.max(lb) + 0.05) / (la.min(lb) + 0.05)
}

pub fn base() -> Color {
    palette().base
}
pub fn mantle() -> Color {
    palette().mantle
}
pub fn surface() -> Color {
    palette().surface
}
/// Selection and highlighted rows.
pub fn selection() -> Color {
    palette().selection
}
pub fn text() -> Color {
    palette().text
}
pub fn subtext() -> Color {
    palette().subtext
}
/// Secondary text and labels.
pub fn muted() -> Color {
    palette().muted
}
/// Unfocused borders.
pub fn border() -> Color {
    palette().border
}
/// The focused pane's border.
pub fn focus() -> Color {
    palette().focus
}
pub fn accent() -> Color {
    palette().accent
}
pub fn good() -> Color {
    palette().good
}
pub fn warn() -> Color {
    palette().warn
}
pub fn bad() -> Color {
    palette().bad
}
/// Personal bests and runs.
pub fn star() -> Color {
    palette().star
}
/// Hints.
pub fn hint() -> Color {
    palette().hint
}

pub fn dim() -> Style {
    Style::new().fg(muted())
}

/// The base layer every frame is drawn on.
pub fn canvas() -> Style {
    Style::new().fg(text()).bg(base())
}

/// Border style for a pane, depending on whether it has focus.
pub fn pane_border(focused: bool) -> Style {
    if focused { Style::new().fg(focus()) } else { Style::new().fg(border()) }
}

/// A label on a block of `colour`: the active tab, a status badge.
/// Reversed rather than given a background, so it stands out without colour too.
pub fn badge(colour: Color) -> Style {
    Style::new().fg(colour).reversed().bold()
}

/// The selected row of a list or table: a highlight bar when focused,
/// underlined accent when focus is elsewhere.
pub fn selected(focused: bool) -> Style {
    if focused { Style::new().bg(selection()).bold() } else { Style::new().fg(accent()).bold().underlined() }
}

/// `NO_COLOR`: strip every colour from a drawn frame, keeping what the
/// colours meant: the selection bar becomes reverse video, and secondary
/// text and unfocused borders are dimmed.
pub fn monochrome(buf: &mut Buffer) {
    let (bar, quiet) = (selection(), [muted(), border()]);
    for cell in &mut buf.content {
        let mut modifier = cell.modifier;
        if cell.bg == bar {
            modifier |= Modifier::REVERSED;
        }
        if quiet.contains(&cell.fg) {
            modifier |= Modifier::DIM;
        }
        cell.set_style(Style::reset());
        cell.modifier = modifier;
    }
}

/// Map the 16 ANSI colours (from compiler and test output) into the palette.
pub fn ansi_color(c: Color) -> Color {
    let f = flavor();
    match c {
        Color::Black => f.surface1.into(),
        Color::Red | Color::LightRed => bad(),
        Color::Green | Color::LightGreen => good(),
        Color::Yellow | Color::LightYellow => warn(),
        Color::Blue | Color::LightBlue => ink(f.blue),
        Color::Magenta | Color::LightMagenta => star(),
        Color::Cyan | Color::LightCyan => palette().cool,
        Color::Gray => ink(f.subtext1),
        Color::DarkGray => f.surface2.into(),
        Color::White => text(),
        other => other,
    }
}

/// Recolour ANSI-styled text into the palette.
pub fn recolor(mut text: Text<'static>) -> Text<'static> {
    for line in &mut text.lines {
        for span in &mut line.spans {
            span.style.fg = span.style.fg.map(ansi_color);
            span.style.bg = span.style.bg.map(ansi_color);
        }
    }
    text
}

// ── text ─────────────────────────────────────────────────────────────────

/// `line` cut to `width` columns, ending in `…` if anything was cut.
pub fn fit(line: Line<'static>, width: usize) -> Line<'static> {
    if line.width() <= width {
        return line;
    }
    let mut room = width.saturating_sub(1);
    let mut spans = Vec::new();
    for span in &line.spans {
        let mut cut = String::new();
        for g in span.styled_graphemes(Style::new()) {
            let w = Span::raw(g.symbol).width();
            if w > room {
                break;
            }
            room -= w;
            cut.push_str(g.symbol);
        }
        let whole = cut.len() == span.content.len();
        spans.push(Span::styled(cut, span.style));
        if !whole {
            break;
        }
    }
    if width > 0 {
        let style = spans.last().map_or(Style::new(), |s| s.style);
        spans.push(Span::styled("…", style));
    }
    Line { spans, ..line }
}

/// `text` cut from the start to `width` columns, beginning with `…` if
/// anything was cut: the end of a path is the part that tells it apart.
pub fn fit_start(text: &str, width: usize) -> String {
    if Span::raw(text).width() <= width {
        return text.to_string();
    }
    let mut room = width.saturating_sub(1);
    let mut tail = Vec::new();
    for c in text.chars().rev() {
        let w = Span::raw(c.to_string()).width();
        if w > room {
            break;
        }
        room -= w;
        tail.push(c);
    }
    let tail: String = tail.into_iter().rev().collect();
    if width == 0 { tail } else { format!("…{tail}") }
}

// ── widgets ──────────────────────────────────────────────────────────────

pub fn difficulty_color(d: Difficulty) -> Color {
    match d {
        Difficulty::Easy => good(),
        Difficulty::Medium => warn(),
        Difficulty::Hard => bad(),
    }
}

pub fn difficulty(d: Difficulty) -> Span<'static> {
    d.label().fg(difficulty_color(d))
}

pub fn status_color(s: Status) -> Color {
    match s {
        Status::Solved => good(),
        Status::InProgress => warn(),
        Status::Unseen => muted(),
    }
}

pub fn status_icon(s: Status) -> Span<'static> {
    let icon = match s {
        Status::Solved => "✓",
        Status::InProgress => "…",
        Status::Unseen => "·",
    };
    icon.fg(status_color(s))
}

pub fn status_badge(s: Status) -> Span<'static> {
    let label = match s {
        Status::Solved => " ✓ solved ",
        Status::InProgress => " … in progress ",
        Status::Unseen => " · new ",
    };
    Span::styled(label, badge(status_color(s)))
}

/// `label ……… value` as a line with a fixed-width label column.
pub fn kv<'a>(label: &str, value: impl Into<Line<'a>>) -> Line<'a> {
    let mut line = Line::from(Span::styled(format!("{label:<14}"), dim()));
    line.spans.extend(value.into().spans);
    line
}

/// Clear a centred `width` × `height` area for a popup, painted in the
/// popup colour, and return it.
pub fn popup(frame: &mut Frame, width: u16, height: u16) -> Rect {
    let area = centered(frame.area(), width, height);
    frame.render_widget(Clear, area);
    frame.render_widget(Block::new().style(Style::new().fg(text()).bg(mantle())), area);
    area
}

/// A `width` × `height` rectangle centred in `area` (clamped to fit).
pub fn centered(area: Rect, width: u16, height: u16) -> Rect {
    let [row] = Layout::vertical([Constraint::Length(height.min(area.height))]).flex(Flex::Center).areas(area);
    let [rect] = Layout::horizontal([Constraint::Length(width.min(area.width))]).flex(Flex::Center).areas(row);
    rect
}

/// Colour for a rating of "I found this problem to be trivial.": hard
/// (disagree) is warm, trivial (agree) is cool.
pub fn rating_color(r: Option<crate::store::Rating>) -> Color {
    match r.map(crate::store::Rating::value) {
        None => muted(),
        Some(1) => bad(),
        Some(2) => hint(),
        Some(3) => warn(),
        Some(4) => good(),
        Some(_) => palette().cool,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_text_role_reads_on_light_and_dark() {
        for f in [&PALETTE.latte.colors, &PALETTE.mocha.colors] {
            let p = Palette::new(f);
            let rgb = |c: Color| match c {
                Color::Rgb(r, g, b) => [r, g, b].map(f64::from),
                other => panic!("{other:?}"),
            };
            let text = [p.text, p.subtext, p.muted, p.accent, p.good, p.warn, p.bad, p.star, p.hint, p.cool];
            for (i, c) in text.into_iter().enumerate() {
                for bg in [p.base, p.mantle] {
                    let ratio = contrast(rgb(c), rgb(bg));
                    assert!(ratio >= TEXT_CONTRAST, "role {i} on {bg:?}: {ratio:.2}");
                }
            }
            assert!(contrast(rgb(p.focus), rgb(p.base)) >= BORDER_CONTRAST);
        }
    }

    #[test]
    fn fitting_cuts_with_an_ellipsis() {
        let line = Line::from(vec!["Two ".bold(), "Sum".into()]);
        assert_eq!(fit(line.clone(), 7).to_string(), "Two Sum");
        assert_eq!(fit(line.clone(), 6).to_string(), "Two S…");
        assert_eq!(fit(line.clone(), 4).to_string(), "Two…");
        assert_eq!(fit(line, 0).to_string(), "");
        // Never half a wide character.
        assert_eq!(fit(Line::from("日本語"), 4).to_string(), "日…");
        assert_eq!(fit_start("~/data/two_sum.rs", 12), "…/two_sum.rs");
        assert_eq!(fit_start("two_sum.rs", 12), "two_sum.rs");
    }
}
