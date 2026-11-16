//! The Stats tab: a tree on the left (Overview, Runs, then sections and
//! their problems), details for whatever is selected on the right. A solved
//! problem whose best is off the reference solution's pace is marked `▲`.

use chrono::{DateTime, Local};
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Style, Stylize};
use ratatui::symbols::Marker;
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{
    Axis, Block, Chart, Dataset, Gauge, GraphType, Padding, Paragraph, Row, Table, TableState, Wrap,
};
use tui_tree_widget::{Tree, TreeItem};

use super::theme::{self, accent, bad, dim, good, kv, muted, star, warn};
use super::{App, StatsPane};
use crate::core::{Gauntlet, Status};
use crate::db::{ALL, RunStatus, Solve};
use crate::fmt;
use crate::perf::{self, Nudge, Perf};
use crate::store::Rating;

/// Tree identifier of the overview node.
pub const OVERVIEW: &str = "★";
/// Tree identifier of the runs node.
pub const RUNS: &str = "⏱";

/// Prefix of section node ids. Problem leaves use the problem id, which
/// can't collide with these prefixed ones.
const SECTION: &str = "s:";

/// Whether a tree node id is a section's.
pub fn is_section(id: &str) -> bool {
    id.starts_with(SECTION)
}

/// The tree's width in `area`: a third and a bit, within reason.
pub fn tree_width(area: Rect) -> u16 {
    (area.width * 3 / 8).clamp(30, 48)
}

/// Columns the tree draws before a label at `depth`: indent and fold symbol.
fn indent(depth: usize) -> usize {
    2 * depth + 2
}

/// A section node, `width` wide: its title, cut short if need be, and its
/// progress flush right. Its problems are the leaves.
fn section_item(g: &Gauntlet, id: String, title: &str, idxs: &[usize], width: usize) -> TreeItem<'static, String> {
    let solved = idxs.iter().filter(|&&i| g.status(i) == Status::Solved).count();
    let count = fmt::count(solved, idxs.len());
    let width = width.saturating_sub(indent(0));
    let mut label = theme::fit(Line::from(title.to_string()), width.saturating_sub(count.len() + 2));
    label.spans.push(" ".repeat(width.saturating_sub(label.width() + count.len())).into());
    label.spans.push(count.fg(if solved == idxs.len() { good() } else { muted() }));
    let children = idxs.iter().map(|&i| leaf(g, i, width)).collect();
    TreeItem::new(id, label, children).expect("tree ids are unique among siblings")
}

/// A problem's status, or `▲` if it's solved and your best is off the
/// reference's pace.
fn icon(g: &Gauntlet, i: usize) -> Span<'static> {
    match g.db.nudge(&g.problem(i).id) {
        Some(_) => "▲".fg(warn()),
        None => theme::status_icon(g.status(i)),
    }
}

fn leaf(g: &Gauntlet, i: usize, width: usize) -> TreeItem<'static, String> {
    let p = g.problem(i);
    let label = Line::from(vec![icon(g, i), " ".into(), p.title.clone().into()]);
    TreeItem::new_leaf(p.id.clone(), theme::fit(label, width.saturating_sub(indent(1))))
}

/// Tree path (ids from the root) of a problem's leaf under its section.
pub fn path_to(g: &Gauntlet, idx: usize) -> Vec<String> {
    let p = g.problem(idx);
    vec![format!("{SECTION}{}", g.ws.section_of(p).slug), p.id.clone()]
}

/// The tree's nodes, labels fitted to `width` (inside the border).
fn tree_items(g: &Gauntlet, width: usize) -> Vec<TreeItem<'static, String>> {
    let ws = &g.ws;
    let mut items = vec![
        TreeItem::new_leaf(OVERVIEW.to_string(), Line::from("★ Overview".bold())),
        TreeItem::new_leaf(RUNS.to_string(), Line::from("⏱ Runs & PBs".bold())),
    ];
    for (s, section) in ws.sections.iter().enumerate() {
        let idxs: Vec<usize> = g.view.iter().copied().filter(|&i| ws.problems[i].section == s).collect();
        if !idxs.is_empty() {
            items.push(section_item(g, format!("{SECTION}{}", section.slug), &section.title, &idxs, width));
        }
    }
    items
}

/// The table in the details pane, which takes focus with Tab.
struct Details<'a> {
    state: &'a mut TableState,
    focused: bool,
    /// Filled in with the problem behind each row, when rows are problems.
    rows: &'a mut Vec<usize>,
}

impl Details<'_> {
    fn render(&mut self, frame: &mut Frame, table: Table<'_>, title: &str, area: Rect) {
        let table = table
            .block(Block::bordered().title(format!(" {title} ").bold()).border_style(theme::pane_border(self.focused)))
            .row_highlight_style(if self.focused { theme::selected(true) } else { Style::new() });
        frame.render_stateful_widget(table, area, self.state);
    }
}

pub fn draw(app: &mut App, frame: &mut Frame, area: Rect) {
    let [left, right] = Layout::horizontal([Constraint::Length(tree_width(area)), Constraint::Fill(1)]).areas(area);
    let items = tree_items(app.g, usize::from(left.width.saturating_sub(2)));
    let tree_focused = app.stats_focus == StatsPane::Tree;
    let tree = Tree::new(&items)
        .expect("identifiers are unique")
        .block(Block::bordered().title(" Problems ".bold()).border_style(theme::pane_border(tree_focused)))
        .highlight_style(theme::selected(tree_focused))
        .highlight_symbol("");
    frame.render_stateful_widget(tree, left, &mut app.tree);

    // A new selection in the tree gets fresh details tables.
    let selected = app.tree.selected().to_vec();
    if selected != app.detail_for {
        app.detail_table = TableState::default();
        app.review_table = TableState::default();
        app.detail_for.clone_from(&selected);
    }
    app.detail_rows.clear();
    app.review_rows.clear();
    let focus = app.stats_focus;
    let mut details =
        Details { state: &mut app.detail_table, focused: focus == StatsPane::Details, rows: &mut app.detail_rows };
    let mut review =
        Details { state: &mut app.review_table, focused: focus == StatsPane::Review, rows: &mut app.review_rows };
    let g = &*app.g;
    match selected.last().map(String::as_str) {
        None | Some(OVERVIEW) => overview(g, frame, right, &mut review, &mut details),
        Some(RUNS) => runs(g, frame, right, &mut details),
        Some(id) => match (g.idx_of(id), id.strip_prefix(SECTION)) {
            (Some(idx), _) => problem(g, idx, frame, right, &mut details),
            (None, Some(scope)) => section(g, scope, app.least_trivial_first, frame, right, &mut details),
            (None, None) => overview(g, frame, right, &mut review, &mut details),
        },
    }
    // The review table went away (nothing left to review, or another page).
    if focus == StatsPane::Review && app.review_rows.is_empty() {
        app.stats_focus = StatsPane::Details;
    }
}

fn panel(title: &str) -> Block<'static> {
    Block::bordered().title(format!(" {title} ").bold()).border_style(theme::pane_border(false))
}

fn opt_secs(s: Option<f64>) -> Span<'static> {
    s.map_or_else(|| "—".fg(muted()), |s| fmt::secs(s).bold())
}

// ───────────────────────────────────────────────────────────── overview ──

/// Rows a table needs to show anything: borders, header, two rows.
const TABLE_MIN: u16 = 5;
/// The ratings panel in full: a bar per rating, then a total.
const RATINGS_FULL: u16 = 9;

fn overview(g: &Gauntlet, frame: &mut Frame, area: Rect, review: &mut Details, details: &mut Details) {
    let up: Vec<usize> = g.review_order(&(0..g.ws.problems.len()).collect::<Vec<_>>());
    // The two tables get room first; the ratings shrink to one line when short.
    let ratings_h = if area.height >= 3 + RATINGS_FULL + 2 * (TABLE_MIN + 1) { RATINGS_FULL } else { 3 };
    let tables_h = area.height.saturating_sub(3 + ratings_h);
    let review_h = if up.is_empty() {
        u16::try_from(no_review().line_count(area.width.saturating_sub(4))).unwrap_or(1).saturating_add(2)
    } else {
        let rows = u16::try_from(up.len()).unwrap_or(u16::MAX).saturating_add(3);
        rows.clamp(TABLE_MIN, (tables_h / 2).max(TABLE_MIN))
    };
    let [gauge, ratings, suggested, recent] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(ratings_h),
        Constraint::Length(review_h),
        Constraint::Min(TABLE_MIN),
    ])
    .areas(area);

    let (solved, total) = g.progress(ALL);
    let streak = g.streak();
    let title = format!("Everything · {streak}-day streak");
    frame.render_widget(
        Gauge::default()
            .block(panel(&title))
            .gauge_style(Style::new().fg(good()).bg(theme::surface()))
            .ratio(solved as f64 / total.max(1) as f64)
            .label(format!("{} solved", fmt::count(solved, total))),
        gauge,
    );

    if ratings_h == RATINGS_FULL {
        draw_ratings(g, frame, ratings);
    } else {
        draw_ratings_line(g, frame, ratings);
    }
    draw_review(g, &up, frame, suggested, review);

    // Recent solves, newest first.
    let mut solves: Vec<(usize, &Solve)> =
        (0..g.ws.problems.len()).flat_map(|i| g.db.history(&g.problem(i).id).iter().map(move |s| (i, s))).collect();
    solves.sort_by_key(|(_, s)| std::cmp::Reverse(s.at));
    solves.truncate(100);
    details.rows.extend(solves.iter().map(|(i, _)| *i));
    // Narrow: no level; narrower still: no date either.
    let width = recent.width.saturating_sub(2);
    let (when, level) = (width >= 70, width >= 50);
    let rows: Vec<Row> = solves
        .iter()
        .map(|&(i, s)| {
            let p = g.problem(i);
            let mut cells = vec![Line::from(p.title.clone()), solve_result(g, s)];
            if level {
                cells.insert(1, Line::from(theme::difficulty(p.difficulty)));
            }
            if when {
                cells.insert(0, Line::from(fmt::local(s.at)));
            }
            Row::new(cells)
        })
        .collect();
    let mut widths = vec![Constraint::Fill(1), Constraint::Length(if level { 28 } else { 19 })];
    let mut header = vec!["Problem", "Result"];
    if level {
        widths.insert(1, Constraint::Length(7));
        header.insert(1, "Level");
    }
    if when {
        widths.insert(0, Constraint::Length(17));
        header.insert(0, "When");
    }
    let table = Table::new(rows, widths).header(Row::new(header).style(dim()));
    let title = if solves.is_empty() { "Recent solves: none yet" } else { "Recent solves" };
    details.render(frame, table, title, recent);
}

/// The problems up for review (`up`, in review order), for `gauntlet review`.
fn draw_review(g: &Gauntlet, up: &[usize], frame: &mut Frame, area: Rect, review: &mut Details) {
    if up.is_empty() {
        frame.render_widget(no_review().block(panel("Suggested review").padding(Padding::horizontal(1))), area);
        return;
    }
    review.rows.extend(up);
    // Narrow: no section, and a shorter date.
    let wide = area.width.saturating_sub(2) >= 70;
    let date = if wide { "%Y-%m-%d" } else { "%b %-d" };
    let rows: Vec<Row> = up
        .iter()
        .map(|&i| {
            let p = g.problem(i);
            let why = match (g.db.review_rating(&p.id), g.db.nudge(&p.id)) {
                (Some(r), _) => r.label().fg(theme::rating_color(Some(r))),
                (None, Some(n)) if n.slower && n.hungrier => "▲ faster, leaner".fg(warn()),
                (None, Some(n)) if n.slower => "▲ could be faster".fg(warn()),
                _ => "▲ could be leaner".fg(warn()),
            };
            let last = g.db.history(&p.id).last().map(|s| s.at.with_timezone(&Local).format(date).to_string());
            let mut cells = vec![Line::from(p.title.clone()), Line::from(why), Line::from(last.unwrap_or_default())];
            if wide {
                cells.insert(1, Line::from(g.ws.section_of(p).title.clone().fg(muted())));
            }
            Row::new(cells)
        })
        .collect();
    let (widths, header) = if wide {
        (
            vec![Constraint::Fill(3), Constraint::Fill(2), Constraint::Length(17), Constraint::Length(11)],
            vec!["Problem", "Section", "Why", "Last solved"],
        )
    } else {
        let widths = vec![Constraint::Fill(1), Constraint::Length(17), Constraint::Length(6)];
        (widths, vec!["Problem", "Why", "Solved"])
    };
    let table = Table::new(rows, widths).header(Row::new(header).style(dim()));
    let title = format!("Suggested review ({}): gauntlet review", up.len());
    review.render(frame, table, &title, area);
}

fn no_review() -> Paragraph<'static> {
    let text = "Nothing rated 1–2 or off the reference's pace. Rate honestly and the hard ones show up here.";
    Paragraph::new(text.fg(muted())).wrap(Wrap { trim: true })
}

/// How many problems have each latest rating, as bars.
fn draw_ratings(g: &Gauntlet, frame: &mut Frame, area: Rect) {
    let dist = g.rating_distribution(ALL);
    let block = panel("Trivial? (latest rating)");
    let inner = block.inner(area);
    frame.render_widget(block, area);
    let max = dist.iter().copied().max().unwrap_or(0).max(1);
    let bar_room = usize::from(inner.width.saturating_sub(24));
    let mut lines = Vec::new();
    for r in Rating::ALL.iter().rev() {
        let n = dist[usize::from(r.value() - 1)];
        let len = (n * bar_room).div_ceil(max);
        lines.push(Line::from(vec![
            format!(" {} {:<18}", r.key(), r.label()).fg(theme::subtext()),
            "█".repeat(len).fg(theme::rating_color(Some(*r))),
            format!(" {n}").bold(),
        ]));
    }
    let rated: usize = dist.iter().sum();
    lines.push(Line::default());
    lines.push(Line::from(format!(" {rated} problem(s) rated").fg(muted())));
    frame.render_widget(Paragraph::new(lines), inner);
}

/// The ratings in one line, when there's no room for the bars.
fn draw_ratings_line(g: &Gauntlet, frame: &mut Frame, area: Rect) {
    let mut spans = vec![Span::raw(" ")];
    spans.extend(rating_counts(g.rating_distribution(ALL)));
    frame.render_widget(Paragraph::new(Line::from(spans)).block(panel("Trivial? (latest rating)")), area);
}

/// `1:0  2:3  3:1  4:0  5:2`, each key in its rating's colour.
fn rating_counts(dist: [usize; 5]) -> Vec<Span<'static>> {
    dist.iter()
        .enumerate()
        .flat_map(|(k, n)| {
            let r = Rating::ALL[k];
            [format!("{}", r.key()).fg(theme::rating_color(Some(r))), format!(":{n}  ").into()]
        })
        .collect()
}

/// A solve's result: its rating (completionist) or time (run).
fn solve_result(g: &Gauntlet, s: &Solve) -> Line<'static> {
    match (s.run, s.secs) {
        (Some(run), secs) => {
            let mut spans = vec![fmt::secs(secs.unwrap_or(0.0)).bold(), format!(" run {run}").fg(muted())];
            if !g.db.counts_for_runs(s) {
                spans.push(" (deleted run)".fg(bad()));
            }
            Line::from(spans)
        }
        (None, _) => match s.rating {
            Some(r) => Line::from(vec![format!("{} ", r.value()).bold(), r.label().fg(theme::rating_color(Some(r)))]),
            None => Line::from("unrated".fg(muted())),
        },
    }
}

// ────────────────────────────────────────────────────────────── section ──

/// A section: progress, its PB and a table of its problems, in content
/// order or least trivial first.
fn section(g: &Gauntlet, scope: &str, least_trivial_first: bool, frame: &mut Frame, area: Rect, details: &mut Details) {
    let mut idxs: Vec<usize> = g.ws.scope_problems(scope);
    let solved = idxs.iter().filter(|&&i| g.status(i) == Status::Solved).count();
    // Narrow: the summary in one column, and the table without level and solves.
    let wide = area.width.saturating_sub(2) >= 74;
    let [gauge, summary, list] =
        Layout::vertical([Constraint::Length(3), Constraint::Length(if wide { 5 } else { 7 }), Constraint::Fill(1)])
            .areas(area);

    frame.render_widget(
        Gauge::default()
            .block(panel(&g.ws.scope_title(scope)))
            .gauge_style(Style::new().fg(good()).bg(theme::surface()))
            .ratio(solved as f64 / idxs.len().max(1) as f64)
            .label(format!("{} solved", fmt::count(solved, idxs.len()))),
        gauge,
    );

    let sob = g.sum_of_best(scope);
    let dist = g.rating_distribution(scope);
    let mut left = vec![
        kv("PB", pb_line(g, scope)),
        kv("Sum of best", sum_of_best_line(sob)),
        kv("In progress", idxs.iter().filter(|&&i| g.status(i) == Status::InProgress).count().to_string()),
    ];
    let right = vec![
        kv("Ratings", Line::from(rating_counts(dist))),
        kv("Solves", idxs.iter().map(|&i| g.db.history(&g.problem(i).id).len()).sum::<usize>().to_string()),
    ];
    let block = panel("Summary");
    let inner = block.inner(summary);
    frame.render_widget(block, summary);
    if wide {
        let [l, r] = Layout::horizontal([Constraint::Percentage(50); 2]).areas(inner);
        frame.render_widget(Paragraph::new(left), l);
        frame.render_widget(Paragraph::new(right), r);
    } else {
        left.extend(right);
        frame.render_widget(Paragraph::new(left), inner);
    }

    if least_trivial_first {
        // Rated, lowest first; then solved but unrated; then unsolved.
        idxs.sort_by_key(|&i| {
            let rating = g.db.latest_rating(&g.problem(i).id);
            (g.status(i) != Status::Solved, rating.is_none(), rating)
        });
    }
    details.rows.extend(&idxs);
    // What's left of the width for titles: the other columns and the gaps between.
    let title_width = usize::from(list.width.saturating_sub(2)).saturating_sub(if wide { 44 } else { 29 });
    let rows: Vec<Row> = idxs
        .iter()
        .map(|&i| {
            let p = g.problem(i);
            let rating = g.db.latest_rating(&p.id);
            let mut cells = vec![
                Line::from(icon(g, i)),
                theme::fit(Line::from(p.title.clone()), title_width),
                Line::from(rating.map_or_else(|| "—".fg(muted()), |r| r.label().fg(theme::rating_color(Some(r))))),
                Line::from(opt_secs(g.db.problem_best(&p.id, None).map(|pb| pb.secs))),
            ];
            if wide {
                cells.insert(2, Line::from(theme::difficulty(p.difficulty)));
                cells.insert(4, Line::from(g.db.history(&p.id).len().to_string()));
            }
            Row::new(cells)
        })
        .collect();
    let mut widths = vec![Constraint::Length(1), Constraint::Fill(1), Constraint::Length(17), Constraint::Length(8)];
    let mut header = vec!["", "Problem", "Trivial?", "Run best"];
    if wide {
        widths.insert(2, Constraint::Length(7));
        header.insert(2, "Level");
        widths.insert(4, Constraint::Length(6));
        header.insert(4, "Solves");
    }
    let table = Table::new(rows, widths).header(Row::new(header).style(dim()));
    let title = if least_trivial_first { "Problems · least trivial first" } else { "Problems · content order" };
    details.render(frame, table, title, list);
}

/// `12:03 run 4`, or `—`.
fn pb_line(g: &Gauntlet, scope: &str) -> Line<'static> {
    match g.scope_pb(scope) {
        Some(pb) => Line::from(vec![fmt::secs(pb.secs).fg(star()).bold(), format!(" run {}", pb.run).fg(muted())]),
        None => Line::from("—".fg(muted())),
    }
}

fn sum_of_best_line(sob: crate::db::SumOfBest) -> Line<'static> {
    let mut spans = vec![fmt::secs(sob.secs).bold()];
    if sob.missing > 0 {
        spans.push(format!(" ({} untimed)", sob.missing).fg(muted()));
    }
    Line::from(spans)
}

// ────────────────────────────────────────────────────────────── problem ──

fn problem(g: &Gauntlet, idx: usize, frame: &mut Frame, area: Rect, details: &mut Details) {
    let p = g.problem(idx);
    let history = g.db.history(&p.id);
    let [header, middle, list] =
        Layout::vertical([Constraint::Length(3), Constraint::Length(12), Constraint::Fill(1)]).areas(area);

    let title = Line::from(vec![
        format!(" {} ", p.title).bold(),
        theme::difficulty(p.difficulty),
        format!(" · {} ", g.ws.location(idx)).fg(muted()),
    ]);
    let badge = Line::from(vec![theme::status_badge(g.status(idx)), " ".into()]);
    let title = theme::fit(title, usize::from(area.width.saturating_sub(3)).saturating_sub(badge.width()));
    let mut block =
        Block::bordered().title(title).title_top(badge.right_aligned()).border_style(theme::pane_border(false));
    let reference = g.db.reference(&p.id);
    let best = g.db.best(&p.id);
    if let Some(nudge) = g.db.nudge(&p.id) {
        block = block.title_bottom(Line::from(format!(" ▲ {} ", nudge.says()).fg(warn()).bold()).right_aligned());
    }
    let hint = Line::from(vec![" Enter".fg(accent()), " to open it in the arena".fg(muted())]);
    frame.render_widget(Paragraph::new(hint).block(block), header);

    // The chart only where it has room to say something; the solves below have the times too.
    let summary_width = if middle.width >= 38 + 30 { 38 } else { middle.width };
    let [summary, times] = Layout::horizontal([Constraint::Length(summary_width), Constraint::Fill(1)]).areas(middle);
    let completionist = history.iter().filter(|s| !s.is_run()).count();
    let in_runs = history.iter().filter(|s| g.db.counts_for_runs(s)).count();
    let latest = g.db.latest_rating(&p.id);
    let mut lines = vec![
        kv("Solves", Line::from(vec![history.len().to_string().bold()])),
        kv("Completionist", completionist.to_string()),
        kv("In runs", in_runs.to_string()),
        kv("Trivial?", latest.map_or_else(|| "—".fg(muted()), |r| r.label().fg(theme::rating_color(Some(r))))),
        kv("Run best", pb_of(g.db.problem_best(&p.id, None))),
        kv("First solved", history.first().map_or_else(|| "never".fg(muted()), |s| fmt::local(s.at).into())),
        kv("Last solved", history.last().map_or_else(|| "never".fg(muted()), |s| fmt::local(s.at).into())),
    ];
    let nudge = reference.and_then(|r| Nudge::of(best, r)).unwrap_or_default();
    let theirs = |r: fn(Perf) -> Option<u64>| reference.and_then(r);
    lines.extend(versus("Best time", best.ms, theirs(|r| r.ms), fmt::runtime, nudge.slower));
    lines.extend(versus("Best memory", best.bytes, theirs(|r| r.bytes), fmt::memory, nudge.hungrier));
    if let Some(r) = reference {
        let parts: Vec<String> = [r.ms.map(fmt::runtime), r.bytes.map(fmt::memory)].into_iter().flatten().collect();
        lines.push(kv("Reference", parts.join(" · ")));
    }
    frame.render_widget(Paragraph::new(lines).block(panel("Summary")), summary);

    let timed: Vec<(f64, f64)> = history
        .iter()
        .filter(|s| g.db.counts_for_runs(s))
        .filter_map(|s| Some((s.at.timestamp() as f64, s.secs?)))
        .collect();
    let top = timed.iter().map(|&(_, y)| y).fold(0.0, f64::max).max(1.0) * 1.1;
    let top_label = fmt::secs(top);
    if !times.is_empty() {
        run_times(frame, times, &timed, top, &top_label);
    }

    // Every solve, newest first. Narrow: no mode, which the result shows
    // anyway, and no memory: the tests' runtime is marked if either is off.
    let wide = list.width.saturating_sub(2) >= 64;
    let rows: Vec<Row> = history
        .iter()
        .enumerate()
        .rev()
        .map(|(n, s)| {
            let off = reference.and_then(|r| Nudge::of(s.perf, r)).is_some();
            let mut tests = Line::from(s.perf.ms.map(fmt::runtime).unwrap_or_default());
            if off {
                tests.spans.push(" ▲".fg(warn()));
            }
            let mut cells =
                vec![Line::from(format!("#{}", n + 1)), Line::from(fmt::local(s.at)), solve_result(g, s), tests];
            if wide {
                cells.insert(2, Line::from(if s.is_run() { "run".fg(star()) } else { "completionist".into() }));
                cells.push(Line::from(s.perf.bytes.map(fmt::memory).unwrap_or_default()));
            }
            Row::new(cells)
        })
        .collect();
    let mut widths = vec![Constraint::Length(4), Constraint::Length(17), Constraint::Fill(1), Constraint::Length(8)];
    let mut header = vec!["#", "When", "Rating / time", "Tests"];
    if wide {
        widths.insert(2, Constraint::Length(14));
        header.insert(2, "Mode");
        widths.push(Constraint::Length(6));
        header.push("Memory");
    }
    let table = Table::new(rows, widths).header(Row::new(header).style(dim()));
    let title = if history.is_empty() { "Solves: none yet" } else { "Solves" };
    details.render(frame, table, title, list);
}

/// `Best time    842 ms · 10× the reference ▲`: your best against the
/// reference's, once it's known, marked if off its pace. Nothing if you have
/// no measured solve.
fn versus(
    label: &str,
    mine: Option<u64>,
    theirs: Option<u64>,
    show: fn(u64) -> String,
    off: bool,
) -> Option<Line<'static>> {
    let mut spans = vec![show(mine?).bold()];
    if let Some(ratio) = perf::ratio(mine, theirs) {
        spans.push(format!(" · {} the ref", fmt::ratio(ratio).trim_start()).fg(if off { warn() } else { muted() }));
    }
    if off {
        spans.push(" ▲".fg(warn()));
    }
    Some(kv(label, Line::from(spans)))
}

fn pb_of(pb: Option<crate::db::Pb>) -> Line<'static> {
    match pb {
        Some(pb) => Line::from(vec![fmt::secs(pb.secs).bold(), format!(" run {}", pb.run).fg(muted())]),
        None => Line::from("—".fg(muted())),
    }
}

/// A line chart of run times (x: unix seconds) with dates on the x axis.
fn run_times(frame: &mut Frame, area: Rect, points: &[(f64, f64)], top: f64, top_label: &str) {
    let title = "Run times";
    if points.is_empty() {
        let text = Text::from(vec![Line::default(), Line::from("  Nothing to plot yet.".fg(muted()))]);
        frame.render_widget(Paragraph::new(text).block(panel(title)), area);
        return;
    }
    let (x0, x1) = points.iter().fold((f64::MAX, f64::MIN), |(a, b), &(x, _)| (a.min(x), b.max(x)));
    // One point (or all at once): give it a day either side.
    let (x0, x1) = if x1 - x0 < 1.0 { (x0 - 86_400.0, x1 + 86_400.0) } else { (x0, x1) };
    let day = |x: f64| {
        DateTime::from_timestamp(x as i64, 0)
            .map_or_else(String::new, |d| d.with_timezone(&Local).format("%b %-d").to_string())
    };
    let datasets = vec![
        Dataset::default()
            .graph_type(GraphType::Line)
            .marker(Marker::Braille)
            .style(Style::new().fg(theme::border()))
            .data(points),
        Dataset::default()
            .graph_type(GraphType::Scatter)
            .marker(Marker::Dot)
            .style(Style::new().fg(accent()))
            .data(points),
    ];
    let chart = Chart::new(datasets)
        .block(panel(title))
        .x_axis(Axis::default().bounds([x0, x1]).labels([day(x0), day(x1)]).style(dim()))
        .y_axis(Axis::default().bounds([0.0, top]).labels(["0:00", top_label]).style(dim()));
    frame.render_widget(chart, area);
}

// ───────────────────────────────────────────────────────────────── runs ──

fn runs(g: &Gauntlet, frame: &mut Frame, area: Rect, details: &mut Details) {
    let all: Vec<&crate::db::Run> = g.db.runs().rev().collect();
    let live = g.run.as_ref();
    let live_height = if live.is_some() { 6 } else { 0 };
    let list_height = u16::try_from(all.len()).unwrap_or(u16::MAX).saturating_add(3).clamp(4, 12);
    let [current, list, pbs] =
        Layout::vertical([Constraint::Length(live_height), Constraint::Length(list_height), Constraint::Fill(1)])
            .areas(area);

    if let Some(state) = live {
        let (done, total) = g.run_progress().unwrap_or_default();
        let current_title = g.idx_of(&state.current).map_or(state.current.clone(), |i| g.problem(i).title.clone());
        let mut splits: Vec<Span> = Vec::new();
        for scope in g.scopes_within(&state.scope) {
            if let Some(t) = g.scope_time(state.run, &scope) {
                let pb = g.db.pb(&g.scope_ids(&scope), Some(state.run));
                splits.push(format!("{} {}", g.ws.scope_title(&scope), fmt::secs(t)).into());
                if let Some(pb) = pb {
                    let d = t - pb.secs;
                    splits.push(format!(" {}", fmt::delta(d)).fg(if d < 0.0 { good() } else { bad() }));
                }
                splits.push("   ".into());
            }
        }
        // Clocks are padded or last, so nothing moves as they tick.
        let lines = [
            kv(
                "Progress",
                Line::from(vec![
                    fmt::count(done, total).bold(),
                    " solved · ⏱ ".fg(muted()),
                    fmt::hms(state.total()).bold(),
                ]),
            ),
            kv(
                "Current",
                Line::from(vec![
                    format!("{:>8}", fmt::clock(state.time_on(&state.current))).fg(muted()),
                    "  ".into(),
                    current_title.into(),
                ]),
            ),
            kv("Splits", if splits.is_empty() { Line::from("none yet".fg(muted())) } else { Line::from(splits) }),
            Line::from(if g.in_run() { "" } else { "  gauntlet run  resumes it" }.fg(accent())),
        ];
        let width = usize::from(current.width.saturating_sub(2));
        let state_word = if state.paused { "paused" } else { "running" };
        let title = format!("Run {} · {} · {state_word}", state.run, g.ws.scope_title(&state.scope));
        let lines = Vec::from(lines.map(|l| theme::fit(l, width)));
        frame.render_widget(Paragraph::new(lines).block(panel(&title)), current);
    }

    // Runs, newest first. Narrow: no start time.
    let wide = list.width.saturating_sub(2) >= 64;
    let rows: Vec<Row> = all
        .iter()
        .map(|r| {
            let live_state = live.filter(|s| s.run == r.id);
            let (status, total) = match (r.status(), live_state) {
                (RunStatus::InProgress, Some(s)) => {
                    (format!("in progress {}/{}", r.solved(), r.problems.len()).fg(warn()), fmt::clock(s.total()))
                }
                (RunStatus::InProgress, None) => {
                    (format!("in progress {}/{}", r.solved(), r.problems.len()).fg(warn()), fmt::secs(r.total()))
                }
                (RunStatus::Finished, _) => ("finished".fg(good()), fmt::secs(r.total())),
                (RunStatus::Deleted, _) => ("deleted".fg(bad()), fmt::secs(r.total())),
            };
            let mut cells = vec![
                Line::from(r.id.to_string()),
                Line::from(g.ws.scope_title(&r.scope)),
                Line::from(status),
                Line::from(total.bold()),
            ];
            if wide {
                cells.insert(1, Line::from(fmt::local(r.started)));
            }
            Row::new(cells)
        })
        .collect();
    let shown = usize::from(list_height.saturating_sub(3));
    let title = if all.is_empty() {
        "Runs: none yet. Start one with `gauntlet run [scope]`".to_string()
    } else if all.len() > shown {
        format!("Runs (newest {shown} of {})", all.len())
    } else {
        "Runs".to_string()
    };
    let title = theme::fit(Line::from(title), usize::from(list.width.saturating_sub(4))).to_string();
    let mut widths = vec![Constraint::Length(4), Constraint::Fill(1), Constraint::Length(18), Constraint::Length(8)];
    let mut header = vec!["Run", "Scope", "Status", "Total"];
    if wide {
        widths.insert(1, Constraint::Length(17));
        header.insert(1, "Started");
    }
    let table = Table::new(rows, widths).header(Row::new(header).style(dim())).block(panel(&title));
    frame.render_widget(table, list);

    // PBs per scope with any run time, in content order.
    let scopes: Vec<String> =
        g.ws.scopes()
            .into_iter()
            .filter(|s| in_view(g, s))
            .filter(|s| s == ALL || g.sum_of_best(s).missing < g.scope_ids(s).len())
            .collect();
    let rows: Vec<Row> = scopes
        .iter()
        .map(|scope| {
            let sob = g.sum_of_best(scope);
            let name = match scope.as_str() {
                ALL => g.ws.scope_title(scope),
                // A section: part of `all`, the row above.
                _ => format!("  {}", g.ws.scope_title(scope)),
            };
            let sob = if sob.missing == g.scope_ids(scope).len() {
                Line::from("—".fg(muted()))
            } else if sob.missing > 0 {
                Line::from(vec![fmt::secs(sob.secs).into(), format!(" ({} untimed)", sob.missing).fg(muted())])
            } else {
                Line::from(fmt::secs(sob.secs))
            };
            Row::new(vec![Line::from(name), pb_line(g, scope), sob])
        })
        .collect();
    let table = Table::new(rows, [Constraint::Fill(1), Constraint::Length(13), Constraint::Length(21)])
        .header(Row::new(["Scope", "PB", "Sum of best"]).style(dim()));
    details.render(frame, table, "Personal bests (scopes with run times)", pbs);
}

/// Whether any problem in view is in `scope`.
fn in_view(g: &Gauntlet, scope: &str) -> bool {
    g.view.iter().any(|&i| g.ws.in_scope(g.problem(i), scope))
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc;

    use ratatui::Terminal;
    use ratatui::backend::TestBackend;

    use super::super::{App, Tab};
    use crate::core::Gauntlet;
    use crate::perf::Perf;
    use crate::problem::Workspace;
    use crate::problem::tests::sample;
    use crate::store::Rating;

    /// The Stats tab drawn at `w`×`h`, the right-hand pane only, as lines.
    fn render(app: &mut App, w: u16, h: u16) -> Vec<String> {
        let mut terminal = Terminal::new(TestBackend::new(w, h)).unwrap();
        terminal.draw(|frame| app.draw(frame)).unwrap();
        let buffer = terminal.backend().buffer();
        let left = super::tree_width(ratatui::layout::Rect::new(0, 0, w, h));
        (0..h).map(|y| (left..w).map(|x| buffer[(x, y)].symbol()).collect()).collect()
    }

    #[test]
    fn the_overview_fits_suggested_review_and_recent_solves_at_any_size() {
        let s = sample("stats-overview");
        let mut g = Gauntlet::with_workspace(Workspace::discover_in(s.0.clone()).unwrap()).unwrap();
        for (id, r) in [("option", 1), ("dup", 2), ("two_sum", 5), ("min_stack", 1)] {
            let i = g.idx_of(id).unwrap();
            g.open(i).unwrap();
            g.record_pass(i, Perf::default()).unwrap();
            g.rate(i, Rating::new(r).unwrap()).unwrap();
        }
        let (tx, _rx) = mpsc::channel();
        let mut app = App::new(&mut g, tx, Tab::Stats);
        for (w, h) in [(80, 24), (100, 30), (160, 50)] {
            let lines = render(&mut app, w, h);
            if std::env::var_os("GAUNTLET_SHOW").is_some() {
                println!("{w}x{h}\n{}", lines.join("\n"));
            }
            let at = |text: &str| lines.iter().position(|l| l.contains(text));
            let (review, recent) = (at("Suggested review").unwrap(), at("Recent solves").unwrap());
            assert!(review < recent, "{w}x{h}");
            let count = |rows: &[String], text: &str| rows.iter().filter(|l| l.contains(text)).count();
            // Every problem up for review (Option, Min Stack, Dup) between the two titles.
            for title in ["Option", "Min Stack", "Dup"] {
                assert_eq!(count(&lines[review..recent], title), 1, "{w}x{h}: {title} in review\n{}", lines.join("\n"));
            }
            // Recent solves: at least two rows, and its bottom border on screen.
            let body = &lines[recent + 2..];
            assert!(count(body, "Two Sum") + count(body, "Min Stack") + count(body, "Dup") >= 2, "{w}x{h}");
            assert!(lines[usize::from(h) - 2].starts_with('└'), "{w}x{h}: bottom border\n{}", lines.join("\n"));
        }
        // With room, the ratings get their bars; at 80x24 one line.
        let short = render(&mut app, 80, 24);
        assert!(!short.iter().any(|l| l.contains("Neutral")));
        let tall = render(&mut app, 100, 30);
        assert!(tall.iter().any(|l| l.contains("Neutral")));
    }
}
