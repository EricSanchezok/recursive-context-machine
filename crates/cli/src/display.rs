use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use crossterm::{
    cursor::{Hide, MoveTo, Show, position},
    execute,
    style::{Color, Stylize},
    terminal::{Clear, ClearType},
};
use std::io::Write;

use crate::hook::HookEvent;

const IDLE_TICK: Duration = Duration::from_millis(50);
const CELL_WIDTH: usize = 2;

#[derive(Clone)]
struct TapeBlock {
    id: u64,
    kind: FragmentKind,
}

#[derive(Clone, Copy, PartialEq)]
enum FragmentKind {
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

impl FragmentKind {
    fn from_parts(role: &str, kind: &str) -> Self {
        match kind {
            "tool_call" => Self::ToolCall,
            "tool_result" => Self::ToolResult,
            "image" => Self::Image,
            "audio" => Self::Audio,
            "video" => Self::Video,
            "document" => Self::Document,
            "hitch" => Self::Hitch,
            "text" => match role {
                "system" => Self::SystemText,
                "user" => Self::UserText,
                "assistant" => Self::AssistantText,
                _ => Self::Unknown,
            },
            _ => Self::Unknown,
        }
    }

    fn glyph(self) -> &'static str {
        match self {
            Self::SystemText => "▓",
            Self::UserText => "░",
            Self::AssistantText => "□",
            Self::ToolCall => "⚒",
            Self::ToolResult => "■",
            Self::Image => "◆",
            Self::Audio => "♪",
            Self::Video => "▶",
            Self::Document => "▤",
            Self::Hitch => "!",
            Self::Unknown => "·",
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

pub(crate) struct State {
    blocks: Vec<TapeBlock>,
    pointer: usize,
    target: usize,
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
    let origin_y = position().map(|(_, y)| y).unwrap_or(0);
    let mut state = State {
        blocks: Vec::new(),
        pointer: 0,
        target: 0,
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
            if matches!(event, HookEvent::Done) {
                state.summary.duration_s = start.elapsed().as_secs_f64();
                finish_animation(&mut state, step);
                return state.summary;
            }
            apply_event(&mut state, event);
        }

        if state.pointer < state.target {
            state.pointer += 1;
            render_frame(&state);
            thread::sleep(step);
        } else {
            render_frame(&state);
            thread::sleep(IDLE_TICK);
        }

        match rx.recv_timeout(IDLE_TICK) {
            Ok(HookEvent::Done) => {
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
    state.status = "done".into();
    while state.pointer < state.target {
        state.pointer += 1;
        render_frame(state);
        thread::sleep(step);
    }
    render_frame(state);
    thread::sleep(Duration::from_millis(250));
    execute!(std::io::stdout(), Show, MoveTo(0, state.origin_y + 2)).ok();
}

fn apply_event(state: &mut State, event: HookEvent) {
    match event {
        HookEvent::FragmentAppended { id, role, kind, .. }
        | HookEvent::FragmentTaken { id, role, kind, .. } => {
            state.blocks.push(TapeBlock {
                id,
                kind: FragmentKind::from_parts(&role, &kind),
            });
            state.target = state.blocks.len();
            state.summary.fragments = state.blocks.len();
            state.status = format!("write {role}/{kind}");
        }
        HookEvent::FragmentInserted { id, role, kind, .. } => {
            state.blocks.push(TapeBlock {
                id,
                kind: FragmentKind::from_parts(&role, &kind),
            });
            state.target = state.blocks.len();
            state.summary.fragments = state.blocks.len();
            state.status = format!("insert {role}/{kind}");
        }
        HookEvent::FragmentReplaced { id, role, kind, .. } => {
            let next_kind = FragmentKind::from_parts(&role, &kind);
            if let Some(block) = state.blocks.iter_mut().find(|block| block.id == id) {
                block.kind = next_kind;
            } else {
                state.blocks.push(TapeBlock {
                    id,
                    kind: next_kind,
                });
            }
            state.target = state.blocks.len();
            state.status = format!("replace {role}/{kind}");
        }
        HookEvent::FragmentRemoved { id } => {
            state.blocks.retain(|block| block.id != id);
            state.pointer = state.pointer.min(state.blocks.len());
            state.target = state.blocks.len();
            state.summary.fragments = state.blocks.len();
            state.status = format!("remove #{id}");
        }
        HookEvent::FragmentsSwapped { first, second } => {
            let first_index = state.blocks.iter().position(|block| block.id == first);
            let second_index = state.blocks.iter().position(|block| block.id == second);
            if let (Some(first_index), Some(second_index)) = (first_index, second_index) {
                state.blocks.swap(first_index, second_index);
            }
            state.status = format!("swap #{first} ↔ #{second}");
        }
        HookEvent::ToolCall { tool, .. } => {
            state.status = format!("tool {tool}");
        }
        HookEvent::ToolResult { tool, .. } => {
            state.summary.tool_calls += 1;
            state.status = format!("tool {tool} done");
        }
        HookEvent::ToolError { tool, .. } => {
            state.status = format!("tool {tool} error");
        }
        HookEvent::CompletionStart => {
            state.status = "completion".into();
        }
        HookEvent::CompletionEnd { .. } => {
            state.status = "draining".into();
        }
        HookEvent::Halt { round } => {
            state.status = format!("halt #{round}");
        }
        HookEvent::Model { name } => {
            state.status = format!("model {name}");
        }
        HookEvent::Activate { name } => {
            state.status = format!("activate {name}");
        }
        HookEvent::Deactivate { name } => {
            state.status = format!("deactivate {name}");
        }
        HookEvent::MachineStart => {
            state.status = "start".into();
        }
        HookEvent::Done => unreachable!(),
    }
}

fn render_frame(state: &State) {
    let width = crossterm::terminal::size()
        .map(|(width, _)| width as usize)
        .unwrap_or(80);
    let max_blocks = (width / CELL_WIDTH).saturating_sub(1);
    let start = state.blocks.len().saturating_sub(max_blocks);
    let visible = &state.blocks[start..];
    let pointer_index = state.pointer.saturating_sub(1).saturating_sub(start);
    let pointer_col = if state.pointer == 0 {
        0
    } else {
        pointer_index.min(visible.len().saturating_sub(1)) * CELL_WIDTH
    };

    let mut out = std::io::stdout();

    execute!(
        out,
        MoveTo(0, state.origin_y),
        Clear(ClearType::CurrentLine)
    )
    .ok();
    if visible.is_empty() {
        write!(out, "{}", "·".with(Color::DarkGrey)).ok();
    } else {
        for block in visible {
            write!(
                out,
                "{} ",
                block.kind.glyph().to_string().with(block.kind.color())
            )
            .ok();
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
    write!(
        out,
        " {}",
        format!("{} {}/{}", state.status, state.pointer, state.blocks.len()).with(Color::DarkCyan)
    )
    .ok();
    out.flush().ok();
}
