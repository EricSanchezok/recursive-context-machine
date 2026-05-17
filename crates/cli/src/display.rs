use std::collections::VecDeque;
use std::io::Write;
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
    CompletionEvent, FragmentEvent, FragmentMeta, HookEvent, MachineEvent, ResourceEvent, ToolEvent,
};

const IDLE_TICK: Duration = Duration::from_millis(50);
const CELL_WIDTH: usize = 2;

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

pub(crate) struct State {
    cells: Vec<TapeCell>,
    pointer: usize,
    queue: VecDeque<TapeOp>,
    active: Option<TapeOp>,
    status: String,
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
    let step = Duration::from_millis(delay_ms);
    let origin_y = position().map(|(_, cursor_y)| cursor_y).unwrap_or(0);
    let mut state = State {
        cells: Vec::new(),
        pointer: 0,
        queue: VecDeque::new(),
        active: None,
        status: "booting".into(),
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
            if matches!(event, HookEvent::Machine(MachineEvent::Done)) {
                state.summary.duration_s = start.elapsed().as_secs_f64();
                finish_animation(&mut state, step);
                return state.summary;
            }
            apply_event(&mut state, event);
        }

        tick(&mut state, step);

        match rx.recv_timeout(IDLE_TICK) {
            Ok(HookEvent::Machine(MachineEvent::Done)) => {
                state.summary.duration_s = start.elapsed().as_secs_f64();
                finish_animation(&mut state, step);
                return state.summary;
            }
            Ok(event) => apply_event(&mut state, event),
            Err(mpsc::RecvTimeoutError::Timeout) => {}
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                state.summary.duration_s = start.elapsed().as_secs_f64();
                finish_animation(&mut state, step);
                return state.summary;
            }
        }
    }
}

fn finish_animation(state: &mut State, step: Duration) {
    while state.active.is_some() || !state.queue.is_empty() {
        tick(state, step);
    }
    state.status = "done".into();
    render_frame(state);
    thread::sleep(Duration::from_millis(250));
    execute!(std::io::stdout(), Show, MoveTo(0, state.origin_y + 2)).ok();
}

fn tick(state: &mut State, step: Duration) {
    if state.active.is_none() {
        state.active = state.queue.pop_front();
    }

    if let Some(op) = state.active.as_ref() {
        let target = op.target().min(state.cells.len().saturating_sub(1));
        state.status = op.label().to_string();
        render_frame(state);
        thread::sleep(step);

        match state.pointer.cmp(&target) {
            std::cmp::Ordering::Less => state.pointer += 1,
            std::cmp::Ordering::Greater => state.pointer -= 1,
            std::cmp::Ordering::Equal => commit_active_op(state),
        }
    } else {
        render_frame(state);
        thread::sleep(IDLE_TICK);
    }
}

fn commit_active_op(state: &mut State) {
    let Some(op) = state.active.take() else {
        return;
    };

    match op {
        TapeOp::Write {
            index, id, kind, ..
        } => {
            if let Some(cell) = state.cells.get_mut(index) {
                cell.id = id;
                cell.kind = kind;
                cell.state = CellState::Written;
            }
        }
        TapeOp::Replace { index, kind, .. } => {
            if let Some(cell) = state.cells.get_mut(index) {
                cell.kind = kind;
                cell.state = CellState::Written;
            }
        }
        TapeOp::Remove { index, id, .. } => {
            if state.cells.get(index).is_some_and(|cell| cell.id == id) {
                state.cells.remove(index);
                if !state.cells.is_empty() {
                    state.pointer = state.pointer.min(state.cells.len() - 1);
                } else {
                    state.pointer = 0;
                }
            }
        }
        TapeOp::Swap { first, second, .. } => {
            if first < state.cells.len() && second < state.cells.len() {
                state.cells.swap(first, second);
            }
        }
    }

    state.summary.fragments = state.cells.len();
}

fn apply_event(state: &mut State, event: HookEvent) {
    match event {
        HookEvent::Fragment(FragmentEvent::Appended(meta))
        | HookEvent::Fragment(FragmentEvent::Taken(meta)) => enqueue_write(state, meta, "write"),
        HookEvent::Fragment(FragmentEvent::Inserted(meta)) => enqueue_write(state, meta, "insert"),
        HookEvent::Fragment(FragmentEvent::Replaced(meta)) => enqueue_replace(state, meta),
        HookEvent::Fragment(FragmentEvent::Removed { id }) => enqueue_remove(state, id),
        HookEvent::Fragment(FragmentEvent::Swapped { first, second }) => {
            enqueue_swap(state, first, second)
        }
        HookEvent::Tool(ToolEvent::Call { tool, arguments }) => {
            state.status = format!("tool {tool} args={}B", arguments.len());
        }
        HookEvent::Tool(ToolEvent::Result {
            tool,
            result_len,
            duration,
        }) => {
            state.summary.tool_calls += 1;
            state.status = format!("tool {tool} done {result_len}B {duration}");
        }
        HookEvent::Tool(ToolEvent::Error {
            tool,
            error,
            retryable,
        }) => {
            let retry = if retryable { "retry" } else { "fatal" };
            state.status = format!("tool {tool} error {retry}: {error}");
        }
        HookEvent::Completion(CompletionEvent::Start) => {
            state.status = "completion".into();
        }
        HookEvent::Completion(CompletionEvent::End { fragments }) => {
            state.status = format!("drain {fragments} fragments");
        }
        HookEvent::Machine(MachineEvent::Halt { round }) => {
            state.status = format!("halt #{round}");
        }
        HookEvent::Machine(MachineEvent::Start) => {
            state.status = "start".into();
        }
        HookEvent::Resource(ResourceEvent::Model { name }) => {
            state.status = format!("model {name}");
        }
        HookEvent::Resource(ResourceEvent::Activate { name }) => {
            state.status = format!("activate {name}");
        }
        HookEvent::Resource(ResourceEvent::Deactivate { name }) => {
            state.status = format!("deactivate {name}");
        }
        HookEvent::Machine(MachineEvent::Done) => unreachable!(),
    }
}

fn enqueue_write(state: &mut State, meta: FragmentMeta, action: &str) {
    let index = state.cells.len();
    let kind = CellKind::from_meta(&meta);
    state.cells.push(TapeCell {
        id: meta.id,
        kind,
        state: CellState::Pending,
    });
    state.queue.push_back(TapeOp::Write {
        index,
        id: meta.id,
        kind,
        label: format!(
            "{action} {}/{} #{} {}",
            meta.role, meta.kind, meta.id, meta.preview
        ),
    });
}

fn enqueue_replace(state: &mut State, meta: FragmentMeta) {
    let kind = CellKind::from_meta(&meta);
    let index = match state.cells.iter().position(|cell| cell.id == meta.id) {
        Some(index) => {
            state.cells[index].kind = kind;
            state.cells[index].state = CellState::Pending;
            index
        }
        None => {
            let index = state.cells.len();
            state.cells.push(TapeCell {
                id: meta.id,
                kind,
                state: CellState::Pending,
            });
            index
        }
    };
    state.queue.push_back(TapeOp::Replace {
        index,
        kind,
        label: format!(
            "replace {}/{} #{} {}",
            meta.role, meta.kind, meta.id, meta.preview
        ),
    });
}

fn enqueue_remove(state: &mut State, id: u64) {
    if let Some(index) = state.cells.iter().position(|cell| cell.id == id) {
        state.cells[index].state = CellState::Removing;
        state.queue.push_back(TapeOp::Remove {
            index,
            id,
            label: format!("remove #{id}"),
        });
    }
}

fn enqueue_swap(state: &mut State, first: u64, second: u64) {
    let first_index = state.cells.iter().position(|cell| cell.id == first);
    let second_index = state.cells.iter().position(|cell| cell.id == second);
    if let (Some(first_index), Some(second_index)) = (first_index, second_index) {
        state.queue.push_back(TapeOp::Swap {
            first: first_index,
            second: second_index,
            label: format!("swap #{first} ↔ #{second}"),
        });
    }
}

fn render_frame(state: &State) {
    let width = crossterm::terminal::size()
        .map(|(width, _)| width as usize)
        .unwrap_or(80);
    let max_cells = (width / CELL_WIDTH).saturating_sub(1);
    let start = state.cells.len().saturating_sub(max_cells);
    let visible = &state.cells[start..];
    let pointer_index = state
        .pointer
        .saturating_sub(start)
        .min(visible.len().saturating_sub(1));
    let pointer_col = pointer_index * CELL_WIDTH;

    let mut out = std::io::stdout();

    execute!(
        out,
        MoveTo(0, state.origin_y),
        Clear(ClearType::CurrentLine)
    )
    .ok();
    if visible.is_empty() {
        write!(out, "{}", "□".with(Color::DarkGrey)).ok();
    } else {
        for cell in visible {
            write!(out, "{} ", cell_glyph(cell).with(cell_color(cell))).ok();
        }
    }

    execute!(
        out,
        MoveTo(0, state.origin_y + 1),
        Clear(ClearType::CurrentLine)
    )
    .ok();
    for _ in 0..pointer_col {
        write!(out, " ").ok();
    }
    write!(out, "{}", "⚙".with(Color::Cyan).bold()).ok();
    write!(out, " {}", status_line(state, width).with(Color::DarkCyan)).ok();
    out.flush().ok();
}

fn status_line(state: &State, width: usize) -> String {
    let position = if state.cells.is_empty() {
        "0/0".to_string()
    } else {
        format!("{}/{}", state.pointer + 1, state.cells.len())
    };
    let mut line = format!("{} {position}", one_line(&state.status));
    let max_len = width.saturating_sub(4).max(20);
    if line.chars().count() > max_len {
        line = line
            .chars()
            .take(max_len.saturating_sub(1))
            .collect::<String>();
        line.push('…');
    }
    line
}

fn one_line(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn cell_glyph(cell: &TapeCell) -> &'static str {
    match cell.state {
        CellState::Pending | CellState::Removing => "□",
        CellState::Written => "■",
    }
}

fn cell_color(cell: &TapeCell) -> Color {
    match cell.state {
        CellState::Removing => Color::Red,
        CellState::Pending => cell.kind.color(),
        CellState::Written => cell.kind.color(),
    }
}
