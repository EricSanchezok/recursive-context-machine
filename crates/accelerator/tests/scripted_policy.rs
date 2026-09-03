//! ScriptedPolicy 端到端测试 (R4-min 后半段): Rhai 沙箱策略驱动真实
//! fire loop, seed-basic 与 captain 脚手架等价, 运行时错误兜底 Done,
//! 只读 view 透传, 确定性, 无 IO 沙箱。无 LLM / 无网络。

use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use accelerator::policy::ScriptedPolicy;
use accelerator::{Accelerator, Captain, Catalog};
use machine::{
    Action, Context, Environment, ExecutionMode, Machine, MachineFrame, MachineState, Model,
    Policy, PolicyView, Purpose, Resources, RunState, Telemetry, Tool, ToolDefinition, ToolResult,
    ToolRuntime,
};
use serde_json::json;
use tokio::time::timeout;

struct NamedTool {
    name: &'static str,
}

impl Tool for NamedTool {
    fn name(&self) -> &str {
        self.name
    }

    fn description(&self) -> &str {
        "test tool"
    }

    fn parameters(&self) -> serde_json::Value {
        json!({"type": "object"})
    }

    fn execute<'a>(
        &'a self,
        _args: serde_json::Value,
        _env: &'a Environment,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<ToolResult, String>> + Send + 'a>>
    {
        Box::pin(async {
            Ok(ToolResult {
                call_id: String::new(),
                content: String::new(),
                title: None,
            })
        })
    }
}

fn named_tool(name: &'static str) -> Arc<dyn Tool> {
    Arc::new(NamedTool { name })
}

/// 与 captain 测试同款资源: 两模型 (fast/careful)、两工具 (read/search)
/// 与 captain 提示文本。工具只入定义表不激活 —— 由策略的 Model/Activate
/// 动作激活 (验证动作真的改状态)。
fn resources() -> Resources {
    let mut resources = Resources::named("test")
        .with_model(Model {
            name: "fast".into(),
            ..Default::default()
        })
        .with_model(Model {
            name: "careful".into(),
            ..Default::default()
        })
        .with_tool_definition(ToolDefinition::from_tool(named_tool("read").as_ref()))
        .with_tool_definition(ToolDefinition::from_tool(named_tool("search").as_ref()));
    resources
        .prompts
        .insert("captain".into(), "Captain prompt".into());
    resources
}

/// 手动驱动态 (captain 测试同款): 无 run_dir, 沙箱 cwd, 空上下文。
/// instruction 分歧: captain 读进程 CWD 的 AGENTS.md 插 instruction cell,
/// 沙箱脚本不能读文件 — 等价断言按锚定 cell 过滤 (instruction 无锚)。
fn manual_state(resources: Resources, purpose: &str) -> MachineState {
    MachineState {
        run: RunState {
            purpose: Purpose::new(purpose),
            run_dir: None,
            context: Context::new(),
            environment: Environment::empty("/sandbox"),
            resources,
            telemetry: Telemetry::default(),
        },
        frame: MachineFrame::default(),
    }
}

/// fire-loop 基态 (tool_families 同款): run_dir 挂上 (env 文本含 run_dir),
/// 资源含模型/工具定义。input 侧环境留空 → merge 整体采纳本环境。
fn base_state(run_dir: &Path) -> RunState {
    let mut state = RunState {
        run_dir: Some(run_dir.to_path_buf()),
        ..Default::default()
    };
    state.environment.run_dir = Some(run_dir.to_path_buf());
    state.resources = resources();
    state
}

fn run_input(run_dir: &Path, purpose: &str) -> RunState {
    RunState {
        purpose: Purpose::new(purpose),
        run_dir: Some(run_dir.to_path_buf()),
        environment: Environment::empty(""),
        ..Default::default()
    }
}

fn seed_from_catalog() -> Box<dyn Policy> {
    Catalog::new()
        .policy("seed-basic")
        .expect("seed-basic registered")
}

fn text_of_anchor(context: &machine::Context, anchor: &str) -> Option<String> {
    context
        .fragments()
        .iter()
        .find(|cell| cell.anchor.as_deref() == Some(anchor))
        .map(|cell| cell.content_as_text())
}

fn snapshot(context: &machine::Context) -> Vec<(String, Option<String>, String, String)> {
    context
        .fragments()
        .iter()
        .map(|cell| {
            (
                format!("{:?}", cell.role),
                cell.anchor.clone(),
                cell.tag.clone(),
                cell.content_as_text(),
            )
        })
        .collect()
}

/// 驱动策略直到 Halt/Done (不施加终态动作) — captain 测试同款, 上限 64 步。
async fn drive_until_terminal(policy: &dyn Policy, state: &mut MachineState) -> Action {
    let mut machine = Machine::new("drive", "drive");
    let runtime = ToolRuntime::new();
    for _ in 0..64 {
        let obs = machine::obs::measure(&state.run);
        let action = policy
            .decide(PolicyView {
                run: &state.run,
                inbox: &state.frame.inbox,
                step: state.frame.step,
                status: state.frame.status,
                obs: &obs,
            })
            .await;
        match &action {
            Action::Halt | Action::Done => return action,
            _ => {
                machine
                    .apply(
                        action,
                        state,
                        ExecutionMode::Live {
                            tool_runtime: &runtime,
                            overlay: &machine::Overlay::default(),
                        },
                    )
                    .await;
            }
        }
    }
    panic!("policy did not reach a terminal action within step budget");
}

/// seed-basic 经真实 fire loop 完成脚手架并终态 (r_cat: 目录名可取用)。
#[tokio::test]
async fn seed_basic_scaffolds_through_real_fire_loop() {
    let run_dir = tempfile::tempdir().unwrap();
    let policy = seed_from_catalog();
    let accelerator = Accelerator::primitive(
        base_state(run_dir.path()),
        policy,
        ToolRuntime::new(),
        "seed-fire",
    );
    let output = timeout(
        Duration::from_secs(30),
        accelerator.run_with(run_input(run_dir.path(), "fire loop scaffold")),
    )
    .await
    .expect("seed-basic must terminate within budget");

    // @agent 具名槽: 幂等 Set, 文本 = prompts["captain"]。
    assert_eq!(
        text_of_anchor(&output.context, "@agent").as_deref(),
        Some("Captain prompt")
    );
    let agent_count = output
        .context
        .fragments()
        .iter()
        .filter(|cell| cell.anchor.as_deref() == Some("@agent"))
        .count();
    assert_eq!(agent_count, 1, "exactly one @agent slot");

    // @env 具名槽: cwd/platform/run_dir, 无 time 行 (沙箱无时钟)。
    let env_text = text_of_anchor(&output.context, "@env").expect("@env present");
    assert!(env_text.contains("cwd: "), "env text: {env_text}");
    assert!(env_text.contains("platform: "), "env text: {env_text}");
    assert!(env_text.contains("run_dir: "), "env text: {env_text}");
    assert!(
        !env_text.contains("time: "),
        "no clock in sandbox: {env_text}"
    );

    // purpose: 尾插 user 文本。
    let purpose_texts: Vec<String> = output
        .context
        .fragments()
        .iter()
        .filter(|cell| cell.tag == "purpose")
        .map(|cell| cell.content_as_text())
        .collect();
    assert_eq!(purpose_texts, vec!["fire loop scaffold".to_string()]);

    // 资源: 模型激活首个, 工具全部激活。
    assert_eq!(output.resources.active_model, "fast");
    assert_eq!(output.resources.active_tools.len(), 2);
    assert!(output.resources.active_tools.contains("read"));
    assert!(output.resources.active_tools.contains("search"));
}

/// E1 等价: seed-basic 与 captain 在确定性脚手架子集上结构等价。
/// 已知分歧 (沙箱边界, 非表达力缺陷): captain 读进程 CWD 指令文件 →
/// 插 instruction cell (无锚, 等价断言过滤); env 含 time 行 (断言剥离);
/// 工具激活顺序 (HashMap 序, 断言集合)。
#[tokio::test]
async fn seed_basic_matches_captain_scaffold() {
    let mut captain_state = manual_state(resources(), "");
    let captain = Captain::new();
    let terminal = drive_until_terminal(&captain, &mut captain_state).await;
    assert_eq!(terminal, Action::Halt, "captain halts after scaffolding");

    let mut seed_state = manual_state(resources(), "");
    let seed = seed_from_catalog();
    let terminal = drive_until_terminal(seed.as_ref(), &mut seed_state).await;
    assert_eq!(terminal, Action::Done, "seed-basic ends without completion");

    let captain_ctx = &captain_state.run.context;
    let seed_ctx = &seed_state.run.context;

    // 锚定指纹一致: (anchor, role, tag) 序列。
    let anchored = |ctx: &machine::Context| -> Vec<(String, String, String)> {
        ctx.fragments()
            .iter()
            .filter_map(|cell| {
                cell.anchor
                    .clone()
                    .map(|anchor| (anchor, format!("{:?}", cell.role), cell.tag.clone()))
            })
            .collect()
    };
    assert_eq!(anchored(captain_ctx), anchored(seed_ctx));

    // @agent 文本一致 (唯一可逐字节比的内容 cell)。
    assert_eq!(
        text_of_anchor(captain_ctx, "@agent"),
        text_of_anchor(seed_ctx, "@agent")
    );
    assert_eq!(
        text_of_anchor(seed_ctx, "@agent").as_deref(),
        Some("Captain prompt")
    );

    // env 文本剥离 time 行后一致 (沙箱无时钟)。
    let env_text = |ctx: &machine::Context| text_of_anchor(ctx, "@env").unwrap_or_default();
    let strip_time = |text: &str| {
        text.lines()
            .filter(|line| !line.starts_with("time: "))
            .collect::<Vec<_>>()
            .join("\n")
    };
    assert_eq!(
        strip_time(&env_text(seed_ctx)),
        strip_time(&env_text(captain_ctx))
    );

    // 资源终态一致。
    assert_eq!(
        seed_state.run.resources.active_model,
        captain_state.run.resources.active_model
    );
    assert_eq!(
        seed_state.run.resources.active_tools,
        captain_state.run.resources.active_tools
    );
}

/// 运行时错误 → 兜底 Done (不 panic、不触发 completion、不挂起):
/// 函数缺失 / 返回值不可解析为 Action / sleep 不存在 (无 IO 沙箱证据)。
#[tokio::test]
async fn script_runtime_failures_fall_back_to_done() {
    for source in [
        "fn decide(view) { return missing_fn(); }", // function not found
        "fn decide(view) { return 42; }",           // unparseable action → from_dynamic 失败
        "fn decide(view) { sleep(1); return done(); }", // sleep 未注册 → not found
    ] {
        let policy = ScriptedPolicy::compile("broken", source, "captain")
            .expect("sources above are syntactically valid");
        let mut state = manual_state(resources(), "");
        let terminal = drive_until_terminal(&policy, &mut state).await;
        assert_eq!(
            terminal,
            Action::Done,
            "runtime failure must map to Done for: {source}"
        );
        assert_eq!(state.frame.step, 0, "fallback fires on the first step");
    }
}

/// 只读 view 透传: purpose / step / directory 都能被脚本读到。
#[tokio::test]
async fn script_view_carries_step_purpose_and_directory() {
    let source = r#"
        fn has_tag(view, tag) {
            for row in view.directory {
                if row.tag == tag {
                    return true;
                }
            }
            false
        }
        fn decide(view) {
            if view.purpose != "" && !has_tag(view, "mirror") {
                return insert_end(view.purpose, "user", "mirror");
            }
            if view.step == 0 {
                return insert_end("first step", "user", "probe");
            }
            done()
        }
    "#;
    let policy = ScriptedPolicy::compile("view-probe", source, "captain").unwrap();

    // purpose 透传 (step 0 插入 mirror, step 1 结束)。
    let mut state = manual_state(resources(), "echo this purpose");
    assert_eq!(
        drive_until_terminal(&policy, &mut state).await,
        Action::Done
    );
    let mirror: Vec<String> = state
        .run
        .context
        .fragments()
        .iter()
        .filter(|cell| cell.tag == "mirror")
        .map(|cell| cell.content_as_text())
        .collect();
    assert_eq!(mirror, vec!["echo this purpose".to_string()]);

    // step 分支 + directory 扫描 (空 purpose 走 probe)。
    let mut state = manual_state(resources(), "");
    assert_eq!(
        drive_until_terminal(&policy, &mut state).await,
        Action::Done
    );
    let probe: Vec<String> = state
        .run
        .context
        .fragments()
        .iter()
        .filter(|cell| cell.tag == "probe")
        .map(|cell| cell.content_as_text())
        .collect();
    assert_eq!(probe, vec!["first step".to_string()]);
}

/// 确定性: 同资源同 purpose 两次运行 → 逐 cell 完全一致
/// (含 env 文本 — 无时钟无 run_dir 时沙箱内零不确定源)。
#[tokio::test]
async fn seed_basic_runs_are_deterministic() {
    let mut first = manual_state(resources(), "determinism");
    let seed = seed_from_catalog();
    drive_until_terminal(seed.as_ref(), &mut first).await;

    let mut second = manual_state(resources(), "determinism");
    let seed = seed_from_catalog();
    drive_until_terminal(seed.as_ref(), &mut second).await;

    assert_eq!(snapshot(&first.run.context), snapshot(&second.run.context));
}
