use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute,
    style::{Color, Stylize},
    terminal::{Clear, ClearType},
};
use std::io::Write;

use crate::hook::HookEvent;

const TICK: Duration = Duration::from_millis(50);

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum FragmentKind {
    System,
    User,
    Assistant,
    ToolCall,
    ToolResult,
}

impl FragmentKind {
    fn from_role(role: &str) -> Self {
        match role {
            "System" => Self::System,
            "User" => Self::User,
            "Assistant" => Self::Assistant,
            "Tool" => Self::ToolCall,
            _ => Self::System,
        }
    }

    fn glyph_color(self) -> (&'static str, Color) {
        match self {
            Self::System => ("▓", Color::Blue),
            Self::User => ("░", Color::Green),
            Self::Assistant => ("▢", Color::White),
            Self::ToolCall => ("▒", Color::Yellow),
            Self::ToolResult => ("█", Color::Cyan),
        }
    }
}

pub(crate) struct State {
    blocks: Vec<FragmentKind>,
    pointer: usize,
    target: usize,
    tool_label: Option<String>,
    tool_progress: usize,
    summary: Summary,
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
    let mut state = State {
        blocks: Vec::new(),
        pointer: 0,
        target: 0,
        tool_label: None,
        tool_progress: 0,
        summary: Summary {
            fragments: 0,
            tool_calls: 0,
            duration_s: 0.0,
        },
    };

    execute!(std::io::stdout(), Hide).ok();

    // drain initial events
    for ev in rx.try_iter() {
        apply_event(&mut state, ev);
    }

    loop {
        // process available events
        for ev in rx.try_iter() {
            if matches!(ev, HookEvent::Done) {
                state.summary.duration_s = start.elapsed().as_secs_f64();
                // finish animation: march pointer to target
                while state.pointer < state.target {
                    state.pointer += 1;
                    render_frame(&state);
                    thread::sleep(step);
                }
                render_frame(&state);
                thread::sleep(Duration::from_millis(300));
                execute!(std::io::stdout(), Show, MoveTo(0, 0)).ok();
                return state.summary;
            }
            apply_event(&mut state, ev);
        }

        // march pointer one step
        if state.pointer < state.target {
            state.pointer += 1;
            if state
                .blocks
                .get(state.pointer.saturating_sub(1))
                .map_or(false, |k| *k == FragmentKind::ToolCall)
            {
                state.tool_progress += 1;
            }
            render_frame(&state);
            thread::sleep(step);
        } else {
            render_frame(&state);
            thread::sleep(TICK);
        }

        // blocking wait with timeout — may return more events
        match rx.recv_timeout(TICK) {
            Ok(HookEvent::Done) => {
                state.summary.duration_s = start.elapsed().as_secs_f64();
                while state.pointer < state.target {
                    state.pointer += 1;
                    render_frame(&state);
                    thread::sleep(step);
                }
                render_frame(&state);
                thread::sleep(Duration::from_millis(300));
                execute!(std::io::stdout(), Show, MoveTo(0, 0)).ok();
                return state.summary;
            }
            Ok(ev) => {
                apply_event(&mut state, ev);
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {}
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                // machine thread finished without sending Done — process what we have
                state.summary.duration_s = start.elapsed().as_secs_f64();
                while state.pointer < state.target {
                    state.pointer += 1;
                    render_frame(&state);
                    thread::sleep(step);
                }
                render_frame(&state);
                thread::sleep(Duration::from_millis(200));
                break;
            }
        }
    }

    execute!(std::io::stdout(), Show, MoveTo(0, 0)).ok();
    state.summary
}

fn apply_event(state: &mut State, ev: HookEvent) {
    match ev {
        HookEvent::FragmentAppended { role, .. } => {
            state.blocks.push(FragmentKind::from_role(&role));
            state.target = state.blocks.len();
        }
        HookEvent::ToolCall { tool, .. } => {
            state.blocks.push(FragmentKind::ToolCall);
            state.target = state.blocks.len();
            state.tool_label = Some(tool);
            state.tool_progress = 0;
        }
        HookEvent::ToolResult { .. } => {
            state.blocks.push(FragmentKind::ToolResult);
            state.target = state.blocks.len();
            state.tool_label = None;
            state.summary.tool_calls += 1;
        }
        HookEvent::CompletionStart => {
            state.tool_progress = 0;
        }
        HookEvent::CompletionEnd { fragments } => {
            state.summary.fragments += fragments;
        }
        HookEvent::Halt { .. }
        | HookEvent::MachineStart
        | HookEvent::FragmentTaken { .. }
        | HookEvent::FragmentInserted { .. }
        | HookEvent::FragmentReplaced { .. }
        | HookEvent::ToolError { .. } => {}
        HookEvent::Done => unreachable!(),
    }
}

fn render_frame(state: &State) {
    let term_width = crossterm::terminal::size()
        .map(|(w, _)| w as usize)
        .unwrap_or(80);

    let (tape, pointer_col) = build_tape_and_pointer(state, term_width);

    // Row 0: tape
    execute!(
        std::io::stdout(),
        MoveTo(0, 0),
        Clear(ClearType::CurrentLine)
    )
    .ok();
    let mut out = std::io::stdout();
    for (i, (ch, color)) in tape.iter().enumerate() {
        if i == pointer_col {
            write!(out, "{}", "⚙".to_string().with(Color::Cyan).bold()).ok();
        }
        write!(out, "{}", ch.to_string().with(*color)).ok();
    }

    // Row 1: pointer arrow + label
    execute!(
        std::io::stdout(),
        MoveTo(0, 1),
        Clear(ClearType::CurrentLine)
    )
    .ok();
    let indent = pointer_col.saturating_sub(1);
    for _ in 0..indent {
        write!(out, " ").ok();
    }
    write!(out, "{}", "╰╴".to_string().with(Color::Cyan)).ok();

    if let Some(tool) = &state.tool_label {
        let elapsed = state.tool_progress * 50 / 1000;
        write!(
            out,
            " {} ({}s)",
            tool.to_string().with(Color::Yellow),
            elapsed
        )
        .ok();
    } else {
        write!(
            out,
            "{}",
            build_label(state).to_string().with(Color::DarkCyan)
        )
        .ok();
    }
    out.flush().ok();
}

fn build_tape_and_pointer(state: &State, max_width: usize) -> (Vec<(String, Color)>, usize) {
    let max_blocks = max_width / 2;
    let show = &state.blocks[..state.blocks.len().min(max_blocks)];
    let mut tape: Vec<(String, Color)> = Vec::with_capacity(show.len() * 2 + 1);
    let mut pointer_col = 0;
    let end = state.pointer.min(show.len());

    for (i, block) in show.iter().enumerate() {
        let (ch, color) = block.glyph_color();
        if i < end {
            // already processed — full brightness
            tape.push((ch.into(), color));
        } else if i == end && state.pointer == state.target {
            // target reached — show dim with idle marker
            tape.push((ch.into(), color));
        } else if i == end {
            // next to be processed — dim
            tape.push((ch.into(), color));
            pointer_col = tape.len();
            if state.pointer < state.target {
                tape.push(("▶".into(), Color::Cyan));
            } else {
                tape.push(("▷".into(), Color::Cyan));
            }
        } else {
            // future — dim
            tape.push((ch.into(), color));
        }
    }

    if state.blocks.is_empty() {
        tape.push(("◉".into(), Color::DarkCyan));
        pointer_col = 1;
    }

    (tape, pointer_col)
}

fn build_label(state: &State) -> String {
    let total = state.blocks.len();
    let idx = state.pointer.min(total);
    if total == 0 {
        if state.target > 0 {
            return format!("assembling… 0/{}", state.target);
        }
        return "waiting…".into();
    }
    let pct = if total > 0 { idx * 100 / total } else { 0 };
    format!("step {}/{} ({}%)", idx, total, pct)
}
