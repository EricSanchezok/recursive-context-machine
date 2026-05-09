//! RICA Core — Recursive Intelligence Creation Accelerator
//!
//! 核心设计：Rica 是一个原语（primitive）。
//!
//! - 单个 Rica：一次 LLM 调用，可能包含多轮工具调用
//! - 组合 Rica：pipeline、parallel、ensemble —— 也实现同样的 `Rica` trait
//! - 递归： Rica 可以包含 Rica，接口不变
//!
//! 关键问题：Rica 本身是不是一个工具节点？
//!   → 不是。Rica 是**编排单元**，工具是 Rica 内部的执行手段。
//!   → 但 Rica 可以被**包装成工具**，供其他 Rica 调用。这是递归的入口。

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

// ============================================================================
// 核心 trait
// ============================================================================

/// 用户意图 —— 想做什么，而不是怎么做
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intent {
    pub prompt: String,
    /// 附加上下文，如文件内容、历史对话等
    pub context: HashMap<String, Value>,
}

impl Intent {
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
            context: HashMap::new(),
        }
    }

    pub fn with_context(mut self, key: impl Into<String>, value: Value) -> Self {
        self.context.insert(key.into(), value);
        self
    }
}

/// 执行上下文 —— 当前 Rica 的运行时环境
#[derive(Debug, Clone, Default)]
pub struct Context {
    /// 当前可用的工具
    pub tools: Vec<ToolBinding>,
    /// 父级 Rica 的上下文（递归时传递）
    pub parent: Option<Box<Context>>,
    /// 运行期状态（如已调用次数、预算等）
    pub state: HashMap<String, Value>,
}

impl Context {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_tools(mut self, tools: Vec<ToolBinding>) -> Self {
        self.tools = tools;
        self
    }

    pub fn with_state(mut self, key: impl Into<String>, value: Value) -> Self {
        self.state.insert(key.into(), value);
        self
    }

    /// 创建子上下文（递归调用时使用）
    pub fn child(&self) -> Self {
        Self {
            tools: self.tools.clone(),
            parent: Some(Box::new(self.clone())),
            state: HashMap::new(),
        }
    }
}

/// 工具绑定 —— 动态链接的接口
///
/// 工具不是注册到全局 registry，而是在运行时通过 `ToolBinding` 动态链接。
/// 工具的实现可以是：
/// - 本地函数（Fn）
/// - 子 Rica（另一个 Rica 实例）
/// - 外部服务（HTTP, gRPC, WASM）
#[derive(Clone)]
pub struct ToolBinding {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
    /// 工具的实际执行体
    pub execute: Arc<
        dyn Fn(Value, Context) -> Pin<Box<dyn Future<Output = Result<Value, String>> + Send>>
            + Send
            + Sync,
    >,
}

impl std::fmt::Debug for ToolBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ToolBinding")
            .field("name", &self.name)
            .field("description", &self.description)
            .field("input_schema", &self.input_schema)
            .finish_non_exhaustive()
    }
}

/// 单次工具调用的记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub tool_name: String,
    pub input: Value,
    pub output: Result<Value, String>,
    pub duration_ms: u64,
}

/// Rica 的输出
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Output {
    /// LLM 的文本回复
    pub text: String,
    /// 是否调用了工具
    pub tool_calls: Vec<ToolCall>,
    /// 元数据（token 用量、模型信息等）
    pub metadata: HashMap<String, Value>,
}

/// 事件轨迹 —— 用于调试、审计、回放
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EventTrail {
    pub events: Vec<Event>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    /// Rica 开始执行
    Start { intent: Intent },
    /// 调用 LLM
    LlmCall { prompt: String, response: String },
    /// 调用工具
    ToolCall(ToolCall),
    /// 递归调用子 Rica
    RecursiveCall { name: String, output: Output },
    /// Rica 完成
    Complete { output: Output },
}

/// Rica 原语 —— 所有 Rica（单细胞、组合体）都实现这个 trait
///
/// 单个 Rica 和组合 Rica 都通过同样的接口调用：
/// ```ignore
/// let rica = LlmRica::new("gpt-4o");
/// let (output, trail) = rica.accelerate(intent, &ctx).await;
///
/// let pipeline = Pipeline::new("pipe", vec![Arc::new(rica_a), Arc::new(rica_b)]);
/// let (output, trail) = pipeline.accelerate(intent, &ctx).await;
/// ```
pub trait Rica: Send + Sync {
    fn accelerate<'a>(
        &'a self,
        intent: Intent,
        ctx: &'a Context,
    ) -> Pin<Box<dyn Future<Output = (Output, EventTrail)> + Send + 'a>>;
}

// ============================================================================
// 组合 Rica —— Pipeline（串行）
// ============================================================================

/// 串行管道：前一个 Rica 的输出作为后一个的输入
pub struct Pipeline {
    pub stages: Vec<Arc<dyn Rica>>,
    pub name: String,
}

impl Pipeline {
    pub fn new(name: impl Into<String>, stages: Vec<Arc<dyn Rica>>) -> Self {
        Self {
            name: name.into(),
            stages,
        }
    }
}

impl Rica for Pipeline {
    fn accelerate<'a>(
        &'a self,
        intent: Intent,
        ctx: &'a Context,
    ) -> Pin<Box<dyn Future<Output = (Output, EventTrail)> + Send + 'a>> {
        Box::pin(async move {
            let mut trail = EventTrail::default();
            let mut current_intent = intent;
            let mut last_output = Output {
                text: "Empty pipeline".into(),
                tool_calls: vec![],
                metadata: HashMap::new(),
            };

            for (i, stage) in self.stages.iter().enumerate() {
                let (output, stage_trail) = stage.accelerate(current_intent, ctx).await;

                trail.events.push(Event::RecursiveCall {
                    name: format!("{}[stage_{}]", self.name, i),
                    output: output.clone(),
                });
                trail.events.extend(stage_trail.events);

                last_output = output.clone();
                current_intent = Intent::new(&output.text).with_context(
                    "previous_output",
                    serde_json::to_value(&output).unwrap_or_default(),
                );
            }

            trail.events.push(Event::Complete {
                output: last_output.clone(),
            });

            (last_output, trail)
        })
    }
}

// ============================================================================
// 组合 Rica —— Parallel（并行）
// ============================================================================

/// 并行执行多个 Rica，然后聚合结果
pub struct Parallel {
    pub branches: Vec<Arc<dyn Rica>>,
    pub name: String,
}

impl Parallel {
    pub fn new(name: impl Into<String>, branches: Vec<Arc<dyn Rica>>) -> Self {
        Self {
            name: name.into(),
            branches,
        }
    }
}

impl Rica for Parallel {
    fn accelerate<'a>(
        &'a self,
        intent: Intent,
        ctx: &'a Context,
    ) -> Pin<Box<dyn Future<Output = (Output, EventTrail)> + Send + 'a>> {
        Box::pin(async move {
            let mut trail = EventTrail::default();
            trail.events.push(Event::Start {
                intent: intent.clone(),
            });

            let futures: Vec<_> = self
                .branches
                .iter()
                .enumerate()
                .map(|(i, branch)| {
                    let branch_intent = Intent::new(format!("[branch_{}] {}", i, intent.prompt))
                        .with_context("branch_index", serde_json::json!(i));
                    branch.accelerate(branch_intent, ctx)
                })
                .collect();

            let results: Vec<(Output, EventTrail)> = futures::future::join_all(futures).await;

            let mut all_texts = Vec::new();
            let mut all_tool_calls = Vec::new();

            for (i, (output, branch_trail)) in results.into_iter().enumerate() {
                trail.events.push(Event::RecursiveCall {
                    name: format!("{}[branch_{}]", self.name, i),
                    output: output.clone(),
                });
                trail.events.extend(branch_trail.events);
                all_texts.push(output.text);
                all_tool_calls.extend(output.tool_calls);
            }

            let output = Output {
                text: all_texts.join("\n---\n"),
                tool_calls: all_tool_calls,
                metadata: HashMap::from([(
                    "parallel_branches".into(),
                    serde_json::json!(self.branches.len()),
                )]),
            };

            trail.events.push(Event::Complete {
                output: output.clone(),
            });

            (output, trail)
        })
    }
}

// ============================================================================
// 组合 Rica —— Ensemble（投票/聚合）
// ============================================================================

/// 多个 Rica 执行同一任务，然后投票选出最佳结果
pub struct Ensemble {
    pub voters: Vec<Arc<dyn Rica>>,
    pub name: String,
    /// 聚合策略：简单拼接、投票、LLM 再总结等
    pub aggregator: Arc<dyn Fn(Vec<Output>) -> Output + Send + Sync>,
}

impl Ensemble {
    pub fn new(
        name: impl Into<String>,
        voters: Vec<Arc<dyn Rica>>,
        aggregator: Arc<dyn Fn(Vec<Output>) -> Output + Send + Sync>,
    ) -> Self {
        Self {
            name: name.into(),
            voters,
            aggregator,
        }
    }
}

impl Rica for Ensemble {
    fn accelerate<'a>(
        &'a self,
        intent: Intent,
        ctx: &'a Context,
    ) -> Pin<Box<dyn Future<Output = (Output, EventTrail)> + Send + 'a>> {
        Box::pin(async move {
            let mut trail = EventTrail::default();
            trail.events.push(Event::Start {
                intent: intent.clone(),
            });

            let futures: Vec<_> = self
                .voters
                .iter()
                .enumerate()
                .map(|(i, voter)| {
                    let voter_intent = Intent::new(format!("[voter_{}] {}", i, intent.prompt))
                        .with_context("voter_index", serde_json::json!(i));
                    voter.accelerate(voter_intent, ctx)
                })
                .collect();

            let results: Vec<(Output, EventTrail)> = futures::future::join_all(futures).await;
            let mut outputs = Vec::new();

            for (i, (output, voter_trail)) in results.into_iter().enumerate() {
                trail.events.push(Event::RecursiveCall {
                    name: format!("{}[voter_{}]", self.name, i),
                    output: output.clone(),
                });
                trail.events.extend(voter_trail.events);
                outputs.push(output);
            }

            let output = (self.aggregator)(outputs);

            trail.events.push(Event::Complete {
                output: output.clone(),
            });

            (output, trail)
        })
    }
}

// ============================================================================
// 工具 → Rica 包装器：让 Rica 可以被当作工具调用
// ============================================================================

/// 把一个 Rica 实例包装成 ToolBinding，供其他 Rica 调用
///
/// 这是递归的关键入口：Rica 不是工具，但可以被包装成工具。
pub fn rica_as_tool(rica: Arc<dyn Rica>, name: String, description: String) -> ToolBinding {
    ToolBinding {
        name: name.clone(),
        description,
        input_schema: serde_json::json!({
            "type": "object",
            "properties": {
                "prompt": { "type": "string", "description": "任务描述" }
            },
            "required": ["prompt"]
        }),
        execute: Arc::new(move |input, ctx| {
            let rica = rica.clone();
            Box::pin(async move {
                let prompt = input
                    .get("prompt")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();

                let intent = Intent::new(prompt);
                let child_ctx = ctx.child();

                let (output, _trail) = rica.accelerate(intent, &child_ctx).await;

                Ok(serde_json::json!({
                    "text": output.text,
                    "tool_calls": output.tool_calls.len(),
                }))
            })
        }),
    }
}

// ============================================================================
// 占位：基于 rig 的 LlmRica 实现
// ============================================================================

/// 基于 LLM 的 Rica 实现（待实现）
///
/// 这是"单细胞" Rica —— 直接调用 LLM，可能触发工具调用。
/// 工具调用通过 rig 的 function calling 实现。
pub struct LlmRica {
    // TODO: rig client
    pub model: String,
}

impl LlmRica {
    pub fn new(model: impl Into<String>) -> Self {
        Self {
            model: model.into(),
        }
    }
}

impl Rica for LlmRica {
    fn accelerate<'a>(
        &'a self,
        intent: Intent,
        _ctx: &'a Context,
    ) -> Pin<Box<dyn Future<Output = (Output, EventTrail)> + Send + 'a>> {
        Box::pin(async move {
            // TODO: 集成 rig，实现真正的 LLM 调用
            let output = Output {
                text: format!("[LlmRica:{}] {}", self.model, intent.prompt),
                tool_calls: vec![],
                metadata: HashMap::new(),
            };
            let trail = EventTrail {
                events: vec![
                    Event::Start { intent },
                    Event::Complete {
                        output: output.clone(),
                    },
                ],
            };
            (output, trail)
        })
    }
}
