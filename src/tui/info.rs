//! The Info tab: the statement (or, if you ask, the tests or the solution) on
//! the left; hints and the target complexity on the right. Hints are shown one
//! at a time so you can stop at the gentlest one that helps; nothing is
//! recorded. In a run there are no hints and no solution, just the statement,
//! the tests and the target.

use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Stylize;
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, Wrap};

use super::theme::{self, accent, muted, star};
use super::{App, InfoPane, InfoView};
use crate::problem::{SOLUTION, TESTS};

pub fn draw(app: &mut App, frame: &mut Frame, area: Rect) {
    let in_run = app.in_run();
    let side_width = (area.width / 3).clamp(32, 48);
    let [main, side] = Layout::horizontal([Constraint::Fill(1), Constraint::Length(side_width)]).areas(area);
    let complexity = if app.complexity.lines.is_empty() {
        Text::from("No target for this one.".fg(muted()))
    } else {
        app.complexity.clone()
    };
    let complexity = Paragraph::new(complexity).wrap(Wrap { trim: false });
    let complexity_height =
        u16::try_from(complexity.line_count(side.width.saturating_sub(2))).unwrap_or(u16::MAX).saturating_add(2);
    let target_height = if in_run { side.height } else { complexity_height.min(side.height / 2) };
    let [hints, target] = Layout::vertical([Constraint::Fill(1), Constraint::Length(target_height)]).areas(side);

    let statement_focused = in_run || app.info_focus == InfoPane::Statement;
    match app.info_view {
        InfoView::Solution if !in_run => draw_code(app, frame, main, statement_focused, InfoView::Solution),
        InfoView::Tests => draw_code(app, frame, main, statement_focused, InfoView::Tests),
        _ => draw_statement(app, frame, main, statement_focused),
    }
    if !in_run {
        draw_hints(app, frame, hints, app.info_focus == InfoPane::Hints);
    }
    let mut block = pane(" Target ".bold(), false);
    if in_run {
        block = block.title_bottom(Line::from(" no hints or solution in a run ".fg(muted())).right_aligned());
    }
    frame.render_widget(complexity.block(block), target);
}

fn pane(title: Span<'static>, focused: bool) -> Block<'static> {
    Block::bordered().title(title).border_style(theme::pane_border(focused))
}

/// A wrapped, scrollable paragraph with a scrollbar when it overflows.
fn scrolled(frame: &mut Frame, area: Rect, text: Text<'static>, block: Block<'static>, offset: &mut u16) {
    let inner = block.inner(area);
    let paragraph = Paragraph::new(text).wrap(Wrap { trim: false });
    let lines = paragraph.line_count(inner.width);
    let max = lines.saturating_sub(inner.height as usize);
    *offset = (*offset).min(u16::try_from(max).unwrap_or(u16::MAX));
    frame.render_widget(paragraph.scroll((*offset, 0)).block(block), area);
    if max > 0 {
        let mut state = ScrollbarState::new(max).position(*offset as usize);
        frame.render_stateful_widget(Scrollbar::new(ScrollbarOrientation::VerticalRight), area, &mut state);
    }
}

fn draw_statement(app: &mut App, frame: &mut Frame, area: Rect, focused: bool) {
    let p = app.g.problem(app.idx);
    let title = Line::from(vec![
        format!(" {} ", p.title).bold(),
        theme::difficulty(p.difficulty),
        format!(" · {} ", app.g.ws.location(app.idx)).fg(muted()),
    ]);
    // Clear of the corners and the scrollbar's arrow.
    let title = theme::fit(title, usize::from(area.width.saturating_sub(3)));
    let mut keys = Vec::new();
    if !app.in_run() {
        keys.extend([" v".fg(accent()), " solution ".fg(muted())]);
    }
    keys.extend([" t".fg(accent()), " tests ".fg(muted())]);
    let block = Block::bordered()
        .title(title)
        .title_bottom(Line::from(keys).right_aligned())
        .border_style(theme::pane_border(focused));
    scrolled(frame, area, app.statement.clone(), block, &mut app.statement_scroll);
}

/// The reference solution or the tests, highlighted.
fn draw_code(app: &mut App, frame: &mut Frame, area: Rect, focused: bool, view: InfoView) {
    let (title, file, key, text, offset) = match view {
        InfoView::Tests => (" Tests ".fg(accent()), TESTS, " t", app.tests.clone(), &mut app.tests_scroll),
        _ => (" Solution ".fg(star()), SOLUTION, " v", app.solution.clone(), &mut app.solution_scroll),
    };
    let block = Block::bordered()
        .title(Line::from(vec![title.bold(), format!("{file} ").fg(muted())]))
        .title_bottom(Line::from(vec![key.fg(accent()), " back to the statement ".fg(muted())]).right_aligned())
        .border_style(theme::pane_border(focused));
    scrolled(frame, area, text, block, offset);
}

fn draw_hints(app: &mut App, frame: &mut Frame, area: Rect, focused: bool) {
    let total = app.hint_texts.len();
    let shown = app.hints_shown.min(total);
    let mut lines: Vec<Line<'static>> = Vec::new();
    for (i, hint) in app.hint_texts.iter().enumerate() {
        let n = i + 1;
        if i < shown {
            lines.push(Line::from(format!("💡 Hint {n}").fg(theme::hint()).bold()));
            lines.extend(hint.lines.iter().cloned());
        } else if i == shown {
            let label = format!("▸ Hint {n}");
            lines.push(if focused {
                Line::from(vec![label.fg(accent()).bold(), "   Enter to show".fg(muted())])
            } else {
                Line::from(vec![label.fg(theme::text()), "   Tab, then Enter".fg(muted())])
            });
        } else {
            lines.push(Line::from(format!("· Hint {n}").fg(muted())));
        }
        lines.push(Line::default());
    }
    if total == 0 {
        lines.push(Line::from("No hints for this one.".fg(muted()).italic()));
    }
    let block = Block::bordered()
        .title(Line::from(vec![" Hints ".bold(), format!("{shown}/{total} ").fg(theme::hint())]))
        .border_style(theme::pane_border(focused));
    scrolled(frame, area, Text::from(lines), block, &mut app.hints_scroll);
}
