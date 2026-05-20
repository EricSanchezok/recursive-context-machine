use std::collections::{BTreeMap, VecDeque};
use std::io::{IsTerminal, Write};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use crossterm::{
    cursor::{Hide, MoveTo, Show, position},
    execute,
    style::{Color, Stylize},
    terminal::{Clear, ClearType},
};

use crate::hook::{
    CompletionEvent, ComponentEvent, ComponentMeta, FragmentEvent, FragmentMeta, GraphEvent,
    HookEvent, HookKind, MachineEvent, ResourceEvent, ToolEvent,
};

const IDLE_TICK: Duration = Duration::from_millis(50);
const CELL_WIDTH: usize = 2;
const VIEW_ROWS: u16 = 18;
const MAX_EXPANDED_TAPES: usize = 3;

#[derive(Clone)]
struct TapeCell {
    id: u64,
    kind: CellKind,
    state: CellState,
}

#[derive(Clone, Copy, PartialEq)]
enum CellState {
    Pending,
    Written,
    Replacing,
    Removing,
}

#[derive(Clone, Copy, PartialEq)]
enum CellKind {
    SystemText,
    UserText,
    AssistantText,
    ToolCall,
    ToolResult,
    Image,
    Audio,
    Video,
    Document,
    Hitch,
    Unknown,
}

impl CellKind {
    fn from_meta(meta: &FragmentMeta) -> Self {
        match meta.kind.as_str() {
            "tool_call" => Self::ToolCall,
            "tool_result" => Self::ToolResult,
            "image" => Self::Image,
            "audio" => Self::Audio,
            "video" => Self::Video,
            "document" => Self::Document,
            "hitch" => Self::Hitch,
            "text" => match meta.role.as_str() {
                "system" => Self::SystemText,
                "user" => Self::UserText,
                "assistant" => Self::AssistantText,
                _ => Self::Unknown,
            },
            _ => Self::Unknown,
        }
    }

    fn color(self) -> Color {
        match self {
            Self::SystemText => Color::DarkBlue,
            Self::UserText => Color::Green,
            Self::AssistantText => Color::White,
            Self::ToolCall => Color::Yellow,
            Self::ToolResult => Color::Cyan,
            Self::Image => Color::Magenta,
            Self::Audio => Color::DarkMagenta,
            Self::Video => Color::DarkYellow,
            Self::Document => Color::DarkGreen,
            Self::Hitch => Color::Red,
            Self::Unknown => Color::DarkGrey,
        }
    }
}

#[derive(Clone)]
enum TapeOp {
    Write {
        index: usize,
        id: u64,
        kind: CellKind,
        label: String,
    },
    Replace {
        index: usize,
        kind: CellKind,
        label: String,
    },
    Remove {
        index: usize,
        id: u64,
        label: String,
    },
    Swap {
        first: usize,
        second: usize,
        label: String,
    },
}

impl TapeOp {
    fn target(&self) -> usize {
        match self {
            Self::Write { index, .. }
            | Self::Replace { index, .. }
            | Self::Remove { index, .. } => *index,
            Self::Swap { first, .. } => *first,
        }
    }

    fn label(&self) -> &str {
        match self {
            Self::Write { label, .. }
            | Self::Replace { label, .. }
            | Self::Remove { label, .. }
            | Self::Swap { label, .. } => label,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum TapeStatus {
    Waiting,
    Running,
    Done,
    Skipped,
}

impl TapeStatus {
    fn glyph(self) -> &'static str {
        match self {
            Self::Waiting => "·",
            Self::Running => "▷",
            Self::Done => "✓",
            Self::Skipped => "⊘",
        }
    }

    fn color(self) -> Color {
        match self {
            Self::Waiting => Color::DarkGrey,
            Self::Running => Color::Cyan,
            Self::Done => Color::Green,
            Self::Skipped => Color::DarkYellow,
        }
    }
}

struct TapeState {
    name: String,
    kind: String,
    cells: Vec<TapeCell>,
    pointer: usize,
    queue: VecDeque<TapeOp>,
    active: Option<TapeOp>,
    status: String,
    lifecycle: TapeStatus,
    frontier: Option<u64>,
    tool_calls: usize,
}

impl TapeState {
    fn new(name: impl Into<String>, kind: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            kind: kind.into(),
            cells: Vec::new(),
            pointer: 0,
            queue: VecDeque::new(),
            active: None,
            status: "waiting".into(),
            lifecycle: TapeStatus::Waiting,
            frontier: None,
            tool_calls: 0,
        }
    }
}

struct ViewState {
    graph: Option<String>,
    frontier: Option<u64>,
    frontier_count: usize,
    tapes: BTreeMap<String, TapeState>,
    summary: Summary,
    origin_y: u16,
}

pub(crate) struct Summary {
    pub(crate) fragments: usize,
    pub(crate) tool_calls: usize,
    pub(crate) duration_s: f64,
}

pub(crate) fn run_animation(
    rx: mpsc::Receiver<HookEvent>,
    delay_ms: u64,
    start: std::time::Instant,
) -> Summary {
    if !std::io::stdout().is_terminal() {
        return run_silent(rx, start);
    }

    let step = Duration::from_millis(delay_ms);
    let origin_y = reserve_animation_rows();
    let mut view = ViewState {
        graph: None,
        frontier: None,
        frontier_count: 0,
        tapes: BTreeMap::new(),
        summary: Summary {
            fragments: 0,
            tool_calls: 0,
            duration_s: 0.0,
        },
        origin_y,
    };

    execute!(std::io::stdout(), Hide).ok();

    loop {
        for event in rx.try_iter() {
            let finished = is_finish_event(&view, &event);
            apply_event(&mut view, event);
            if finished {
                return finish_animation(&mut view, step, start);
            }
        }

        tick(&mut view, step);

        match rx.recv_timeout(IDLE_TICK) {
            Ok(event) => {
                let finished = is_finish_event(&view, &event);
                apply_event(&mut view, event);
                if finished {
                    return finish_animation(&mut view, step, start);
                }
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {}
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                return finish_animation(&mut view, step, start);
            }
        }
    }
}

fn run_silent(rx: mpsc::Receiver<HookEvent>, start: std::time::Instant) -> Summary {
    let mut graph_seen = false;
    let mut summary = Summary {
        fragments: 0,
        tool_calls: 0,
        duration_s: 0.0,
    };

    for event in rx {
        match event.kind {
            HookKind::Graph(GraphEvent::Start { .. }) => graph_seen = true,
            HookKind::Graph(GraphEvent::Done { .. }) => break,
            HookKind::Machine(MachineEvent::Done) if !graph_seen => break,
            HookKind::Fragment(FragmentEvent::Appended(_))
            | HookKind::Fragment(FragmentEvent::Taken(_))
            | HookKind::Fragment(FragmentEvent::Inserted(_)) => summary.fragments += 1,
            HookKind::Fragment(FragmentEvent::Removed { .. }) => {
                summary.fragments = summary.fragments.saturating_sub(1);
            }
            HookKind::Tool(ToolEvent::Result { .. }) => summary.tool_calls += 1,
            _ => {}
        }
    }

    summary.duration_s = start.elapsed().as_secs_f64();
    summary
}

fn reserve_animation_rows() -> u16 {
    let (_, before_y) = position().unwrap_or((0, 0));
    for _ in 0..VIEW_ROWS {
        println!();
    }
    let (_, after_y) = position().unwrap_or((0, before_y.saturating_add(VIEW_ROWS)));
    after_y.saturating_sub(VIEW_ROWS)
}

fn is_finish_event(view: &ViewState, event: &HookEvent) -> bool {
    match &event.kind {
        HookKind::Graph(GraphEvent::Done { .. }) => true,
        HookKind::Machine(MachineEvent::Done) => view.graph.is_none(),
        _ => false,
    }
}

fn finish_animation(view: &mut ViewState, step: Duration, start: std::time::Instant) -> Summary {
    while view
        .tapes
        .values()
        .any(|tape| tape.active.is_some() || !tape.queue.is_empty())
    {
        tick(view, step);
    }
    view.summary.duration_s = start.elapsed().as_secs_f64();
    render_frame(view);
    thread::sleep(Duration::from_millis(250));
    execute!(
        std::io::stdout(),
        Show,
        MoveTo(0, view.origin_y + VIEW_ROWS)
    )
    .ok();
    Summary {
        fragments: view.summary.fragments,
        tool_calls: view.summary.tool_calls,
        duration_s: view.summary.duration_s,
    }
}

fn tick(view: &mut ViewState, step: Duration) {
    for tape in view.tapes.values_mut() {
        advance_tape(tape);
    }
    view.summary.fragments = view.tapes.values().map(|tape| tape.cells.len()).sum();
    render_frame(view);
    thread::sleep(step.max(IDLE_TICK));
}

fn advance_tape(tape: &mut TapeState) {
    if tape.active.is_none() {
        tape.active = tape.queue.pop_front();
    }

    let Some(op) = tape.active.as_ref() else {
        return;
    };

    let target = op.target().min(tape.cells.len().saturating_sub(1));
    tape.status = op.label().to_string();
    match tape.pointer.cmp(&target) {
        std::cmp::Ordering::Less => tape.pointer += 1,
        std::cmp::Ordering::Greater => tape.pointer -= 1,
        std::cmp::Ordering::Equal => advance_active_op(tape),
    }
}

fn advance_active_op(tape: &mut TapeState) {
    if let Some(TapeOp::Replace { index, .. }) = tape.active.as_ref()
        && let Some(cell) = tape.cells.get_mut(*index)
        && cell.state != CellState::Replacing
    {
        cell.state = CellState::Replacing;
        tape.status = "overwriting".into();
        return;
    }

    let Some(op) = tape.active.take() else {
        return;
    };

    match op {
        TapeOp::Write {
            index, id, kind, ..
        } => {
            if let Some(cell) = tape.cells.get_mut(index) {
                cell.id = id;
                cell.kind = kind;
                cell.state = CellState::Written;
            }
        }
        TapeOp::Replace { index, kind, .. } => {
            if let Some(cell) = tape.cells.get_mut(index) {
                cell.kind = kind;
                cell.state = CellState::Written;
            }
        }
        TapeOp::Remove { index, id, .. } => {
            if tape.cells.get(index).is_some_and(|cell| cell.id == id) {
                tape.cells.remove(index);
                if !tape.cells.is_empty() {
                    tape.pointer = tape.pointer.min(tape.cells.len() - 1);
                } else {
                    tape.pointer = 0;
                }
            }
        }
        TapeOp::Swap { first, second, .. } => {
            if first < tape.cells.len() && second < tape.cells.len() {
                tape.cells.swap(first, second);
            }
        }
    }
}

fn apply_event(view: &mut ViewState, event: HookEvent) {
    let source = event.source;
    match event.kind {
        HookKind::Graph(event) => apply_graph_event(view, event),
        HookKind::Component(event) => apply_component_event(view, event),
        HookKind::Fragment(event) => {
            apply_fragment_event(tape_for_source(view, source.as_ref()), event)
        }
        HookKind::Tool(event) => apply_tool_event(view, source.as_ref(), event),
        HookKind::Completion(event) => {
            apply_completion_event(tape_for_source(view, source.as_ref()), event)
        }
        HookKind::Machine(event) => {
            apply_machine_event(tape_for_source(view, source.as_ref()), event)
        }
        HookKind::Resource(event) => {
            apply_resource_event(tape_for_source(view, source.as_ref()), event)
        }
    }
}

fn apply_graph_event(view: &mut ViewState, event: GraphEvent) {
    match event {
        GraphEvent::Start { graph } => view.graph = Some(graph),
        GraphEvent::Done { graph } => {
            view.graph = Some(graph);
            for tape in view.tapes.values_mut() {
                if tape.lifecycle == TapeStatus::Running {
                    tape.lifecycle = TapeStatus::Done;
                }
            }
        }
        GraphEvent::FrontierStart {
            graph,
            frontier,
            count,
        } => {
            view.graph = Some(graph);
            view.frontier = Some(frontier);
            view.frontier_count = count;
        }
        GraphEvent::FrontierDone {
            graph,
            frontier,
            count,
        } => {
            view.graph = Some(graph);
            view.frontier = Some(frontier);
            view.frontier_count = view.frontier_count.saturating_sub(count);
        }
    }
}

fn apply_component_event(view: &mut ViewState, event: ComponentEvent) {
    match event {
        ComponentEvent::Start(meta) => {
            let tape = tape_for_source(view, Some(&meta));
            tape.name = meta.name;
            tape.kind = meta.kind;
            tape.frontier = meta.frontier;
            tape.lifecycle = TapeStatus::Running;
            tape.status = "start".into();
        }
        ComponentEvent::Done(meta) => {
            let tape = tape_for_source(view, Some(&meta));
            tape.name = meta.name;
            tape.kind = meta.kind;
            tape.frontier = meta.frontier;
            tape.lifecycle = TapeStatus::Done;
            tape.status = "done".into();
        }
        ComponentEvent::Skipped(meta) => {
            let tape = tape_for_source(view, Some(&meta));
            tape.name = meta.name;
            tape.kind = meta.kind;
            tape.frontier = meta.frontier;
            tape.lifecycle = TapeStatus::Skipped;
            tape.status = "skipped".into();
        }
    }
}

fn apply_fragment_event(tape: &mut TapeState, event: FragmentEvent) {
    match event {
        FragmentEvent::Appended(meta) | FragmentEvent::Taken(meta) => {
            enqueue_write(tape, meta, "write")
        }
        FragmentEvent::Inserted(meta) => enqueue_write(tape, meta, "insert"),
        FragmentEvent::Replaced(meta) => enqueue_replace(tape, meta),
        FragmentEvent::Removed { id } => enqueue_remove(tape, id),
        FragmentEvent::Swapped { first, second } => enqueue_swap(tape, first, second),
    }
}

fn apply_tool_event(view: &mut ViewState, source: Option<&ComponentMeta>, event: ToolEvent) {
    match event {
        ToolEvent::Call { tool, arguments } => {
            tape_for_source(view, source).status = format!("tool {tool} args={}B", arguments.len());
        }
        ToolEvent::Result {
            tool,
            result_len,
            duration,
        } => {
            view.summary.tool_calls += 1;
            let tape = tape_for_source(view, source);
            tape.tool_calls += 1;
            tape.status = format!("tool {tool} done {result_len}B {duration}");
        }
        ToolEvent::Error {
            tool,
            error,
            retryable,
        } => {
            let retry = if retryable { "retry" } else { "fatal" };
            tape_for_source(view, source).status = format!("tool {tool} error {retry}: {error}");
        }
    }
}

fn apply_completion_event(tape: &mut TapeState, event: CompletionEvent) {
    match event {
        CompletionEvent::Start => tape.status = "completion".into(),
        CompletionEvent::End { fragments } => tape.status = format!("drain {fragments} fragments"),
    }
}

fn apply_machine_event(tape: &mut TapeState, event: MachineEvent) {
    match event {
        MachineEvent::Start => {
            tape.lifecycle = TapeStatus::Running;
            tape.status = "start".into();
        }
        MachineEvent::Halt { step } => tape.status = format!("halt #{step}"),
        MachineEvent::Done => {
            tape.lifecycle = TapeStatus::Done;
            tape.status = "done".into();
        }
    }
}

fn apply_resource_event(tape: &mut TapeState, event: ResourceEvent) {
    match event {
        ResourceEvent::Model { name } => tape.status = format!("model {name}"),
        ResourceEvent::Activate { name } => tape.status = format!("activate {name}"),
        ResourceEvent::Deactivate { name } => tape.status = format!("deactivate {name}"),
    }
}

fn tape_for_source<'a>(
    view: &'a mut ViewState,
    source: Option<&ComponentMeta>,
) -> &'a mut TapeState {
    let key = source.map(component_key).unwrap_or_else(|| "main".into());
    view.tapes.entry(key).or_insert_with(|| match source {
        Some(meta) => TapeState::new(meta.name.clone(), meta.kind.clone()),
        None => TapeState::new("accelerator", "accelerator"),
    })
}

fn component_key(meta: &ComponentMeta) -> String {
    format!("{}:{}:{}", meta.graph, meta.index, meta.name)
}

fn enqueue_write(tape: &mut TapeState, meta: FragmentMeta, action: &str) {
    let index = tape.cells.len();
    let kind = CellKind::from_meta(&meta);
    tape.cells.push(TapeCell {
        id: meta.id,
        kind,
        state: CellState::Pending,
    });
    tape.queue.push_back(TapeOp::Write {
        index,
        id: meta.id,
        kind,
        label: format!(
            "{action} {}/{} #{} {}",
            meta.role, meta.kind, meta.id, meta.preview
        ),
    });
}

fn enqueue_replace(tape: &mut TapeState, meta: FragmentMeta) {
    let kind = CellKind::from_meta(&meta);
    let Some(index) = tape.cells.iter().position(|cell| cell.id == meta.id) else {
        enqueue_write(tape, meta, "write");
        return;
    };
    tape.queue.push_back(TapeOp::Replace {
        index,
        kind,
        label: format!(
            "replace {}/{} #{} {}",
            meta.role, meta.kind, meta.id, meta.preview
        ),
    });
}

fn enqueue_remove(tape: &mut TapeState, id: u64) {
    if let Some(index) = tape.cells.iter().position(|cell| cell.id == id) {
        tape.cells[index].state = CellState::Removing;
        tape.queue.push_back(TapeOp::Remove {
            index,
            id,
            label: format!("remove #{id}"),
        });
    }
}

fn enqueue_swap(tape: &mut TapeState, first: u64, second: u64) {
    let first_index = tape.cells.iter().position(|cell| cell.id == first);
    let second_index = tape.cells.iter().position(|cell| cell.id == second);
    if let (Some(first_index), Some(second_index)) = (first_index, second_index) {
        tape.queue.push_back(TapeOp::Swap {
            first: first_index,
            second: second_index,
            label: format!("swap #{first} ↔ #{second}"),
        });
    }
}

fn render_frame(view: &ViewState) {
    let width = crossterm::terminal::size()
        .map(|(width, _)| width as usize)
        .unwrap_or(80);
    let inner = width.saturating_sub(4).max(20);
    let mut out = std::io::stdout();

    for row in 0..VIEW_ROWS {
        execute!(
            out,
            MoveTo(0, view.origin_y + row),
            Clear(ClearType::CurrentLine)
        )
        .ok();
    }

    draw_border(&mut out, view.origin_y, width, &view_title(view));
    draw_graph_status(&mut out, view.origin_y + 1, inner, view);

    let mut row = view.origin_y + 2;
    for tape in expanded_tapes(view).into_iter().take(MAX_EXPANDED_TAPES) {
        draw_tape_card(&mut out, row, inner, tape);
        row += 3;
    }

    let compact = compact_tapes(view);
    for tape in compact
        .into_iter()
        .take((VIEW_ROWS.saturating_sub(row - view.origin_y + 1)) as usize)
    {
        draw_compact_tape(&mut out, row, inner, tape);
        row += 1;
    }

    draw_footer(&mut out, view.origin_y + VIEW_ROWS - 1, width, view);
    out.flush().ok();
}

fn view_title(view: &ViewState) -> String {
    match &view.graph {
        Some(graph) => format!("Graph · {graph}"),
        None => "Context Tape".into(),
    }
}

fn draw_graph_status(out: &mut std::io::Stdout, row: u16, inner: usize, view: &ViewState) {
    execute!(out, MoveTo(0, row), Clear(ClearType::CurrentLine)).ok();
    let running = view
        .tapes
        .values()
        .filter(|tape| tape.lifecycle == TapeStatus::Running)
        .count();
    let done = view
        .tapes
        .values()
        .filter(|tape| tape.lifecycle == TapeStatus::Done)
        .count();
    let skipped = view
        .tapes
        .values()
        .filter(|tape| tape.lifecycle == TapeStatus::Skipped)
        .count();
    let frontier = view
        .frontier
        .map(|frontier| format!("frontier #{frontier}"))
        .unwrap_or_else(|| "booting".into());
    let line = format!(
        "{frontier} · running {running}/{} · done {done} · skipped {skipped}",
        view.frontier_count
    );
    draw_inside(out, row, inner, &line, Color::DarkCyan);
}

fn draw_tape_card(out: &mut std::io::Stdout, row: u16, inner: usize, tape: &TapeState) {
    let frontier = tape
        .frontier
        .map(|frontier| format!(" · frontier #{frontier}"))
        .unwrap_or_default();
    let title = format!("{} {}{}", tape.name, tape.lifecycle.glyph(), frontier);
    draw_inside(out, row, inner, &title, tape.lifecycle.color());
    draw_tape(out, row + 1, tape, inner);
    let status = format!(
        "⚙ {} · {} · {} cells · {} tools",
        one_line(&tape.status),
        position_text(tape),
        tape.cells.len(),
        tape.tool_calls
    );
    draw_inside(out, row + 2, inner, &status, Color::DarkCyan);
}

fn draw_compact_tape(out: &mut std::io::Stdout, row: u16, inner: usize, tape: &TapeState) {
    let line = format!(
        "{} {} · {} · {} cells · {} tools",
        tape.lifecycle.glyph(),
        tape.name,
        tape.kind,
        tape.cells.len(),
        tape.tool_calls
    );
    draw_inside(out, row, inner, &line, tape.lifecycle.color());
}

fn expanded_tapes(view: &ViewState) -> Vec<&TapeState> {
    let mut tapes = view
        .tapes
        .values()
        .filter(|tape| tape.lifecycle == TapeStatus::Running)
        .collect::<Vec<_>>();
    if tapes.is_empty() {
        tapes = view.tapes.values().take(1).collect();
    }
    tapes
}

fn compact_tapes(view: &ViewState) -> Vec<&TapeState> {
    view.tapes
        .values()
        .filter(|tape| tape.lifecycle != TapeStatus::Running)
        .collect()
}

fn draw_border(out: &mut std::io::Stdout, row: u16, width: usize, title: &str) {
    execute!(out, MoveTo(0, row), Clear(ClearType::CurrentLine)).ok();
    let label = format!(" {title} ");
    let right = width.saturating_sub(label.chars().count() + 2);
    write!(
        out,
        "{}{}{}{}",
        "╭".with(Color::DarkGrey),
        "─".repeat(1).with(Color::DarkGrey),
        label.with(Color::DarkCyan),
        format!("{}╮", "─".repeat(right.saturating_sub(1))).with(Color::DarkGrey),
    )
    .ok();
}

fn draw_footer(out: &mut std::io::Stdout, row: u16, width: usize, view: &ViewState) {
    execute!(out, MoveTo(0, row), Clear(ClearType::CurrentLine)).ok();
    let label = format!(
        " {} cells · {} tools · {:.1}s ",
        view.summary.fragments, view.summary.tool_calls, view.summary.duration_s,
    );
    let right = width.saturating_sub(label.chars().count() + 2);
    write!(
        out,
        "{}{}{}{}",
        "╰".with(Color::DarkGrey),
        "─".repeat(1).with(Color::DarkGrey),
        label.with(Color::DarkCyan),
        format!("{}╯", "─".repeat(right.saturating_sub(1))).with(Color::DarkGrey),
    )
    .ok();
}

fn draw_inside(out: &mut std::io::Stdout, row: u16, inner: usize, text: &str, color: Color) {
    execute!(out, MoveTo(0, row), Clear(ClearType::CurrentLine)).ok();
    let text = trim_to_width(text, inner);
    let used = text.chars().count();
    write!(
        out,
        "{} {}{}{}",
        "│".with(Color::DarkGrey),
        text.with(color),
        " ".repeat(inner.saturating_sub(used)),
        "│".with(Color::DarkGrey),
    )
    .ok();
}

fn draw_tape(out: &mut std::io::Stdout, row: u16, tape: &TapeState, inner: usize) {
    execute!(out, MoveTo(0, row), Clear(ClearType::CurrentLine)).ok();
    write!(out, "{} ", "│".with(Color::DarkGrey)).ok();
    let (start, end) = visible_window(tape, inner);
    if start > 0 {
        write!(out, "{} ", "…".with(Color::DarkGrey)).ok();
    }
    if tape.cells.is_empty() {
        write!(out, "{}", "□".with(Color::DarkGrey)).ok();
    } else {
        for index in start..end {
            let cell = &tape.cells[index];
            let glyph = if index == tape.pointer {
                "◈"
            } else {
                cell_glyph(cell)
            };
            write!(out, "{} ", glyph.with(cell_color(cell))).ok();
        }
    }
    if end < tape.cells.len() {
        write!(out, "{}", "…".with(Color::DarkGrey)).ok();
    }
    write!(out, "{}", "│".with(Color::DarkGrey)).ok();
}

fn visible_window(tape: &TapeState, inner: usize) -> (usize, usize) {
    if tape.cells.is_empty() {
        return (0, 0);
    }
    let max_cells = (inner / CELL_WIDTH).saturating_sub(3).max(1);
    if tape.cells.len() <= max_cells {
        return (0, tape.cells.len());
    }
    let half = max_cells / 2;
    let start = tape
        .pointer
        .saturating_sub(half)
        .min(tape.cells.len().saturating_sub(max_cells));
    (start, start + max_cells)
}

fn position_text(tape: &TapeState) -> String {
    if tape.cells.is_empty() {
        "0/0".into()
    } else {
        format!("{}/{}", tape.pointer + 1, tape.cells.len())
    }
}

fn trim_to_width(text: &str, width: usize) -> String {
    let text = one_line(text);
    if text.chars().count() <= width {
        return text;
    }
    let mut trimmed = text
        .chars()
        .take(width.saturating_sub(1))
        .collect::<String>();
    trimmed.push('…');
    trimmed
}

fn one_line(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn cell_glyph(cell: &TapeCell) -> &'static str {
    match cell.state {
        CellState::Pending | CellState::Removing => "□",
        CellState::Replacing => "◈",
        CellState::Written => "■",
    }
}

fn cell_color(cell: &TapeCell) -> Color {
    match cell.state {
        CellState::Removing => Color::Red,
        CellState::Replacing => Color::Yellow,
        CellState::Pending | CellState::Written => cell.kind.color(),
    }
}
