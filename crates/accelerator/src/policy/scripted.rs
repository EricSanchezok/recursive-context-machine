//! ScriptedPolicy — Rhai 沙箱策略 (MoEH v5, R4-min 后半段)。
//!
//! 策略以纯文本脚本资源存在 (registry 的可演化单元): 构造时编译一次
//! (load-time fail fast), 每步 `decide(view)` 调用脚本内的同名函数,
//! 返回值经 serde 桥转换为 [`machine::Action`]。
//!
//! 沙箱边界 (构造保证):
//! - [`Engine::new_raw`] 空引擎: 无 IO/网络/时钟/模块解析器/print 回调;
//! - 只注册纯函数小包 (数组/字符串/映射/迭代/逻辑/算术), 无 time/lang_core;
//! - `eval` 关键字 parse 期硬拒 ([`Engine::disable_symbol`]);
//! - 引擎级护栏: operations/call-levels/expr-depths/string/array/map 上限;
//! - 脚本作用面 = 注册的 Action 构造器 (done/halt/model/activate/deactivate/
//!   set/insert_end/delete_where/move_after/tool), 输出空间有界。
//!
//! 错误约定: 编译错误 → `compile` 返回 Err (load-time 暴露);
//! 运行时错误 (函数缺失/类型错/护栏触发/反序列化失败) → `tracing::warn!`
//! + [`Action::Done`] (零成本终态, 与 react "permanent → done" 惯例一致,
//!   不触发 completion)。步数硬护栏: step ≥ [`MAX_DECIDE_STEPS`] 直接 Done。

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use machine::edit::{CellPredicate, ContentSpec, EditOp, Position, Selector};
use machine::{Action, Policy, PolicyView, Role};
use rhai::packages::Package;
use rhai::packages::{
    ArithmeticPackage, BasicArrayPackage, BasicIteratorPackage, BasicMapPackage,
    BasicStringPackage, LogicPackage, MoreStringPackage,
};
use rhai::serde::{from_dynamic, to_dynamic};
use rhai::{AST, Dynamic, Engine, Map, Scope};
use tracing::warn;

/// seed-basic 种子脚本: captain 确定性脚手架 (E1 表达力等价基准)。
pub const SEED_BASIC_SOURCE: &str = include_str!("seed_basic.rhai");

/// 步数硬护栏: 脚本策略无法自我终止 (死循环/反复 halt) 时的最后防线。
/// fire loop 本身无 step cap, 此护栏是唯一的无成本退出路径。
const MAX_DECIDE_STEPS: u64 = 4096;

/// 运行时错误后记录 warning 的上限 (防刷屏)。
const MAX_ERROR_WARNINGS: u64 = 8;

/// 引擎级护栏参数: 宽松到不挡合法策略, 紧到挡住失控脚本。
const MAX_OPERATIONS: u64 = 1_000_000;
const MAX_CALL_LEVELS: usize = 64;
const MAX_EXPR_DEPTHS: (usize, usize) = (64, 32);
const MAX_STRING_SIZE: usize = 1 << 20; // 1 MiB bytes
const MAX_ARRAY_SIZE: usize = 65_536;
const MAX_MAP_SIZE: usize = 65_536;

/// Rhai 沙箱策略。
///
/// Clone 廉价 (Arc 共享 Engine/AST/计数); 每次 decide 用全新空 Scope +
/// view map 参数调用 `decide(view)`, 无跨步可变状态 —— 同 obs 同动作,
/// 确定性可回放。
#[derive(Clone, Debug)]
pub struct ScriptedPolicy {
    name: String,
    prompt_key: String,
    engine: Arc<Engine>,
    ast: Arc<AST>,
    error_warnings: Arc<std::sync::atomic::AtomicU64>,
}

impl ScriptedPolicy {
    /// 编译脚本源码为策略。编译错误即时返回 (load-time fail fast)。
    ///
    /// `prompt_key` 决定 `view.prompt` 从 `resources.prompts` 取哪个
    /// 提示文本 (默认 "captain"; 与 captain 的 agent.rs 语义对齐)。
    pub fn compile(
        name: impl Into<String>,
        source: &str,
        prompt_key: impl Into<String>,
    ) -> Result<Self, String> {
        let name = name.into();
        let engine = Self::new_policy_engine();
        let ast = engine
            .compile(source)
            .map_err(|error| format!("policy {name} compile error: {error}"))?;
        Ok(Self {
            name,
            prompt_key: prompt_key.into(),
            engine: Arc::new(engine),
            ast: Arc::new(ast),
            error_warnings: Arc::new(std::sync::atomic::AtomicU64::new(0)),
        })
    }

    /// 确定性沙箱引擎: new_raw + 纯函数小包 + 护栏 + eval 硬拒。
    fn new_policy_engine() -> Engine {
        let mut engine = Engine::new_raw();
        engine
            .set_max_operations(MAX_OPERATIONS)
            .set_max_call_levels(MAX_CALL_LEVELS)
            .set_max_expr_depths(MAX_EXPR_DEPTHS.0, MAX_EXPR_DEPTHS.1)
            .set_max_string_size(MAX_STRING_SIZE)
            .set_max_array_size(MAX_ARRAY_SIZE)
            .set_max_map_size(MAX_MAP_SIZE);
        // eval 是关键字级解释器后门 (与注册函数无关), parse 期硬拒。
        engine.disable_symbol("eval");
        // 纯函数小包: 数组/映射/字符串/for-in 迭代/逻辑/算术。
        // 刻意排除: BasicTimePackage (时钟)、LanguageCorePackage (sleep/exit/
        // parse_json)、BasicBlobPackage/BitFieldPackage/BasicFnPackage。
        for module in [
            BasicArrayPackage::new().as_shared_module(),
            BasicMapPackage::new().as_shared_module(),
            BasicStringPackage::new().as_shared_module(),
            MoreStringPackage::new().as_shared_module(),
            BasicIteratorPackage::new().as_shared_module(),
            LogicPackage::new().as_shared_module(),
            ArithmeticPackage::new().as_shared_module(),
        ] {
            engine.register_global_module(module);
        }
        Self::register_action_builders(&mut engine);
        engine
    }

    /// 白名单 Action 构造器 (脚本对机器的唯一作用面)。
    fn register_action_builders(engine: &mut Engine) {
        engine.register_fn("done", || unit_action(Action::Done));
        engine.register_fn("halt", || unit_action(Action::Halt));
        engine.register_fn("model", |name: &str| {
            unit_action(Action::Model(name.to_string()))
        });
        engine.register_fn("activate", |name: &str| {
            unit_action(Action::Activate(name.to_string()))
        });
        engine.register_fn("deactivate", |name: &str| {
            unit_action(Action::Deactivate(name.to_string()))
        });
        engine.register_fn("set", set_fn);
        engine.register_fn("insert_end", insert_end_fn);
        engine.register_fn("delete_where", delete_where_fn);
        engine.register_fn("move_after", move_after_fn);
        engine.register_fn("tool", tool_fn);
    }

    /// 单次决策: 组装 view map → 调 `decide(view)` → serde 转 Action。
    fn decide_action(&self, view: PolicyView<'_>) -> Action {
        if view.step >= MAX_DECIDE_STEPS {
            self.warn_once(format!(
                "policy {}: step {} exceeds hard cap {}, ending run",
                self.name, view.step, MAX_DECIDE_STEPS
            ));
            return Action::Done;
        }
        if view.status == machine::MachineStatus::Done {
            return Action::Done;
        }

        let view_map = policy_view_map(view, &self.prompt_key);
        let mut scope = Scope::new();
        let result: Result<Dynamic, _> =
            self.engine
                .call_fn(&mut scope, &self.ast, "decide", (view_map,));

        match result {
            Ok(decision) => match from_dynamic::<Action>(&decision) {
                Ok(action) => action,
                Err(error) => {
                    self.warn_once(format!(
                        "policy {}: decide() returned an unparseable action: {error}",
                        self.name
                    ));
                    Action::Done
                }
            },
            Err(error) => {
                self.warn_once(format!(
                    "policy {}: decide() failed at runtime: {error}",
                    self.name
                ));
                Action::Done
            }
        }
    }

    fn warn_once(&self, message: String) {
        let already = self
            .error_warnings
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if already < MAX_ERROR_WARNINGS {
            warn!("{message}");
        }
    }
}

impl Policy for ScriptedPolicy {
    fn clone_box(&self) -> Box<dyn Policy> {
        Box::new(self.clone())
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn decide<'a>(
        &'a self,
        view: PolicyView<'a>,
    ) -> Pin<Box<dyn Future<Output = Action> + Send + 'a>> {
        let action = self.decide_action(view);
        Box::pin(async move { action })
    }
}

/// 把脚本可见的只读视图组装为 Rhai map (全纯数据, 确定性)。
fn policy_view_map(view: PolicyView<'_>, prompt_key: &str) -> Map {
    let run = view.run;
    let mut map = Map::new();
    map.insert("step".into(), Dynamic::from(view.step as i64));
    map.insert(
        "status".into(),
        Dynamic::from(if view.status == machine::MachineStatus::Done {
            "done"
        } else {
            "running"
        }),
    );
    map.insert("purpose".into(), Dynamic::from(run.purpose.text.clone()));
    map.insert(
        "prompt".into(),
        Dynamic::from(
            run.resources
                .prompts
                .get(prompt_key)
                .cloned()
                .unwrap_or_default(),
        ),
    );
    map.insert(
        "cwd".into(),
        Dynamic::from(run.environment.cwd.display().to_string()),
    );
    map.insert(
        "platform".into(),
        Dynamic::from(run.environment.platform.clone()),
    );
    map.insert(
        "run_dir".into(),
        Dynamic::from(
            run.environment
                .run_dir
                .as_ref()
                .map(|path| path.display().to_string())
                .unwrap_or_default(),
        ),
    );

    let directory = view
        .obs
        .context_directory
        .iter()
        .map(|row| Dynamic::from(directory_row_map(row)))
        .collect::<Vec<_>>();
    map.insert("directory".into(), Dynamic::from(directory));
    map.insert(
        "directory_total".into(),
        Dynamic::from(view.obs.context_directory_total as i64),
    );
    map.insert("inbox_depth".into(), Dynamic::from(view.inbox.len() as i64));

    map.insert(
        "active_model".into(),
        Dynamic::from(run.resources.active_model.clone()),
    );
    let mut active_tools: Vec<String> = run.resources.active_tools.iter().cloned().collect();
    active_tools.sort();
    map.insert(
        "active_tools".into(),
        Dynamic::from(
            active_tools
                .into_iter()
                .map(Dynamic::from)
                .collect::<Vec<_>>(),
        ),
    );
    map.insert(
        "model_order".into(),
        Dynamic::from(
            run.resources
                .model_order
                .iter()
                .cloned()
                .map(Dynamic::from)
                .collect::<Vec<_>>(),
        ),
    );
    let mut tool_names: Vec<String> = run.resources.tool_definitions.keys().cloned().collect();
    tool_names.sort();
    map.insert(
        "tool_names".into(),
        Dynamic::from(
            tool_names
                .into_iter()
                .map(Dynamic::from)
                .collect::<Vec<_>>(),
        ),
    );

    // 预算四维 + 上次实际输入 (-1 = 无记录)。
    let budget = &view.obs.budget;
    let mut budget_map = Map::new();
    budget_map.insert(
        "context_limit".into(),
        Dynamic::from(budget.context_limit as i64),
    );
    budget_map.insert(
        "estimated_input".into(),
        Dynamic::from(budget.estimated_input as i64),
    );
    budget_map.insert(
        "soft_threshold".into(),
        Dynamic::from(budget.soft_threshold as i64),
    );
    budget_map.insert("headroom".into(), Dynamic::from(budget.headroom as i64));
    budget_map.insert(
        "last_actual_input".into(),
        Dynamic::from(budget.last_actual_input.map_or(-1, |v| v as i64)),
    );
    map.insert("budget".into(), Dynamic::from(budget_map));
    map
}

fn directory_row_map(row: &machine::obs::CellDirEntry) -> Map {
    let mut map = Map::new();
    map.insert("id".into(), Dynamic::from(row.id as i64));
    map.insert(
        "anchor".into(),
        Dynamic::from(row.anchor.clone().unwrap_or_default()),
    );
    map.insert("role".into(), Dynamic::from(row.role.clone()));
    map.insert("kind".into(), Dynamic::from(row.kind.clone()));
    map.insert("tag".into(), Dynamic::from(row.tag.clone()));
    map.insert("bytes".into(), Dynamic::from(row.bytes as i64));
    map.insert(
        "created_step".into(),
        Dynamic::from(row.created_step as i64),
    );
    map.insert(
        "last_seen_step".into(),
        Dynamic::from(row.last_seen_step as i64),
    );
    map.insert("preview".into(), Dynamic::from(row.preview.clone()));
    map
}

/// Action → Dynamic。构造器产物只经 to_dynamic (结构固定, 不应失败);
/// 兜底 unit 会在 from_dynamic 阶段落入运行时错误路径 → Done。
fn unit_action(action: Action) -> Dynamic {
    to_dynamic(action).unwrap_or_else(|_| Dynamic::from(()))
}

fn parse_role(role: &str) -> Option<Role> {
    match role.to_ascii_lowercase().as_str() {
        "system" => Some(Role::System),
        "user" => Some(Role::User),
        "assistant" => Some(Role::Assistant),
        "tool" => Some(Role::Tool),
        _ => None,
    }
}

/// `set(anchor, text, role, tag)` — 具名槽幂等 Set (tag 空串 → None)。
fn set_fn(anchor: &str, text: &str, role: &str, tag: &str) -> Dynamic {
    let Some(role) = parse_role(role) else {
        return Dynamic::from(());
    };
    unit_action(Action::Edit {
        ops: vec![EditOp::Set {
            anchor: anchor.to_string(),
            content: ContentSpec::Literal {
                text: text.to_string(),
                role,
                tag: (!tag.is_empty()).then(|| tag.to_string()),
            },
        }],
        because: None,
    })
}

/// `insert_end(text, role, tag)` — 文档尾 Insert (tag 空串 → None)。
fn insert_end_fn(text: &str, role: &str, tag: &str) -> Dynamic {
    let Some(role) = parse_role(role) else {
        return Dynamic::from(());
    };
    unit_action(Action::Edit {
        ops: vec![EditOp::Insert {
            position: Position::End,
            content: ContentSpec::Literal {
                text: text.to_string(),
                role,
                tag: (!tag.is_empty()).then(|| tag.to_string()),
            },
            anchor: None,
        }],
        because: None,
    })
}

/// `delete_where(pred)` — 谓词删除 (pred 为 CellPredicate wire 形状 map)。
fn delete_where_fn(pred: Map) -> Dynamic {
    let Ok(predicate) = from_dynamic::<CellPredicate>(&Dynamic::from(pred)) else {
        return Dynamic::from(());
    };
    unit_action(Action::Edit {
        ops: vec![EditOp::Delete {
            selector: Selector::Where(predicate),
        }],
        because: None,
    })
}

/// `move_after(anchor, after)` — 把具名槽移动到另一锚后。
fn move_after_fn(anchor: &str, after: &str) -> Dynamic {
    unit_action(Action::Edit {
        ops: vec![EditOp::Move {
            anchor: anchor.to_string(),
            after: Position::Anchor(after.to_string()),
        }],
        because: None,
    })
}

/// `tool(name, args)` — 调用注册工具 (args 为 JSON 形状 map)。
fn tool_fn(name: &str, args: Map) -> Dynamic {
    let args_value: Result<serde_json::Value, _> = from_dynamic(&Dynamic::from(args));
    let args_value = args_value.unwrap_or(serde_json::Value::Null);
    unit_action(Action::Tool {
        name: name.to_string(),
        args: args_value,
        because: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compile_rejects_invalid_syntax_with_position() {
        let error = ScriptedPolicy::compile("bad", "fn decide( {", "captain")
            .expect_err("syntax error must fail compilation");
        assert!(error.contains("compile error"), "got: {error}");
        assert!(error.contains('('), "parse errors carry position: {error}");
    }

    #[test]
    fn compile_rejects_eval_keyword() {
        let error = ScriptedPolicy::compile(
            "evil",
            r#"fn decide(view) { eval("1"); return done(); }"#,
            "captain",
        )
        .expect_err("eval must be rejected at parse time");
        assert!(
            error.contains("compile error") || error.contains("eval"),
            "got: {error}"
        );
    }
}
