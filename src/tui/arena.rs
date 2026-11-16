//! The Arena tab: the problem you're on and the test output, after a pass
//! with the tests' runtime and memory against the reference solution's. Also
//! the popups that follow a solve: the rating, and the end of a run.

use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Clear, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, Wrap};
use throbber_widgets_tui::Throbber;

use super::theme::{self, accent, bad, good, kv, muted, star, warn};
use super::{App, RunOver, Verdict};
use crate::core::Split;
use crate::dirs::show;
use crate::fmt;
use crate::perf::{self, Nudge, Perf};
use crate::runner::SENTINEL;
use crate::store::Rating;

pub fn draw(app: &mut App, frame: &mut Frame, area: Rect) {
    let paused = app.in_run() && app.g.run.as_ref().is_some_and(|s| s.paused);
    let [header, banner, output] =
        Layout::vertical([Constraint::Length(5), Constraint::Length(u16::from(paused)), Constraint::Fill(1)])
            .areas(area);

    draw_header(app, frame, header);
    if paused {
        frame.render_widget(
            Paragraph::new(Line::from(vec![
                " ⏸ Paused: the clock is stopped. Press ".into(),
                "p".bold(),
                " to resume.".into(),
            ]))
            .style(theme::badge(warn()).not_bold()),
            banner,
        );
    }
    draw_output(app, frame, output);
    draw_toasts(app, frame, output);
}

fn draw_header(app: &App, frame: &mut Frame, area: Rect) {
    let g = &*app.g;
    let idx = app.idx;
    let p = g.problem(idx);
    let width = usize::from(area.width.saturating_sub(2));
    // In a run, completionist status is beside the point.
    let badge = (!app.in_run()).then(|| Line::from(vec![theme::status_badge(g.status(idx)), " ".into()]));
    let title = Line::from(vec![
        format!(" {} ", p.title).bold(),
        theme::difficulty(p.difficulty),
        format!(" · {} ", g.ws.location(idx)).fg(muted()),
    ]);
    let title = theme::fit(title, width.saturating_sub(badge.as_ref().map_or(0, |b| b.width() + 1)));

    let history = g.db.history(&p.id);
    let mut facts: Vec<Span> = Vec::new();
    let second = if let Some(state) = g.run.as_ref().filter(|_| app.in_run()) {
        // The clock goes last, so nothing moves as it gains a digit.
        match g.db.problem_best(&p.id, Some(state.run)) {
            Some(pb) => facts.push(format!("best {} (run {})", fmt::secs(pb.secs), pb.run).fg(muted())),
            None => facts.push("no best yet".fg(muted())),
        }
        facts.extend(["  ·  on this problem ⏱ ".fg(muted()), fmt::secs(state.time_on(&p.id).as_secs_f64()).bold()]);
        let (done, total) = g.run_progress().unwrap_or_default();
        Line::from(
            format!("{} solved in run {} ({})", fmt::count(done, total), state.run, g.ws.scope_title(&state.scope))
                .fg(muted()),
        )
    } else {
        let solves = history.len();
        facts.push(match solves {
            0 => "never solved".fg(muted()),
            1 => "solved once".into(),
            n => format!("solved {n} times").into(),
        });
        if let Some(r) = g.db.latest_rating(&p.id) {
            facts.push("  ·  trivial? ".fg(muted()));
            facts.push(r.label().fg(theme::rating_color(Some(r))));
        }
        if let Some(pb) = g.db.problem_best(&p.id, None) {
            facts.push(format!("  ·  run best {}", fmt::secs(pb.secs)).fg(muted()));
        }
        review_line(app)
    };

    // The working file, and the keys after it if there's room.
    let file = show(&g.working_file(idx));
    let keys = if app.in_run() {
        ["p".fg(accent()).bold(), " pause  ".fg(muted()), "n".fg(accent()).bold(), " skip".fg(muted())]
    } else {
        ["i".fg(accent()).bold(), " info  ".fg(muted()), "h".fg(accent()).bold(), " hints".fg(muted())]
    };
    let mut file = Line::from(theme::fit_start(&file, width).fg(accent()));
    if file.width() + 3 + keys.iter().map(Span::width).sum::<usize>() <= width {
        file.spans.push("   ".into());
        file.spans.extend(keys);
    }

    let mut block = Block::bordered().title(title).border_style(theme::pane_border(false));
    if let Some(badge) = badge {
        block = block.title_top(badge.right_aligned());
    }
    let lines = [Line::from(facts), second, file].map(|l| theme::fit(l, width));
    frame.render_widget(Paragraph::new(Text::from(Vec::from(lines))).block(block), area);
}

/// What to do next with a problem up for review (rated 1–2, or off the
/// reference's pace), or with one a review session has seen re-rated since.
fn review_line(app: &App) -> Line<'static> {
    let g = &*app.g;
    let id = &g.problem(app.idx).id;
    let key = |k: &'static str| k.fg(accent()).bold();
    let review = "↻ Review: ".fg(warn()).bold();
    let armed = g.db.completionist_armed(id);
    if let (None, Some(nudge)) = (g.db.review_rating(id), g.db.nudge(id)) {
        return Line::from(if armed {
            vec![review, format!("fresh template. Your best solution {}.", nudge.says()).into()]
        } else {
            vec![review, format!("your best solution {}. Improve it and save.", nudge.says()).into()]
        });
    }
    match g.db.latest_rating(id) {
        Some(_) if g.db.up_for_review(id) && armed => {
            Line::from(vec![review, "fresh template. Pass the tests, then rate it again.".into()])
        }
        Some(r) if g.db.up_for_review(id) => Line::from(vec![
            "↻ Review: ".fg(warn()).bold(),
            "you rated it ".into(),
            r.label().fg(theme::rating_color(Some(r))),
            ". ".into(),
            key("x"),
            " for a fresh template, solve it again, rate it again.".into(),
        ]),
        Some(r) if g.review => {
            let mut spans =
                vec!["✓ Reviewed: ".fg(good()).bold(), "now ".into(), r.label().fg(theme::rating_color(Some(r)))];
            spans.extend(match g.next_up() {
                Some(_) => vec![". ".into(), key("u"), " for the next one.".into()],
                None => vec![". That was the last one up for review.".into()],
            });
            Line::from(spans)
        }
        _ => Line::default(),
    }
}

fn draw_output(app: &mut App, frame: &mut Frame, area: Rect) {
    let verdict = match app.verdict {
        None => Span::raw(""),
        Some(Verdict::Passed(n)) => format!(" ✓ {n} passed ").fg(good()).bold(),
        // Exited early, or ran nothing.
        Some(Verdict::Failed { failed: 0, .. }) => " ✗ no test result ".fg(bad()).bold(),
        Some(Verdict::Failed { passed, failed }) => format!(" ✗ {failed} failed, {passed} passed ").fg(bad()).bold(),
        Some(Verdict::CompileError) => " ✗ doesn't compile ".fg(bad()).bold(),
        Some(Verdict::Timeout) => " ✗ timed out ".fg(bad()).bold(),
        Some(Verdict::NotStarted) => " not started ".fg(warn()).bold(),
        Some(Verdict::Lint { clean: true }) => " ✓ no notes ".fg(good()).bold(),
        Some(Verdict::Lint { clean: false }) => " suggestions ".fg(warn()).bold(),
    };
    let mut block = Block::bordered()
        .title(Line::from(vec![format!(" {} ", app.output_title).bold(), verdict]))
        .border_style(theme::pane_border(false));
    if let Some(label) = app.busy {
        let throbber = Throbber::default()
            .label(format!("{label}… "))
            .style(Style::new().fg(accent()))
            .throbber_set(throbber_widgets_tui::BRAILLE_SIX);
        block = block.title_top(throbber.to_line(&app.throbber).right_aligned());
    }

    let inner = block.inner(area);
    let mut text = Text::from(perf_lines(app));
    text.lines.extend(app.output.lines.iter().cloned());
    let lines = text.lines.len();
    let max_scroll = lines.saturating_sub(inner.height as usize);
    app.output_scroll = app.output_scroll.min(u16::try_from(max_scroll).unwrap_or(u16::MAX));
    frame.render_widget(
        Paragraph::new(text).wrap(Wrap { trim: false }).scroll((app.output_scroll, 0)).block(block),
        area,
    );
    if lines > inner.height as usize {
        let mut state = ScrollbarState::new(max_scroll).position(app.output_scroll as usize);
        frame.render_stateful_widget(Scrollbar::new(ScrollbarOrientation::VerticalRight), area, &mut state);
    }
}

/// After a pass, above the tests' output: their runtime and peak memory, and
/// the reference solution's once it's been measured, with a nudge where
/// they're off its pace. Columns are fixed, so nothing moves when the
/// reference arrives.
fn perf_lines(app: &App) -> Vec<Line<'static>> {
    let Some(mine) = app.perf.filter(|_| matches!(app.verdict, Some(Verdict::Passed(_)))) else {
        return Vec::new();
    };
    let reference = app.g.db.reference(&app.g.problem(app.idx).id);
    let nudge = reference.and_then(|r| Nudge::of(mine, r));
    let row = |label: &str, mine: Option<u64>, theirs: Option<u64>, show: fn(u64) -> String, off: bool, says| {
        let mut spans = vec![format!(" {label:<8}").fg(muted()), show(mine?).bold()];
        if let (Some(theirs), Some(ratio)) = (theirs, perf::ratio(mine, theirs)) {
            let colour = if off { warn() } else { muted() };
            spans.extend([
                "  reference ".fg(muted()),
                show(theirs).into(),
                format!("  {}", fmt::ratio(ratio)).fg(colour),
            ]);
            if off {
                spans.push(format!("  ▲ this {says}").fg(warn()).bold());
            }
        }
        Some(Line::from(spans))
    };
    let (slower, hungrier) = nudge.map_or((false, false), |n| (n.slower, n.hungrier));
    let mut lines: Vec<Line> = [
        row("Tests", mine.ms, reference.and_then(|r| r.ms), fmt::runtime, slower, "could probably be faster"),
        row(
            "Memory",
            mine.bytes,
            reference.and_then(|r| r.bytes),
            fmt::memory,
            hungrier,
            "could probably use less memory",
        ),
    ]
    .into_iter()
    .flatten()
    .collect();
    // libtest's output starts with a blank line of its own.
    if app.output.lines.first().is_none_or(|l| l.width() > 0) {
        lines.push(Line::default());
    }
    lines
}

/// In a run, where the next problem follows a pass at once: a line for the
/// solve's toast, if it's off the reference's pace.
pub fn nudge_toast(mine: Perf, reference: Option<Perf>) -> Option<Line<'static>> {
    let reference = reference?;
    let nudge = Nudge::of(mine, reference)?;
    let (ratio, what) = if nudge.slower {
        (perf::ratio(mine.ms, reference.ms)?, "runtime")
    } else {
        (perf::ratio(mine.bytes, reference.bytes)?, "memory")
    };
    let ratio = fmt::ratio(ratio);
    Some(Line::from(vec![
        "▲ ".fg(warn()),
        format!("{} the reference's {what}: it {}", ratio.trim_start(), nudge.says()).fg(muted()),
    ]))
}

/// Run solves and splits, in a box at the bottom right of the output.
fn draw_toasts(app: &App, frame: &mut Frame, area: Rect) {
    if app.toasts.is_empty() {
        return;
    }
    let lines: Vec<Line> = app.toasts.iter().map(|(l, _)| l.clone()).collect();
    let width = lines.iter().map(Line::width).max().unwrap_or(0) + 4;
    let width = u16::try_from(width).unwrap_or(u16::MAX).min(area.width.saturating_sub(2));
    let height = u16::try_from(lines.len() + 2).unwrap_or(u16::MAX).min(area.height.saturating_sub(1));
    let rect =
        Rect { x: area.right().saturating_sub(width + 1), y: area.bottom().saturating_sub(height + 1), width, height };
    frame.render_widget(Clear, rect);
    let block = Block::bordered()
        .title(" Splits ".fg(star()).bold())
        .border_style(Style::new().fg(star()))
        .style(Style::new().bg(theme::mantle()));
    frame.render_widget(Paragraph::new(lines).block(block), rect);
}

/// `  -0:12 ★ new best` / `  +0:30 behind`, given the seconds ahead of the
/// previous best, or `  first time` without one.
pub fn versus(ahead_by: Option<f64>) -> Vec<Span<'static>> {
    match ahead_by {
        Some(a) if a > 0.0 => vec![format!("  {} ", fmt::delta(-a)).fg(good()).bold(), "★ new best".fg(star())],
        Some(a) => vec![format!("  {} ", fmt::delta(-a)).fg(bad()).bold(), "behind".fg(muted())],
        None => vec!["  first time".fg(muted())],
    }
}

/// `Pure DSA · Trees  12:03  -0:41 ★ new best`
pub fn split_line(title: &str, split: &Split) -> Line<'static> {
    let mut spans = vec!["◆ ".fg(star()), title.to_string().bold(), format!("  {}", fmt::secs(split.secs)).bold()];
    spans.extend(versus(split.ahead_by()));
    Line::from(spans)
}

/// The rating popup for problem `idx`, which may not be the one on screen.
pub fn draw_rate(idx: usize, app: &App, frame: &mut Frame) {
    let p = app.g.problem(idx);
    let area = theme::popup(frame, 60, 15);
    let mut lines = vec![
        Line::default(),
        Line::from(vec!["  How much do you agree?".fg(muted())]),
        Line::default(),
        Line::from(format!("  “{}”", Rating::STATEMENT).bold()),
        Line::default(),
    ];
    for r in Rating::ALL {
        lines.push(Line::from(vec![
            format!("    [{}] ", r.key()).fg(accent()).bold(),
            r.label().fg(theme::rating_color(Some(r))),
        ]));
    }
    lines.push(Line::default());
    lines.push(Line::from("  Press 1–5: a rating is required.".fg(muted())));
    let block = Block::bordered()
        .title(format!(" ✓ Solved: {} ", p.title).fg(good()).bold())
        .border_style(Style::new().fg(good()));
    frame.render_widget(Paragraph::new(lines).block(block), area);
}

pub fn draw_confirm_redo(app: &App, frame: &mut Frame) {
    let area = theme::popup(frame, 62, 9);
    let file = show(&app.g.working_file(app.idx));
    let lines = vec![
        Line::default(),
        Line::from(vec!["  Overwrite ".bold(), file.fg(accent()), " with".bold()]),
        Line::from("  a fresh template? Your code there is gone (not archived).".bold()),
        Line::from("  Passing it again counts as another solve, with a rating.".fg(muted())),
        Line::default(),
        Line::from(vec!["  [x]".fg(accent()).bold(), " redo   ".into(), "any other key".fg(muted()), " cancel".into()]),
    ];
    let block = Block::bordered().title(" Redo? ".fg(warn()).bold()).border_style(Style::new().fg(warn()));
    frame.render_widget(Paragraph::new(lines).block(block), area);
}

/// The end of a run: its total, and every scope it covered against the PB
/// from other runs.
pub fn draw_run_over(over: &RunOver, app: &App, frame: &mut Frame) {
    let g = &*app.g;
    let run = g.db.run(over.run);
    let total = run.map_or(0.0, crate::db::Run::total);
    let count = run.map_or(0, crate::db::Run::solved);
    let mut lines = vec![
        Line::default(),
        kv("  Scope", g.ws.scope_title(&over.scope).bold()),
        kv("  Total", Line::from(vec![fmt::secs(total).bold(), format!("  over {count} problems").fg(muted())])),
    ];
    let (last, secs) = &over.last;
    let last_title = g.idx_of(last).map_or(last.clone(), |i| g.problem(i).title.clone());
    lines.push(kv("  Last solve", Line::from(vec![last_title.into(), format!("  {}", fmt::secs(*secs)).bold()])));
    lines.push(Line::default());
    lines.push(Line::from("  Splits vs your PBs from other runs".fg(muted())));
    let scopes = g.scopes_within(&over.scope);
    let mut shown = 0;
    for scope in &scopes {
        let Some(time) = g.scope_time(over.run, scope) else { continue };
        let pb = g.db.pb(&g.scope_ids(scope), Some(over.run));
        let split = Split { scope: scope.clone(), secs: time, pb };
        let mut line = split_line(&g.ws.scope_title(scope), &split);
        line.spans.insert(0, "  ".into());
        lines.push(line);
        shown += 1;
    }
    if shown == 0 {
        lines.push(Line::from("  (none)".fg(muted())));
    }
    lines.push(Line::default());
    lines.push(Line::from(vec![
        "  Enter".fg(accent()).bold(),
        " close (back to completionist mode); every run is in ".fg(muted()),
        "s".fg(accent()).bold(),
        " Stats".fg(muted()),
    ]));
    let height = u16::try_from(lines.len() + 2).unwrap_or(u16::MAX);
    let area = theme::popup(frame, 72, height);
    let block = Block::bordered()
        .title(format!(" ★ Run {} complete ", over.run).fg(star()).bold())
        .border_style(Style::new().fg(star()));
    frame.render_widget(Paragraph::new(lines).block(block), area);
}

/// Shown when every test hits `todo!()`; `file` is the working file.
pub fn not_started_text(in_run: bool, file: &str) -> Text<'static> {
    let mut lines = vec![
        Line::default(),
        Line::from("  Not started yet.".fg(warn()).bold()),
        Line::default(),
        Line::from(format!("  Open this file in your editor, replace each {SENTINEL}, and save:")),
        Line::from(vec!["    ".into(), file.to_string().fg(accent())]),
        Line::from("  The tests run again every time you save."),
        Line::default(),
    ];
    lines.push(if in_run {
        Line::from(vec![
            "  Read the problem with ".fg(muted()),
            "i".fg(accent()).bold(),
            "; stuck? ".fg(muted()),
            "n".fg(accent()).bold(),
            " skips it for now.".fg(muted()),
        ])
    } else {
        Line::from(vec![
            "  Read the problem with ".fg(muted()),
            "i".fg(accent()).bold(),
            "; stuck? ".fg(muted()),
            "h".fg(accent()).bold(),
            " for hints.".fg(muted()),
        ])
    });
    lines.into()
}
