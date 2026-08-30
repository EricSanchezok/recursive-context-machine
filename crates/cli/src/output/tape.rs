use std::collections::{BTreeMap, VecDeque};
use std::io::{IsTerminal, Write};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use crossterm::{
    cursor::{Hide, MoveTo, Show, position},
    execute,
    style::{Color, Stylize},
    terminal::{Clear, ClearType, size},
};

use crate::hook::{
    CompletionEvent, ComponentEvent, ComponentMeta, FragmentEvent, FragmentMeta, GraphEvent,
    HookEvent, HookKind, MachineEvent, ResourceEvent, ToolEvent,
};

const IDLE_TICK: Duration = Duration::from_millis(50);
const CELL_WIDTH: usize = 2;
const MIN_VIEW_ROWS: u16 = 6;
const MAX_VIEW_ROWS: u16 = 24;
const LONG_JUMP_THRESHOLD: usize = 12;
const MAX_TRAVEL_TICKS: usize = 6;

#[derive(Clone)]
struct TapeCell {
    id: u64,
    tag: String,
    kind: CellKind,
    state: CellState,
}

#[derive(Clone, Copy, PartialEq)]
enum CellState {
    Pending,
    Written,
    Replacing,
    Flashing,
    Taking,
    Removing,
    Swapping,
}

#[derive(Clone, PartialEq)]
enum CellKind {
    SystemText,
    UserText,
    AssistantText,
    Agent,
    Instruction,
    Environment,
    Purpose,
    ToolCall,
    ToolResult,
    Image,
    Audio,
    Video,
    Document,
    Hitch,
    Tagged(String),
    Unknown,
}

impl CellKind {
    fn from_meta(meta: &FragmentMeta) -> Self {
        match meta.tag.as_str() {
            "agent" => Self::Agent,
            "instruction" => Self::Instruction,
            "env" => Self::Environment,
            "purpose" | "purpose_initial" | "purpose_b" => Self::Purpose,
            "" => Self::from_role_and_kind(meta),
            tag if tag == meta.kind || tag == meta.role => Self::from_role_and_kind(meta),
            tag if meta.kind == "text" => Self::Tagged(tag.into()),
            _ => Self::from_role_and_kind(meta),
        }
    }

    fn from_role_and_kind(meta: &FragmentMeta) -> Self {
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

    fn color(&self) -> Color {
        match self {
            Self::SystemText => Color::Rgb {
                r: 119,
                g: 125,
                b: 141,
            },
            Self::UserText => Color::Rgb {
                r: 111,
                g: 176,
                b: 131,
            },
            Self::AssistantText => Color::Rgb {
                r: 219,
                g: 213,
                b: 190,
            },
            Self::Agent => Color::Rgb {
                r: 157,
                g: 132,
                b: 205,
            },
            Self::Instruction => Color::Rgb {
                r: 205,
                g: 160,
                b: 106,
            },
            Self::Environment => Color::Rgb {
                r: 104,
                g: 166,
                b: 157,
            },
            Self::Purpose => Color::Rgb {
                r: 230,
                g: 173,
                b: 91,
            },
            Self::ToolCall => Color::Yellow,
            Self::ToolResult => Color::Cyan,
            Self::Image => Color::Magenta,
            Self::Audio => Color::DarkMagenta,
            Self::Video => Color::DarkYellow,
            Self::Document => Color::DarkGreen,
            Self::Hitch => Color::Red,
            Self::Tagged(tag) => tag_color(tag),
            Self::Unknown => Color::DarkGrey,
        }
    }

    fn name(&self) -> String {
        match self {
            Self::SystemText => "system".into(),
            Self::UserText => "user".into(),
            Self::AssistantText => "assistant".into(),
            Self::Agent => "agent".into(),
            Self::Instruction => "instruction".into(),
            Self::Environment => "env".into(),
            Self::Purpose => "purpose".into(),
            Self::ToolCall => "tool_call".into(),
            Self::ToolResult => "tool_result".into(),
            Self::Image => "image".into(),
            Self::Audio => "audio".into(),
            Self::Video => "video".into(),
            Self::Document => "document".into(),
            Self::Hitch => "hitch".into(),
            Self::Tagged(tag) => format!("tag:{tag}"),
            Self::Unknown => "unknown".into(),
        }
    }

    /// Scaffolding fragments (agent/instruction/env/purpose) are re-emitted every
    /// round — most visibly the `env` timestamp tick. Their updates should flash
    /// in place rather than drag the main pointer back across the whole tape.
    fn is_scaffolding(&self) -> bool {
        matches!(
            self,
            Self::Agent | Self::Instruction | Self::Environment | Self::Purpose
        )
    }
}

fn tag_color(tag: &str) -> Color {
    // 16 evenly-spread hues so distinct tags read as distinct colors. The hash is
    // stable, so a given tag always gets the same tone across runs.
    const PALETTE: [Color; 16] = [
        Color::Rgb {
            r: 196,
            g: 132,
            b: 93,
        },
        Color::Rgb {
            r: 207,
            g: 156,
            b: 78,
        },
        Color::Rgb {
            r: 177,
            g: 151,
            b: 83,
        },
        Color::Rgb {
            r: 170,
            g: 173,
            b: 88,
        },
        Color::Rgb {
            r: 143,
            g: 166,
            b: 105,
        },
        Color::Rgb {
            r: 112,
            g: 170,
            b: 110,
        },
        Color::Rgb {
            r: 101,
            g: 166,
            b: 146,
        },
        Color::Rgb {
            r: 96,
            g: 168,
            b: 173,
        },
        Color::Rgb {
            r: 102,
            g: 157,
            b: 191,
        },
        Color::Rgb {
            r: 121,
            g: 151,
            b: 199,
        },
        Color::Rgb {
            r: 140,
            g: 140,
            b: 205,
        },
        Color::Rgb {
            r: 164,
            g: 133,
            b: 198,
        },
        Color::Rgb {
            r: 188,
            g: 128,
            b: 184,
        },
        Color::Rgb {
            r: 202,
            g: 126,
            b: 154,
        },
        Color::Rgb {
            r: 204,
            g: 124,
            b: 124,
        },
        Color::Rgb {
            r: 189,
            g: 143,
            b: 116,
        },
    ];
    let hash = tag.bytes().fold(0usize, |accumulator, byte| {
        accumulator.wrapping_mul(33) ^ byte as usize
    });
    PALETTE[hash % PALETTE.len()]
}

#[derive(Clone)]
enum TapeOp {
    Write {
        index: usize,
        id: u64,
        kind: CellKind,
        intake: bool,
        label: String,
    },
    Replace {
        index: usize,
        id: u64,
        tag: String,
        kind: CellKind,
        label: String,
    },
    /// In-place update of a scaffolding cell (env/agent/instruction/purpose).
    /// Unlike `Replace`, it never moves the main pointer — it just blinks the
    /// target cell where it sits, so the per-round env tick stops dragging the
    /// pointer across long tapes.
    Flash {
        id: u64,
        tag: String,
        kind: CellKind,
        label: String,
    },
    Remove {
        index: usize,
        id: u64,
        label: String,
    },
    Swap {
        first: u64,
        second: u64,
        label: String,
    },
}

impl TapeOp {
    fn target(&self, tape: &TapeState) -> usize {
        match self {
            Self::Write { index, id, .. }
            | Self::Replace { index, id, .. }
            | Self::Remove { index, id, .. } => tape
                .cells
                .iter()
                .position(|cell| cell.id == *id)
                .unwrap_or(*index),
            Self::Swap { first, .. } => tape
                .cells
                .iter()
                .position(|cell| cell.id == *first)
                .unwrap_or(0),
            // Flash never travels; report its current cell so callers that
            // inspect target stay consistent, but advance_tape handles it early.
            Self::Flash { id, .. } => tape
                .cells
                .iter()
                .position(|cell| cell.id == *id)
                .unwrap_or(0),
        }
    }

    fn label(&self) -> &str {
        match self {
            Self::Write { label, .. }
            | Self::Replace { label, .. }
            | Self::Flash { label, .. }
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
    /// Transient marker for cell-less actions (model switch, tool activate /
    /// deactivate): `(text, remaining_ticks)`. Drawn on the status row and
    /// counted down each animation tick.
    badge: Option<(String, u8)>,
    /// Sticky record of the last action this tape performed, for observability /
    /// snapshot tests. Unlike `status` (which the next op's label overwrites) and
    /// `badge` (which counts down to None), this persists.
    last_action: String,
    /// Ticks left to hold the in-place scaffolding flash, so it reads as a gentle
    /// breathe rather than a one-frame blink.
    flash_remaining: u8,
}

/// How long a resource-action badge stays on screen, in animation ticks.
const BADGE_TICKS: u8 = 4;
/// How long a scaffolding cell stays lit during an in-place flash.
const FLASH_TICKS: u8 = 4;

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
            badge: None,
            last_action: String::new(),
            flash_remaining: 0,
        }
    }

    fn set_badge(&mut self, text: impl Into<String>) {
        self.badge = Some((text.into(), BADGE_TICKS));
    }
}

struct ViewState {
    graph: Option<String>,
    frontier: Option<u64>,
    frontier_count: usize,
    tapes: BTreeMap<String, TapeState>,
    tape_order: Vec<String>,
    summary: Summary,
    origin_y: u16,
    reserved_rows: u16,
}

pub(crate) struct Summary {
    pub(crate) fragments: usize,
    pub(crate) tool_calls: usize,
    pub(crate) duration_s: f64,
    /// Completion (LLM turn) count and token totals for run reports.
    pub(crate) completions: u64,
    pub(crate) input_tokens: u64,
    pub(crate) output_tokens: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TapeSnapshot {
    pub tapes: Vec<TapeSnapshotTape>,
    pub hidden_tapes: usize,
    pub fragments: usize,
    pub tool_calls: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TapeSnapshotTape {
    pub name: String,
    pub pointer: usize,
    pub status: String,
    /// Sticky record of the last action verb (intake / refresh / overwriting /
    /// swapping / activate … ). Survives later ops, unlike `status`.
    pub last_action: String,
    pub cells: Vec<TapeSnapshotCell>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TapeSnapshotCell {
    pub id: u64,
    pub tag: String,
    pub tone: String,
    pub glyph: String,
}

pub fn snapshot_events<I>(events: I, reserved_rows: u16) -> TapeSnapshot
where
    I: IntoIterator<Item = HookEvent>,
{
    let mut view = ViewState {
        graph: None,
        frontier: None,
        frontier_count: 0,
        tapes: BTreeMap::new(),
        tape_order: Vec::new(),
        summary: Summary {
            fragments: 0,
            tool_calls: 0,
            duration_s: 0.0,
            completions: 0,
            input_tokens: 0,
            output_tokens: 0,
        },
        origin_y: 0,
        reserved_rows: reserved_rows.max(MIN_VIEW_ROWS),
    };

    for event in events {
        apply_event(&mut view, event);
    }
    drain_animation_queues(&mut view);
    view.summary.fragments = view.tapes.values().map(|tape| tape.cells.len()).sum();

    let visible_tapes = ordered_visible_tapes(&view);
    let tapes = visible_tapes
        .iter()
        .map(|tape| TapeSnapshotTape {
            name: tape.name.clone(),
            pointer: tape.pointer,
            status: tape.status.clone(),
            last_action: tape.last_action.clone(),
            cells: tape
                .cells
                .iter()
                .map(|cell| TapeSnapshotCell {
                    id: cell.id,
                    tag: cell.tag.clone(),
                    tone: cell.kind.name(),
                    glyph: cell_glyph(cell).into(),
                })
                .collect(),
        })
        .collect::<Vec<_>>();

    TapeSnapshot {
        hidden_tapes: ordered_tapes(&view).len().saturating_sub(tapes.len()),
        fragments: view.summary.fragments,
        tool_calls: view.summary.tool_calls,
        tapes,
    }
}

pub(crate) fn run_animation(
    rx: mpsc::Receiver<HookEvent>,
    delay_ms: u64,
    start: std::time::Instant,
    runtime_thread: &thread::JoinHandle<()>,
) -> Summary {
    if !std::io::stdout().is_terminal() {
        return run_silent(rx, start, runtime_thread);
    }

    let step = Duration::from_millis(delay_ms);
    let reserved_rows = initial_view_rows();
    let origin_y = reserve_animation_rows(reserved_rows);
    let mut view = ViewState {
        graph: None,
        frontier: None,
        frontier_count: 0,
        tapes: BTreeMap::new(),
        tape_order: Vec::new(),
        summary: Summary {
            fragments: 0,
            tool_calls: 0,
            duration_s: 0.0,
            completions: 0,
            input_tokens: 0,
            output_tokens: 0,
        },
        origin_y,
        reserved_rows,
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
            Err(mpsc::RecvTimeoutError::Timeout) if runtime_thread.is_finished() => {
                return finish_animation(&mut view, step, start);
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {}
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                return finish_animation(&mut view, step, start);
            }
        }
    }
}

fn run_silent(
    rx: mpsc::Receiver<HookEvent>,
    start: std::time::Instant,
    runtime_thread: &thread::JoinHandle<()>,
) -> Summary {
    let mut graph_seen = false;
    let mut summary = Summary {
        fragments: 0,
        tool_calls: 0,
        duration_s: 0.0,
        completions: 0,
        input_tokens: 0,
        output_tokens: 0,
    };

    loop {
        let event = match rx.recv_timeout(IDLE_TICK) {
            Ok(event) => event,
            Err(mpsc::RecvTimeoutError::Timeout) if runtime_thread.is_finished() => break,
            Err(mpsc::RecvTimeoutError::Timeout) => continue,
            Err(mpsc::RecvTimeoutError::Disconnected) => break,
        };
        let root_event = event.source.is_none();
        match event.kind {
            HookKind::Graph(GraphEvent::Start { .. }) if root_event => graph_seen = true,
            HookKind::Graph(GraphEvent::Done { .. }) if root_event => break,
            HookKind::Machine(MachineEvent::Done) if !graph_seen && root_event => break,
            HookKind::Fragment(FragmentEvent::Appended(_))
            | HookKind::Fragment(FragmentEvent::Taken(_))
            | HookKind::Fragment(FragmentEvent::Inserted { .. }) => summary.fragments += 1,
            HookKind::Fragment(FragmentEvent::Removed { .. }) => {
                summary.fragments = summary.fragments.saturating_sub(1);
            }
            HookKind::Tool(ToolEvent::Result { .. }) => summary.tool_calls += 1,
            HookKind::Completion(CompletionEvent::End {
                input_tokens,
                output_tokens,
                ..
            }) => {
                summary.completions += 1;
                summary.input_tokens = summary.input_tokens.saturating_add(input_tokens);
                summary.output_tokens = summary.output_tokens.saturating_add(output_tokens);
            }
            _ => {}
        }
    }

    summary.duration_s = start.elapsed().as_secs_f64();
    summary
}

fn initial_view_rows() -> u16 {
    let (_, cursor_y) = position().unwrap_or((0, 0));
    let (_, height) = size().unwrap_or((80, MAX_VIEW_ROWS));
    let remaining = height.saturating_sub(cursor_y.saturating_add(1));
    remaining.clamp(MIN_VIEW_ROWS, MAX_VIEW_ROWS)
}

fn reserve_animation_rows(rows: u16) -> u16 {
    let (_, before_y) = position().unwrap_or((0, 0));
    for _ in 0..rows {
        println!();
    }
    let (_, after_y) = position().unwrap_or((0, before_y.saturating_add(rows)));
    after_y.saturating_sub(rows)
}

fn is_finish_event(view: &ViewState, event: &HookEvent) -> bool {
    match &event.kind {
        HookKind::Graph(GraphEvent::Done { .. }) => event.source.is_none(),
        HookKind::Machine(MachineEvent::Done) => event.source.is_none() && view.graph.is_none(),
        _ => false,
    }
}

fn finish_animation(view: &mut ViewState, step: Duration, start: std::time::Instant) -> Summary {
    while has_pending_animation(view) {
        tick(view, step);
    }
    view.summary.duration_s = start.elapsed().as_secs_f64();
    render_frame(view);
    thread::sleep(Duration::from_millis(250));
    execute!(
        std::io::stdout(),
        Show,
        MoveTo(0, view.origin_y + view.reserved_rows)
    )
    .ok();
    Summary {
        fragments: view.summary.fragments,
        tool_calls: view.summary.tool_calls,
        duration_s: view.summary.duration_s,
        completions: view.summary.completions,
        input_tokens: view.summary.input_tokens,
        output_tokens: view.summary.output_tokens,
    }
}

fn tick(view: &mut ViewState, step: Duration) {
    advance_all_tapes(view);
    render_frame(view);
    thread::sleep(step.max(IDLE_TICK));
}

fn drain_animation_queues(view: &mut ViewState) {
    while has_pending_animation(view) {
        advance_all_tapes(view);
    }
}

fn has_pending_animation(view: &ViewState) -> bool {
    view.tapes
        .values()
        .any(|tape| tape.active.is_some() || !tape.queue.is_empty())
}

fn advance_all_tapes(view: &mut ViewState) {
    for tape in view.tapes.values_mut() {
        advance_tape(tape);
    }
    view.summary.fragments = view.tapes.values().map(|tape| tape.cells.len()).sum();
}

fn advance_tape(tape: &mut TapeState) {
    tick_badge(tape);

    if tape.active.is_none() {
        tape.active = tape.queue.pop_front();
    }

    let Some(op) = tape.active.as_ref() else {
        return;
    };

    // Flash (scaffolding re-emit, e.g. the per-round env tick) never moves the
    // main pointer — it blinks the target cell in place wherever it sits.
    if matches!(op, TapeOp::Flash { .. }) {
        tape.status = op.label().to_string();
        advance_active_op(tape);
        return;
    }

    let target = op.target(tape).min(tape.cells.len().saturating_sub(1));
    tape.status = op.label().to_string();
    match tape.pointer.cmp(&target) {
        std::cmp::Ordering::Less => {
            tape.pointer = target.min(tape.pointer + travel_stride(tape.pointer, target));
            fill_trail(tape);
        }
        std::cmp::Ordering::Greater => {
            tape.pointer = target.max(tape.pointer - travel_stride(tape.pointer, target));
        }
        std::cmp::Ordering::Equal => advance_active_op(tape),
    }
}

/// The head writes everything it passes: any still-pending cell behind the
/// current pointer is committed. Without this, a long jump (stride > 1) leaves a
/// trail of hollow □ cells the head skipped over — the "filled/hollow/filled"
/// patchwork. Only `Pending` is promoted; transient states (removing, swapping,
/// …) are left for their own op to resolve.
fn fill_trail(tape: &mut TapeState) {
    let pointer = tape.pointer;
    for cell in tape.cells.iter_mut().take(pointer) {
        if cell.state == CellState::Pending {
            cell.state = CellState::Written;
        }
    }
}

fn tick_badge(tape: &mut TapeState) {
    if let Some((_, remaining)) = tape.badge.as_mut() {
        *remaining = remaining.saturating_sub(1);
        if *remaining == 0 {
            tape.badge = None;
        }
    }
}

fn travel_stride(pointer: usize, target: usize) -> usize {
    let distance = pointer.abs_diff(target);
    if distance <= LONG_JUMP_THRESHOLD {
        1
    } else {
        distance.div_ceil(MAX_TRAVEL_TICKS).max(1)
    }
}

fn advance_active_op(tape: &mut TapeState) {
    // Intermediate-state passes: each returns after marking a transient state so
    // the action reads as a deliberate motion rather than an instant settle.
    if let Some(TapeOp::Replace { index, id, .. }) = tape.active.as_ref() {
        let current_index = cell_index(tape, *id).unwrap_or(*index);
        if let Some(cell) = tape.cells.get_mut(current_index)
            && cell.state != CellState::Replacing
        {
            cell.state = CellState::Replacing;
            tape.status = "overwriting".into();
            tape.last_action = "overwriting".into();
            return;
        }
    }

    if let Some(TapeOp::Flash { id, .. }) = tape.active.as_ref() {
        let id = *id;
        if let Some(index) = cell_index(tape, id) {
            // Light it, hold it lit for a few ticks (a gentle breathe rather than
            // a one-frame blink), then fall through to settle.
            if tape.cells[index].state != CellState::Flashing {
                tape.cells[index].state = CellState::Flashing;
                tape.flash_remaining = FLASH_TICKS;
                tape.status = "refresh".into();
                tape.last_action = "refresh".into();
                return;
            }
            if tape.flash_remaining > 0 {
                tape.flash_remaining -= 1;
                return;
            }
        }
    }

    if let Some(TapeOp::Write {
        index,
        id,
        intake: true,
        ..
    }) = tape.active.as_ref()
    {
        let current_index = cell_index(tape, *id).unwrap_or(*index);
        if let Some(cell) = tape.cells.get_mut(current_index)
            && cell.state != CellState::Taking
        {
            cell.state = CellState::Taking;
            tape.status = "intake".into();
            tape.last_action = "intake".into();
            return;
        }
    }

    let swap_ids = match tape.active.as_ref() {
        Some(TapeOp::Swap { first, second, .. }) => Some((*first, *second)),
        _ => None,
    };
    if let Some((first, second)) = swap_ids
        && mark_swap_cells(tape, first, second)
    {
        tape.status = "swapping".into();
        tape.last_action = "swapping".into();
        return;
    }

    let Some(op) = tape.active.take() else {
        return;
    };

    match op {
        TapeOp::Write {
            index, id, kind, ..
        } => {
            let current_index = cell_index(tape, id).unwrap_or(index);
            if let Some(cell) = tape.cells.get_mut(current_index) {
                cell.id = id;
                cell.kind = kind;
                cell.state = CellState::Written;
            }
        }
        TapeOp::Replace {
            index,
            id,
            tag,
            kind,
            ..
        } => {
            let current_index = cell_index(tape, id).unwrap_or(index);
            if let Some(cell) = tape.cells.get_mut(current_index) {
                cell.tag = tag;
                cell.kind = kind;
                cell.state = CellState::Written;
            }
        }
        TapeOp::Flash { id, tag, kind, .. } => {
            if let Some(index) = cell_index(tape, id) {
                let cell = &mut tape.cells[index];
                cell.tag = tag;
                cell.kind = kind;
                cell.state = CellState::Written;
            }
        }
        TapeOp::Remove { index, id, .. } => {
            let current_index = cell_index(tape, id).unwrap_or(index);
            if tape
                .cells
                .get(current_index)
                .is_some_and(|cell| cell.id == id)
            {
                tape.cells.remove(current_index);
                if !tape.cells.is_empty() {
                    tape.pointer = tape.pointer.min(tape.cells.len() - 1);
                } else {
                    tape.pointer = 0;
                }
            }
        }
        TapeOp::Swap { first, second, .. } => {
            if let (Some(first_index), Some(second_index)) =
                (cell_index(tape, first), cell_index(tape, second))
            {
                tape.cells.swap(first_index, second_index);
                if let Some(cell) = tape.cells.get_mut(first_index) {
                    cell.state = CellState::Written;
                }
                if let Some(cell) = tape.cells.get_mut(second_index) {
                    cell.state = CellState::Written;
                }
            }
        }
    }
}

fn mark_swap_cells(tape: &mut TapeState, first: u64, second: u64) -> bool {
    let (Some(first_index), Some(second_index)) =
        (cell_index(tape, first), cell_index(tape, second))
    else {
        return false;
    };
    if tape.cells[first_index].state == CellState::Swapping
        && tape.cells[second_index].state == CellState::Swapping
    {
        return false;
    }
    tape.cells[first_index].state = CellState::Swapping;
    tape.cells[second_index].state = CellState::Swapping;
    true
}

fn cell_index(tape: &TapeState, id: u64) -> Option<usize> {
    tape.cells.iter().position(|cell| cell.id == id)
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
            if let CompletionEvent::End {
                input_tokens,
                output_tokens,
                ..
            } = &event
            {
                view.summary.completions += 1;
                view.summary.input_tokens = view.summary.input_tokens.saturating_add(*input_tokens);
                view.summary.output_tokens =
                    view.summary.output_tokens.saturating_add(*output_tokens);
            }
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
            if meta.kind != "accelerator" {
                return;
            }
            let tape = tape_for_source(view, Some(&meta));
            tape.name = meta.name;
            tape.kind = meta.kind;
            tape.frontier = meta.frontier;
            tape.lifecycle = TapeStatus::Running;
            tape.status = "start".into();
        }
        ComponentEvent::Done(meta) => {
            if meta.kind != "accelerator" {
                return;
            }
            let tape = tape_for_source(view, Some(&meta));
            tape.name = meta.name;
            tape.kind = meta.kind;
            tape.frontier = meta.frontier;
            tape.lifecycle = TapeStatus::Done;
            tape.status = "done".into();
        }
        ComponentEvent::Skipped(meta) => {
            if meta.kind != "accelerator" {
                return;
            }
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
        FragmentEvent::Appended(meta) => enqueue_write(tape, meta, "write"),
        FragmentEvent::Taken(meta) => enqueue_take(tape, meta),
        FragmentEvent::Inserted { meta, after } => enqueue_insert(tape, meta, after),
        FragmentEvent::Replaced(meta) => enqueue_replace(tape, meta),
        FragmentEvent::Removed { id } => enqueue_remove(tape, id),
        FragmentEvent::Swapped { first, second } => enqueue_swap(tape, first, second),
    }
}

fn apply_tool_event(view: &mut ViewState, source: Option<&ComponentMeta>, event: ToolEvent) {
    match event {
        ToolEvent::Call {
            call_id,
            tool,
            arguments,
        } => {
            tape_for_source(view, source).status =
                format!("tool {tool} #{call_id} args={}B", arguments.len());
        }
        ToolEvent::Result {
            call_id,
            tool,
            result_len,
            duration,
        } => {
            view.summary.tool_calls += 1;
            let tape = tape_for_source(view, source);
            tape.tool_calls += 1;
            tape.status = format!("tool {tool} #{call_id} done {result_len}B {duration}");
        }
        ToolEvent::Error {
            call_id,
            tool,
            error,
            retryable,
        } => {
            let retry = if retryable { "retry" } else { "fatal" };
            tape_for_source(view, source).status =
                format!("tool {tool} #{call_id} error {retry}: {error}");
        }
    }
}

fn apply_completion_event(tape: &mut TapeState, event: CompletionEvent) {
    match event {
        CompletionEvent::Start => tape.status = "completion".into(),
        CompletionEvent::End { fragments, .. } => {
            tape.status = format!("drain {fragments} fragments")
        }
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
    // Resource actions have no cell, so they surface as a transient badge on the
    // status row (held for a few ticks) plus the status text.
    let (badge, status) = match event {
        ResourceEvent::Model { name } => (format!("⟳ model {name}"), format!("model {name}")),
        ResourceEvent::Activate { name } => {
            (format!("⚡ activate {name}"), format!("activate {name}"))
        }
        ResourceEvent::Deactivate { name } => {
            (format!("⌁ deactivate {name}"), format!("deactivate {name}"))
        }
    };
    tape.set_badge(badge);
    tape.status = status.clone();
    tape.last_action = status;
}

fn tape_for_source<'a>(
    view: &'a mut ViewState,
    source: Option<&ComponentMeta>,
) -> &'a mut TapeState {
    let key = source.map(component_key).unwrap_or_else(|| "main".into());
    if !view.tapes.contains_key(&key) {
        view.tape_order.push(key.clone());
        let tape = match source {
            Some(meta) => TapeState::new(meta.name.clone(), meta.kind.clone()),
            None => TapeState::new("accelerator", "accelerator"),
        };
        view.tapes.insert(key.clone(), tape);
    }
    view.tapes.get_mut(&key).expect("tape was just inserted")
}

fn component_key(meta: &ComponentMeta) -> String {
    format!("{}:{}:{}", meta.graph, meta.index, meta.name)
}

fn enqueue_write(tape: &mut TapeState, meta: FragmentMeta, action: &str) {
    let index = tape.cells.len();
    enqueue_cell_write(tape, meta, index, action, false);
}

fn enqueue_take(tape: &mut TapeState, meta: FragmentMeta) {
    let index = tape.cells.len();
    enqueue_cell_write(tape, meta, index, "take", true);
}

fn enqueue_insert(tape: &mut TapeState, meta: FragmentMeta, after: u64) {
    let index = tape
        .cells
        .iter()
        .position(|cell| cell.id == after)
        .map(|index| index + 1)
        .unwrap_or(tape.cells.len());
    enqueue_cell_write(tape, meta, index, "insert", false);
}

fn enqueue_cell_write(
    tape: &mut TapeState,
    meta: FragmentMeta,
    index: usize,
    action: &str,
    intake: bool,
) {
    let kind = CellKind::from_meta(&meta);
    let cell = TapeCell {
        id: meta.id,
        tag: meta.tag.clone(),
        kind: kind.clone(),
        state: CellState::Pending,
    };
    if index >= tape.cells.len() {
        tape.cells.push(cell);
    } else {
        tape.cells.insert(index, cell);
    }
    tape.queue.push_back(TapeOp::Write {
        index,
        id: meta.id,
        kind,
        intake,
        label: fragment_label(action, &meta),
    });
}

fn enqueue_replace(tape: &mut TapeState, meta: FragmentMeta) {
    let kind = CellKind::from_meta(&meta);
    if tape.cells.iter().all(|cell| cell.id != meta.id) {
        enqueue_write(tape, meta, "write");
        return;
    }
    // Scaffolding (env/agent/instruction/purpose) is re-emitted every round;
    // flash it in place so the pointer doesn't run back across the tape.
    if kind.is_scaffolding() {
        tape.queue.push_back(TapeOp::Flash {
            id: meta.id,
            tag: meta.tag.clone(),
            kind,
            label: fragment_label("refresh", &meta),
        });
        return;
    }
    let index = tape
        .cells
        .iter()
        .position(|cell| cell.id == meta.id)
        .expect("id presence checked above");
    tape.queue.push_back(TapeOp::Replace {
        index,
        id: meta.id,
        tag: meta.tag.clone(),
        kind,
        label: fragment_label("replace", &meta),
    });
}

fn fragment_label(action: &str, meta: &FragmentMeta) -> String {
    let tag = if meta.tag.is_empty() {
        String::new()
    } else {
        format!(" [{}]", meta.tag)
    };
    format!(
        "{action} {}/{}{} #{} {}",
        meta.role, meta.kind, tag, meta.id, meta.preview
    )
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
    if cell_index(tape, first).is_some() && cell_index(tape, second).is_some() {
        tape.queue.push_back(TapeOp::Swap {
            first,
            second,
            label: format!("swap #{first} ↔ #{second}"),
        });
    }
}

fn render_frame(view: &mut ViewState) {
    let width = crossterm::terminal::size()
        .map(|(width, _)| width as usize)
        .unwrap_or(80);
    let inner = width.saturating_sub(4).max(20);
    let mut out = std::io::stdout();

    for row in 0..view.reserved_rows {
        execute!(
            out,
            MoveTo(0, view.origin_y + row),
            Clear(ClearType::CurrentLine)
        )
        .ok();
    }

    let visible_tapes = ordered_visible_tapes(view);
    let hidden_tapes = ordered_tapes(view)
        .len()
        .saturating_sub(visible_tapes.len());

    draw_border(&mut out, view.origin_y, width, &view_title(view));
    draw_graph_status(&mut out, view.origin_y + 1, inner, view, hidden_tapes);

    let mut row = view.origin_y + 2;
    let last_body_row = view.origin_y + view.reserved_rows - 2;
    for tape in visible_tapes {
        if row + 2 > last_body_row {
            break;
        }
        draw_tape_card(&mut out, row, inner, tape);
        row += 3;
    }

    draw_footer(
        &mut out,
        view.origin_y + view.reserved_rows - 1,
        width,
        view,
    );
    out.flush().ok();
}

fn view_title(view: &ViewState) -> String {
    match &view.graph {
        Some(graph) => format!("Graph · {graph}"),
        None => "Context Tape".into(),
    }
}

fn draw_graph_status(
    out: &mut std::io::Stdout,
    row: u16,
    inner: usize,
    view: &ViewState,
    hidden_tapes: usize,
) {
    execute!(out, MoveTo(0, row), Clear(ClearType::CurrentLine)).ok();
    let running = view
        .tapes
        .values()
        .filter(|tape| tape.kind == "accelerator" && tape.lifecycle == TapeStatus::Running)
        .count();
    let done = view
        .tapes
        .values()
        .filter(|tape| tape.kind == "accelerator" && tape.lifecycle == TapeStatus::Done)
        .count();
    let skipped = view
        .tapes
        .values()
        .filter(|tape| tape.kind == "accelerator" && tape.lifecycle == TapeStatus::Skipped)
        .count();
    let frontier = view
        .frontier
        .map(|frontier| format!("frontier #{frontier}"))
        .unwrap_or_else(|| "booting".into());
    let hidden = if hidden_tapes == 0 {
        String::new()
    } else {
        format!(" · +{hidden_tapes} hidden")
    };
    let line = format!(
        "{frontier} · running {running}/{} · done {done} · skipped {skipped}{hidden}",
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
    draw_tape_status(out, row + 2, inner, tape);
}

fn ordered_visible_tapes(view: &ViewState) -> Vec<&TapeState> {
    let tapes = ordered_tapes(view);
    let capacity = visible_tape_capacity(view);
    if tapes.len() <= capacity {
        return tapes;
    }

    let mut visible = Vec::new();
    for tape in tapes
        .iter()
        .copied()
        .filter(|tape| tape.lifecycle == TapeStatus::Running)
    {
        if visible.len() == capacity {
            return visible;
        }
        visible.push(tape);
    }
    for tape in tapes {
        if visible.len() == capacity {
            break;
        }
        if !visible
            .iter()
            .any(|visible_tape| std::ptr::eq(*visible_tape, tape))
        {
            visible.push(tape);
        }
    }
    visible
}

fn visible_tape_capacity(view: &ViewState) -> usize {
    ((view.reserved_rows.saturating_sub(3)) / 3).max(1) as usize
}

fn ordered_tapes(view: &ViewState) -> Vec<&TapeState> {
    view.tape_order
        .iter()
        .filter_map(|key| view.tapes.get(key))
        .filter(|tape| tape.kind == "accelerator")
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
        "─".with(Color::DarkGrey),
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
        "─".with(Color::DarkGrey),
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

fn draw_tape_status(out: &mut std::io::Stdout, row: u16, inner: usize, tape: &TapeState) {
    execute!(out, MoveTo(0, row), Clear(ClearType::CurrentLine)).ok();
    let (start, _) = visible_window(tape, inner);
    let ellipsis_offset = if start > 0 { 2 } else { 0 };
    let pointer_offset = tape.pointer.saturating_sub(start) * CELL_WIDTH;
    let gear_col = (ellipsis_offset + pointer_offset).min(inner.saturating_sub(2));
    let badge = tape
        .badge
        .as_ref()
        .map(|(text, _)| format!("{text} · "))
        .unwrap_or_default();
    let status = format!(
        "{}{} · {} · {} cells · {} tools",
        badge,
        one_line(&tape.status),
        position_text(tape),
        tape.cells.len(),
        tape.tool_calls
    );
    let status = trim_to_width(&status, inner.saturating_sub(gear_col + 2));
    let used = gear_col + 2 + status.chars().count();
    write!(
        out,
        "{} {}{} {}{}{}",
        "│".with(Color::DarkGrey),
        " ".repeat(gear_col),
        "⚙".with(Color::Cyan).bold(),
        status.with(Color::DarkCyan),
        " ".repeat(inner.saturating_sub(used)),
        "│".with(Color::DarkGrey),
    )
    .ok();
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
        CellState::Flashing => "◇",
        CellState::Taking => "▼",
        CellState::Swapping => "◆",
        CellState::Written => "■",
    }
}

fn cell_color(cell: &TapeCell) -> Color {
    match cell.state {
        // Red is reserved for errors (hitch). A removal is a normal action, so it
        // fades out in dim grey rather than signalling an error.
        CellState::Removing => Color::DarkGrey,
        CellState::Replacing => Color::Yellow,
        CellState::Flashing => Color::White,
        CellState::Taking => Color::Green,
        CellState::Swapping => Color::Magenta,
        CellState::Pending | CellState::Written => cell.kind.color(),
    }
}
