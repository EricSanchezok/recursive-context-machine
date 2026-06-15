下面给出一个我认为比较完整、可以逐步落地的 Survey Engine / Auto Research Survey 框架设计。核心判断是：Survey 不是“搜索 + 总结”，而是“研究问题建模 + 文献宇宙构建 + 证据约束理解 + 比较性综合 + 质量门控”的流水线。你现在的 SEARCH → EXPAND → COVERAGE 是正确骨架，但还需要补上两个关键层：UNDERSTAND / RELATION 和 SYNTHESIS / VERIFY。

⸻

0. 总体目标：不是生成一篇“像综述的文章”，而是构建一个可审计的研究地图

SurveyBench、DeepSurvey-Bench、DeepScholar-Bench 的共同结论可以压缩成一句话：

当前 LLM Survey 的主要失败不是语言能力不足，而是 文献宇宙不完整、文献理解不深、跨文献关系建模不足、引用-论断约束不足。

SurveyBench 发现 LLM 综述在内容质量上平均低于人工综述约 21%，且在细节、跨概念关联、独立综合、新主题时效性上失败明显。 ￼ DeepSurvey-Bench 强调，流畅度和结构质量不能代表学术价值，真正要看信息价值、学术交流价值和研究引导价值。 ￼ DeepScholar-Bench 则把问题进一步拆成知识综合、检索质量和可验证性，并指出当前系统离饱和还很远，检索和综合都是瓶颈。 ￼

所以我建议 Holos-research 的 Survey Engine 不要直接以“写 survey”为最终内部目标，而是分成两个产物：

1. Research Map：一个结构化、可查询、可验证的领域知识图谱 / 文献地图。
2. Survey Draft：从 Research Map 中投影出来的一种叙事版本。

这很重要。因为如果直接写 survey，系统很容易变成“边搜边写边幻觉”；如果先构建 Research Map，后续写作、查错、补文献、改结构都可以基于同一个中间表示迭代。

⸻

1. 总体架构：SEARCH → EXPAND → UNDERSTAND → RELATION → COVERAGE → SYNTHESIS → VERIFY

可以设计成如下流水线：

Stage	目标	主要解决的问题	输出
0. Anchor Modeling	明确这篇 survey 到底综述什么	防止目标漂移、范围模糊	SurveySpec
1. Search	初始召回候选文献	解决关键词检索浅、术语覆盖不足	CandidatePool
2. Expand	从种子文献向引用图和相似空间扩展	找到关键词搜不到的重要论文	ExpandedPool
3. Understand	对单篇论文做结构化理解	解决只读摘要导致的 overclaim	PaperCard / EvidenceCard
4. Relation	建立论文之间的方法、任务、数据集、指标、假设关系	解决“只罗列不比较”	LiteratureGraph
5. Coverage	判断是否覆盖了主要主题、方法族、benchmark、争议与 gap	解决召回不足和结构盲区	CoverageReport
6. Synthesis	生成 taxonomy、技术演进线、比较表、gap 分析、写作大纲	解决综合深度不足	SurveyPlan
7. Verify	逐条检查 claims 与引用证据是否对齐	解决可信度不足	VerifiedSurvey

更具体地说，你原来的三段式可以扩展成：

ANCHOR
  ↓
SEARCH → EXPAND
  ↓
UNDERSTAND
  ↓
RELATION GRAPH
  ↓
COVERAGE / GAP / BENCHMARK CHECK
  ↓
SYNTHESIS PLAN
  ↓
EVIDENCE-CONSTRAINED WRITING
  ↓
CLAIM-CITATION VERIFICATION

最近的 DeepSurvey 系统也走向了类似方向：它强调从 full-text paper 中抽取结构化 keynotes、通过聚类和比较分析建模跨论文关系、结合 citation-graph expansion 和 evidence-constrained citation assignment 提升深度与引用可靠性。 ￼ 这说明你提出的方向不是“堆工程”，而是当前自动综述系统正在收敛到的核心路线。

⸻

2. Stage 0：Anchor Modeling —— 先定义“这篇 survey 的研究对象”

很多自动综述失败的第一步不是检索，而是 不知道自己在综述什么。DeepSurvey-Bench 特别指出，自动综述在目标清晰度、数据集与指标覆盖等信息价值维度上受损明显。 ￼

所以在 Search 之前，应先生成一个结构化的 SurveySpec。

2.1 SurveySpec 数据结构

SurveySpec:
  title_candidate: "..."
  reader_need:
    reader_type: "new researcher / domain expert / engineer / reviewer"
    expected_depth: "introductory / technical / critical / exhaustive"
    use_case: "learn field / write related work / find gaps / reproduce methods"
  anchor_question:
    main_question: "What problem is this survey about?"
    sub_questions:
      - "What are the major task formulations?"
      - "What method families exist?"
      - "What datasets and metrics are used?"
      - "What are unresolved limitations?"
  scope:
    include:
      - task/domain/method keywords
    exclude:
      - adjacent but irrelevant topics
    time_range:
      from: 2018
      to: present
  concept_seed:
    core_terms: []
    synonyms: []
    abbreviations: []
    related_terms: []
    cross_domain_terms: []
  expected_dimensions:
    methods: true
    datasets: true
    metrics: true
    theory: true
    applications: true
    limitations: true
    future_work: true

这个对象有两个作用：

第一，它是检索器的 query generator 输入。

第二，它是后续 coverage checker 的基准。也就是说，系统不是泛泛地问“文献够不够”，而是问：

对于 SurveySpec 中定义的每个 research need，我是否已经有足够证据？

2.2 Anchor Modeling 的关键原则

Survey 的 anchor 不应该只是一个题目，而应该至少包含四层：

层次	问题	例子
Problem anchor	解决什么问题？	Long-context LLM 的 KV cache compression
Method anchor	用什么方法族？	pruning / merging / low-rank / retrieval / spectral compression
Evaluation anchor	怎么证明有效？	perplexity, passkey retrieval, LongBench, latency, memory
Reader anchor	读者想拿走什么？	了解技术路线、复现方法、找创新点

这对应你提到的 reader needs。好的 Survey Engine 应该能先问：

这篇 survey 是为了“入门理解”、为了“找 paper gap”、为了“写 related work”，还是为了“复现 benchmark”？

不同目标会导致完全不同的检索策略和写作结构。

⸻

3. Stage 1：SEARCH —— 从关键词检索升级为“查询程序生成”

你现在设计的 SEARCH 已经有 5 个步骤：查询扩展、多源并行、合并去重、增强、过滤排序。这里我建议进一步把它形式化为 Query Program Generation。

不是让 LLM 生成 5~10 个普通搜索词，而是生成一组有类型、有目的、有覆盖维度的查询程序。

3.1 查询类型设计

对一个 survey topic，至少生成以下几类 query：

Query 类型	目的	示例
Core method query	找直接相关方法论文	"KV cache compression" "large language models"
Mechanism query	找具体机制变体	"attention preserving" "KV cache" merging
Problem query	找同一问题的不同解法	"long context inference" memory reduction transformer
Benchmark query	找数据集、评估协议	"LongBench" "KV cache compression"
Survey query	找已有综述和 taxonomy	"survey" "long context LLM" inference efficiency
Citation seed query	找高被引基础论文	"Transformer" "KV cache" "autoregressive decoding"
Negative / boundary query	确定不该纳入什么	"prompt compression" vs "KV cache compression"
Cross-domain query	找相邻领域术语	"key-value memory compression" "attention"

这样做的好处是：每个 query 都服务于一个 coverage 维度，后续可以知道自己缺的是方法、benchmark、理论、还是应用。

3.2 多源检索不只是 API 多，而是证据类型多

建议多源包括：

Source	价值
arXiv / Semantic Scholar / OpenAlex	论文元数据、引用数、相关论文
Google Scholar 或 SerpAPI	补齐引用传播、灰色文献
ACL Anthology / CVF / NeurIPS / ICML / ICLR / ACM / IEEE	领域权威会议版本
Papers with Code	数据集、指标、排行榜、代码
GitHub	实现细节、是否可复现
Connected Papers / Semantic Scholar graph	引用图扩展
CrossRef / DBLP	规范元数据与 BibTeX

对于 Survey Engine 来说，论文不是唯一资源。很多 benchmark、dataset、metric、code implementation 不会在 abstract 中体现，但对综述的学术价值非常关键。DeepSurvey-Bench 特别把 benchmark 覆盖作为信息价值的一部分，这说明检索器需要主动找 dataset 和 metric，而不是只找方法论文。 ￼

3.3 初筛排序：不应该只按相似度

一个候选论文的分数可以设计为：

PaperScore =
  α * semantic_relevance
+ β * citation_influence
+ γ * recency
+ δ * venue_quality
+ ε * graph_centrality
+ ζ * benchmark_relevance
+ η * diversity_gain
- λ * redundancy
- μ * scope_risk

其中：

* semantic_relevance：与 SurveySpec 的语义相似度。
* citation_influence：引用数、领域内引用、时间归一化引用。
* recency：新主题尤其重要，但不能盲目偏向新论文。
* graph_centrality：在引用图中是否连接多个社区。
* benchmark_relevance：是否提出 dataset / metric / evaluation protocol。
* diversity_gain：是否覆盖了新方法族。
* scope_risk：是否只是名字相似但领域不同。

DeepScholar-Bench 指出当前系统能找到一些相关论文，但遗漏大多数重要论文，并且无法区分高影响力和低影响力来源。 ￼ 因此排序必须显式建模“重要性”，不能只做 embedding similarity。

⸻

4. Stage 2：EXPAND —— 从“搜论文”变成“构造文献宇宙”

SEARCH 得到的只是 seed。真正的文献覆盖主要来自 EXPAND。

4.1 三种扩展方向

扩展方向	作用	风险
Backward citation	找基础论文、理论来源、早期方法	可能太老、太泛
Forward citation	找后续改进、最新趋势、争议	可能噪声大
Semantic neighbor	找术语不同但问题相似的论文	容易跑偏

你可以把 EXPAND 设计成带预算的图搜索：

Input: seed papers S
For step in 1..K:
    for paper in frontier:
        collect references(paper)
        collect citations(paper)
        collect semantic_neighbors(paper)
        score each candidate by:
            relevance + influence + diversity_gain + bridge_value - drift_risk
    keep top-B candidates per cluster
    update frontier

关键是不能无脑扩展。每一轮扩展都要问：

这个新论文是否增加了一个新的方法族、一个关键 benchmark、一个理论来源、一个重要反例，或者一个争议节点？

如果只是“相似但重复”，应该降权。

4.2 Bridge Paper 很重要

我建议引入一个概念：bridge paper。

Bridge paper 是连接两个方法社区的论文。例如它可能同时引用传统方法和新方法，或者把一个领域的技术迁移到另一个领域。它的价值不一定体现在引用数上，但对 survey 的“综合深度”非常关键。

可以定义：

bridge_value(p) =
  number_of_clusters_connected_by_p
+ cross_domain_term_overlap
+ citation_path_betweenness

这直接对应 SurveyBench 中的 F2：LLM 无法跨概念建立类比或联系。要解决 F2，不能只靠写作阶段让 LLM “多做关联”，而是要在文献图里显式找到关联节点。

⸻

5. Stage 3：UNDERSTAND —— 单篇论文必须形成 PaperCard，而不是摘要总结

这是整个框架的核心。你提到“只读标题和摘要容易 overclaim”，这个判断非常关键。

Survey 写作中的可信度问题往往来自：

1. 把论文的 motivation 当成 contribution。
2. 把实验设置外推到整个领域。
3. 忽略 benchmark scope。
4. 忽略 ablation 和 failure case。
5. 把作者 claim 当作已验证事实。

因此必须设计 PaperCard / EvidenceCard。

5.1 PaperCard 数据结构

PaperCard:
  paper_id: "..."
  metadata:
    title:
    authors:
    year:
    venue:
    citation_count:
    url:
  problem:
    task:
    motivation:
    assumptions:
  method:
    core_idea:
    method_family:
    technical_components:
    novelty_claims:
    relation_to_prior_work:
  evidence:
    datasets:
    metrics:
    baselines:
    experimental_setting:
    main_results:
    ablations:
    failure_cases:
  scope:
    what_is_demonstrated:
    what_is_not_demonstrated:
    generalization_risk:
  claims:
    - claim_text:
      claim_type: "method / result / limitation / comparison"
      evidence_span:
      confidence:
  survey_use:
    possible_section:
    comparison_axes:
    gap_relevance:
    citation_role: "foundation / method / benchmark / critique / application"

这里最重要的是 scope：

scope:
  what_is_demonstrated:
  what_is_not_demonstrated:
  generalization_risk:

这可以直接缓解你说的 overclaim。

5.2 EvidenceCard：每个 claim 必须绑定证据

PaperCard 是论文级别，EvidenceCard 是证据级别。

EvidenceCard:
  evidence_id:
  paper_id:
  section:
  quote_or_span:
  normalized_statement:
  evidence_type:
    - definition
    - method_detail
    - theorem
    - experiment_result
    - ablation
    - limitation
    - dataset_description
    - metric_definition
  supports_claims:
    - claim_id
  confidence:

后续写 survey 时，不允许凭空写：

Method A outperforms Method B.

除非 LiteratureGraph 里有对应 EvidenceCard 支持：

claim:
  text: "Method A achieves lower latency than Method B under benchmark X."
  evidence:
    - paper_A_table_2
    - paper_B_table_1
  condition:
    dataset: X
    model_size: Y
    metric: latency

这能把 survey 写作从“语言生成”变成“证据投影”。

5.3 Full-text 分层读取策略

不是所有论文都要全文精读。可以分层：

层级	读取内容	适用论文
L0	title + abstract + metadata	初筛候选
L1	intro + related work + conclusion	判断 scope 和定位
L2	method + experiment + limitation	主要方法论文
L3	full text + tables + appendix + code	核心论文、争议论文、benchmark 论文

这样控制成本。系统可以先生成 importance_score，只有超过阈值的论文进入 L2/L3。

⸻

6. Stage 4：RELATION —— 文献之间的关系比单篇总结更重要

自动 survey 的最大弱点是“会描述，不会比较”。DeepSurvey-Bench 指出学术交流价值是弱项，尤其是深度比较和批判性分析。 ￼ 所以必须显式构建 LiteratureGraph。

6.1 LiteratureGraph 节点类型

Nodes:
  Paper
  Method
  Task
  Dataset
  Metric
  Claim
  Limitation
  Assumption
  Concept
  Benchmark

6.2 边类型

Edges:
  Paper -> proposes -> Method
  Paper -> evaluates_on -> Dataset
  Paper -> uses_metric -> Metric
  Method -> extends -> Method
  Method -> contrasts_with -> Method
  Method -> assumes -> Assumption
  Paper -> reports_limitation -> Limitation
  Claim -> supported_by -> EvidenceCard
  Paper -> cites -> Paper
  Paper -> improves_on -> Paper
  Method -> belongs_to -> MethodFamily

这样你才能问出高质量 survey 需要的问题：

* 哪些方法解决同一个 task？
* 哪些方法用了相同 benchmark？
* 哪些方法只在不同 benchmark 上评估，所以不能直接比较？
* 哪个方法族依赖某个共同假设？
* 哪些 limitation 被多个 paper 反复承认？
* 哪些 dataset/metric 被多数方法使用，哪些只是孤例？

6.3 比较矩阵自动生成

例如针对某个 cluster，生成：

Paper	Method family	Core mechanism	Assumption	Dataset	Metric	Baseline	Strength	Weakness

这个表不是最终展示用的，而是 synthesis 的中间件。它可以强制系统发现：

* 某方法是否没有强 baseline；
* 某论文是否只在 toy dataset 上实验；
* 某些方法是否其实解决的是不同 problem formulation；
* 某个 claimed improvement 是否没有公平比较。

这一步是从“总结型 LLM”转向“研究助理型系统”的关键。

⸻

7. Stage 5：COVERAGE —— 不只是看论文数量，而是看研究结构是否闭合

你提出的 COVERAGE 已经很好：embedding 聚类、anchor 映射、主题邻域论文数量、识别不足主题、扩展查询。我建议进一步拆成五种 coverage。

7.1 五类 Coverage

Coverage 类型	检查问题	失败后动作
Concept coverage	核心概念和术语变体是否覆盖？	query expansion
Method coverage	每个主要方法族是否有代表论文？	expand citation graph
Benchmark coverage	数据集、指标、leaderboard 是否覆盖？	benchmark-specific search
Comparison coverage	每个方法族是否有可比较对象？	找同 benchmark 或同 task 论文
Gap coverage	是否有足够 limitation / future work 证据？	搜 limitation、failure、challenge 论文

其中最容易被忽视的是 Comparison coverage。

一个 cluster 里有 5 篇论文，不代表可比较。如果它们用不同数据集、不同模型规模、不同指标，那么不能直接写“谁优谁劣”。

因此可以定义：

comparison_ready(cluster) = 
  exists at least 2 papers
  with same task
  and overlapping dataset/metric
  and comparable baseline or setting

你提到的“每个主要方法聚类有 ≥2 篇竞争论文”是对的，但还应加上：

它们是否在同一 evaluation frame 下可比？

否则 survey 会产生伪比较。

7.2 Gap Readiness

你提出的 “gap 就绪度” 非常有价值。可以定义：

gap_readiness(topic) =
  limitation_evidence_count
+ contradiction_count
+ benchmark_failure_count
+ unresolved_assumption_count
+ lack_of_standard_eval_score

例如一个 topic 下，如果多篇论文都承认：

* scaling 不好；
* 需要特定数据；
* benchmark 不统一；
* 只在合成数据上有效；
* 理论解释不足；

那么这个 topic 的 gap readiness 高，适合写成“未来方向”。

但如果只是 LLM 自己觉得“未来可以探索 XXX”，而没有论文证据支持，那就是低质量 gap。

7.3 Coverage Report

输出应类似：

CoverageReport:
  overall_status: "insufficient / acceptable / strong"
  missing_concepts:
    - term:
      reason:
      suggested_query:
  weak_clusters:
    - cluster_name:
      paper_count:
      representative_papers:
      missing_dimension: "benchmark / comparison / limitation"
  benchmark_coverage:
    datasets:
    metrics:
    missing:
  comparison_readiness:
    ready_clusters:
    not_ready_clusters:
  gap_readiness:
    strong_gap_topics:
    weak_gap_topics:
  next_actions:
    - search_query:
      target_dimension:
      expected_gain:

这样 COVERAGE 不只是一个分数，而是能驱动下一轮 search / expand。

⸻

8. Stage 6：SYNTHESIS —— 先生成 SurveyPlan，再写正文

SurveyBench 中 F3 说得很准：LLM 综述常常只是把来源重写，而没有独立聚类和总结。 ￼ 所以 synthesis 不应该直接写段落，而应该先做三件事：

1. 生成 taxonomy；
2. 生成 evolution line；
3. 生成 comparison axes。

8.1 Taxonomy 不是按论文聚类，而是按“解释力”聚类

常见失败是 embedding cluster 出来的主题不等于好的 survey taxonomy。

好的 taxonomy 应该满足：

标准	含义
Mutually distinguishable	各类方法有清晰差异
Explanatory	分类能解释为什么方法不同
Comparative	分类支持优缺点比较
Evolutionary	分类能承载技术演进
Reader-useful	读者读完能建立心智地图

例如不要只是：

Section 1: Paper A
Section 2: Paper B
Section 3: Paper C

而应该是：

1. Compression by token selection
2. Compression by representation merging
3. Compression by low-rank approximation
4. Compression by retrieval or eviction policy
5. Hybrid systems with hardware-aware scheduling

这才是 survey。

8.2 自动生成 taxonomy 的方法

可以让系统产生多个候选 taxonomy，然后用评估器打分。

TaxonomyCandidate:
  name:
  principle:
    - by task formulation
    - by method mechanism
    - by model component
    - by evaluation setting
    - by historical stage
  clusters:
    - cluster_name:
      definition:
      included_papers:
      excluded_papers:
      distinguishing_features:
  score:
    coverage:
    separation:
    comparison_value:
    reader_clarity:
    evidence_support:

然后选择得分最高的 taxonomy，或者混合两个 taxonomy：

* 主体按 method mechanism；
* 每一节内部按 evolution；
* 最后用 benchmark 维度横向比较。

8.3 Evolution Line

Survey 不只是分类，还要讲“为什么领域会这样发展”。

可以提取：

EvolutionLine:
  stage:
    - time_range:
      dominant_problem:
      representative_methods:
      bottleneck:
      transition_reason:

例如：

Stage 1: 早期方法关注能不能做。
Stage 2: 后续方法关注效率和可扩展性。
Stage 3: 新方法开始关注真实部署、长尾场景和统一 benchmark。

每个 transition 都必须有 evidence：

* 某个 benchmark 出现；
* 某个方法暴露瓶颈；
* 某类应用需求变化；
* 算力或模型规模变化。

8.4 Comparison Axes

每个 survey 应该有一组比较轴：

ComparisonAxes:
  - problem_formulation
  - input/output assumption
  - model architecture
  - training/inference cost
  - data requirement
  - benchmark
  - metric
  - scalability
  - robustness
  - interpretability
  - deployment constraints

比较轴不是固定的，应该从领域自动生成。

比如对 Auto Research / Survey Engine 这个领域，比较轴可以是：

轴	问题
Retrieval breadth	能否找全关键文献？
Retrieval depth	是否能读全文、图表、附录、代码？
Synthesis depth	是否能生成 taxonomy、比较、gap？
Citation reliability	claim 是否有精确证据？
Freshness	是否能处理最新论文？
Human alignment	是否符合读者需求？
Evaluation	是否有 quiz、nugget、citation、academic value 评估？

⸻

9. Stage 7：Writing —— Evidence-constrained generation，而不是自由写作

最终写作阶段应该被严格约束。

9.1 每段都要有 ParagraphPlan

ParagraphPlan:
  section:
  purpose:
    - define concept
    - compare methods
    - explain evolution
    - summarize benchmark
    - identify gap
  key_claims:
    - claim_id
  required_evidence:
    - evidence_id
  allowed_papers:
    - paper_id
  forbidden_overclaims:
    - "do not claim SOTA unless benchmark-aligned"
    - "do not generalize from dataset X to all domains"

然后 LLM 只能根据 ParagraphPlan 写段落。

9.2 Claim 类型分级

不是所有句子都需要同等强度引用。可以把 claim 分为：

Claim 类型	是否必须引用	例子
Definition claim	是	“X refers to…”
Method claim	是	“A uses B to achieve C.”
Result claim	强制	“A outperforms B on dataset D.”
Comparative claim	强制且需要多源	“A is more efficient than B.”
Gap claim	强制且需要 limitation 证据	“Existing methods struggle with…”
Narrative transition	可选	“This motivated later work…”

尤其是 comparative claim 和 gap claim 必须多证据支持，否则容易胡说。

⸻

10. Stage 8：VERIFY —— 可信度不是靠“让 LLM 再检查一下”，而是 claim-level 审计

你提到的可信度问题可以用一个专门的 Verifier Agent 解决，但它必须工作在结构化 claim 上。

10.1 Claim-Citation Verification

对每个 claim 做判断：

ClaimVerification:
  claim:
  cited_evidence:
  verdict:
    - supported
    - partially_supported
    - unsupported
    - contradicted
    - overgeneralized
    - wrong_scope
  reason:
  suggested_fix:

典型错误包括：

错误	例子
Unsupported	引用论文没有说这句话
Wrong scope	论文只在小数据集验证，却被写成通用结论
Metric mismatch	AUC 和 accuracy 被混着比较
Dataset mismatch	不同 benchmark 上的结果被直接比较
Temporal mismatch	旧论文被说成最新方法
Citation laundering	引用综述中的二手说法，而不是原论文

DeepScholar-Bench 的一个重要启示是，可验证性并非完全不可达；通过精心策展和过滤排序，引用精确率可以显著提高。 ￼ 所以你可以把 “high citation precision” 作为系统的一等目标。

10.2 Overclaim Detector

可以单独做一个 ScopeGuard：

Input: claim + evidence + PaperCard.scope
Output:
  是否把局部实验说成全局结论？
  是否把作者假设说成事实？
  是否把未来工作说成已有结果？
  是否把相关方向错误合并？

例如：

Claim: "Method X solves long-context reasoning."
Evidence: Method X improves passkey retrieval on synthetic tasks.
Verdict: overgeneralized.
Fix: "Method X improves performance on synthetic long-context retrieval tasks, but its effect on broader reasoning benchmarks remains less established."

这正是你说的“每篇工作的评测 scope 可能不同”的问题。

⸻

11. Agent 设计：不要让 subagent 自由发挥，而要让它们填结构化槽位

你现在 Holos-research 的问题之一是：5~8 个 scholar/scout subagent 自由文本返回论文，主 agent 再 ingest。这个模式的问题是：

* 子 agent 覆盖范围不可控；
* 返回格式不可比；
* 主 agent 认知负担过高；
* 没有全局 coverage signal；
* 很难 debug 为什么漏文献。

建议改成 typed agents + shared state。

11.1 推荐 agent 角色

Agent	任务	输入	输出
AnchorAgent	定义 SurveySpec	用户问题	SurveySpec
QueryPlanner	生成查询程序	SurveySpec	QueryPlan
SearchAgent	多源检索	QueryPlan	RawCandidates
GraphExpander	引用图扩展	Seed papers	ExpandedCandidates
DedupRanker	去重排序	Candidates	RankedPool
PaperReader	结构化读论文	PDF/full text	PaperCard
EvidenceExtractor	抽取证据	Paper sections	EvidenceCard
RelationBuilder	建图	PaperCards	LiteratureGraph
CoverageCritic	找覆盖缺口	Graph + SurveySpec	CoverageReport
TaxonomyBuilder	生成分类体系	Graph	Taxonomy
SynthesisPlanner	生成写作计划	Taxonomy + Evidence	SurveyPlan
Writer	写正文	ParagraphPlan	Draft
Verifier	查 claim-citation	Draft + Evidence	VerificationReport
Refiner	修订	VerificationReport	FinalSurvey

11.2 每个 agent 都应该“低自由度”

例如 SearchAgent 不应该返回自由文本：

I found several relevant papers...

而应该返回：

SearchResult:
  query_id:
  source:
  paper_id:
  title:
  url:
  reason_for_inclusion:
  matched_terms:
  likely_role:
    - foundation
    - method
    - benchmark
    - application
    - survey
  confidence:

PaperReader 不应该写“这篇论文主要讲了……”，而应该填 PaperCard。

这样主 agent 不需要“相信子 agent”，只需要聚合结构化对象。

⸻

12. 数据库与中间表示：建议用 Research Wiki + Graph + Vector Store 三层

你已有 research_wiki(ingest_paper)，可以扩展成三层存储。

12.1 Metadata DB

存论文元数据：

paper_id, title, authors, year, venue, arxiv_id, doi, citation_count, references, citations

12.2 Vector Store

存：

* abstract embedding；
* full-text chunk embedding；
* method section embedding；
* experiment section embedding；
* PaperCard embedding；
* EvidenceCard embedding。

注意不同字段应分开 embedding。不要把整篇论文混成一个向量。

12.3 Graph DB

存 LiteratureGraph：

Paper -proposes-> Method
Paper -evaluates_on-> Dataset
Method -extends-> Method
Claim -supported_by-> Evidence
Paper -cites-> Paper

Graph DB 的好处是后续可以问：

* 哪些方法共享 dataset？
* 哪些论文都报告同一个 limitation？
* 哪些 paper 是某 cluster 的中心？
* 哪些 cluster 缺 benchmark？
* 某 claim 的证据链是什么？

⸻

13. 一个完整 Survey 生成流程示例

假设用户输入：

我想写一篇关于 LLM KV cache compression 的 survey。

系统内部流程：

Step 1：AnchorAgent

输出：

main_question: "How do recent methods reduce KV cache memory/computation while preserving long-context performance?"
reader_need: "technical researcher seeking taxonomy, comparison, and open problems"
scope:
  include:
    - KV cache eviction
    - KV cache merging
    - low-rank KV
    - quantization
    - retrieval-based cache
    - long-context inference benchmark
  exclude:
    - general model pruning
    - prompt compression unless related to KV cache

Step 2：QueryPlanner

生成 query：

Q1: "KV cache compression large language models"
Q2: "KV cache eviction long context transformer"
Q3: "attention sink KV cache streaming LLM"
Q4: "KV cache merging RoPE long context"
Q5: "LongBench KV cache compression benchmark"
Q6: "LLM inference memory reduction key value cache"
Q7: "survey efficient long context LLM inference"

Step 3：Search + Expand

得到 seed 论文，然后：

* backward 找 Transformer、attention、long-context 基础；
* forward 找近期 follow-up；
* semantic 找命名不同但问题相近的方法；
* benchmark 搜 LongBench、Needle-in-a-Haystack、passkey retrieval 等。

Step 4：PaperReader

为核心论文生成 PaperCard：

method_family: "token eviction"
core_idea: "retain only selected KV entries based on attention statistics"
datasets: ["LongBench", "..."]
metrics: ["accuracy", "latency", "memory"]
limitation: "may fail when discarded tokens become relevant later"
scope: "validated on selected long-context benchmarks, not all reasoning tasks"

Step 5：RelationBuilder

建图：

StreamingLLM -> introduces -> attention sink
H2O -> uses -> heavy hitter eviction
SnapKV -> uses -> observation window clustering
PyramidKV -> uses -> layer-wise budget allocation
...

Step 6：CoverageCritic

发现：

weakness:
  - benchmark coverage insufficient for retrieval-heavy tasks
  - quantization-based KV methods underrepresented
  - deployment latency papers missing
next_queries:
  - "KV cache quantization LLM inference"
  - "KV cache compression latency benchmark GPU"

Step 7：TaxonomyBuilder

生成 taxonomy：

1. Selection-based compression
2. Merging-based compression
3. Quantization-based compression
4. Architecture-aware or training-based compression
5. Hybrid and systems-level methods

Step 8：SynthesisPlanner

生成每节的比较轴：

- retained information criterion
- compression granularity
- whether training-free
- compatibility with RoPE
- benchmark scope
- memory-latency tradeoff

Step 9：Writer + Verifier

写出正文后逐 claim 检查：

Claim: "SnapKV consistently outperforms H2O."
Verifier: unsupported unless benchmark X/Y and setting Z match.
Fix: "On the reported long-context benchmarks under comparable cache budgets, SnapKV reports stronger performance than H2O in several settings..."

⸻

14. 关键创新点：你的系统可以主打什么？

如果你要把这个做成一个 research proposal 或工程系统，我建议不要泛泛说“我们做了更好的 survey agent”，而是强调下面四个创新点。

14.1 Coverage-driven Literature Discovery

现有系统通常是：

search → write

你的系统可以是：

search → graph → coverage diagnosis → targeted search

也就是，检索不是一次性的，而是由 coverage report 驱动的闭环。

核心贡献：

We propose coverage-driven literature discovery, where retrieval is iteratively guided by missing concept, benchmark, comparison, and gap signals extracted from a structured literature graph.

这能直接回应 DeepScholar-Bench 暴露的 citation coverage 和 document importance 问题。 ￼

14.2 Evidence-constrained Survey Writing

不是先写再补引用，而是：

EvidenceCard → Claim → ParagraphPlan → Draft

核心贡献：

Every comparative or gap claim must be grounded in structured evidence spans and verified against paper-specific evaluation scope.

这直接回应可信度问题。

14.3 Comparison-ready Taxonomy Construction

不只是聚类，而是判断 cluster 是否支持比较：

same task + overlapping benchmark + comparable metric + enough competing papers

核心贡献：

We introduce comparison readiness as a criterion for survey taxonomy construction.

这直接回应 DeepSurvey-Bench 所说的学术交流价值不足。 ￼

14.4 Gap Readiness Scoring

不是让 LLM 随便写 future work，而是从 limitation evidence、contradiction、benchmark failure 中归纳 gap。

核心贡献：

We formulate gap readiness to distinguish evidence-supported research gaps from speculative future directions.

这能显著提高 survey 的研究引导价值。

⸻

15. 评估设计：不能只看最终文本，要评估每个中间环节

你可以把评估分为五层。

15.1 Retrieval Evaluation

指标	含义
Citation coverage	是否找到人工 survey 引用的关键论文
Important paper recall	是否找到高影响力论文
Diversity coverage	是否覆盖不同方法族
Benchmark paper recall	是否找到 dataset/metric/benchmark 论文
Freshness	是否覆盖近期关键论文

DeepScholar-Bench 已经把检索质量作为核心维度之一，包括相关率、引用覆盖和文档重要性。 ￼ 你可以直接借鉴。

15.2 Understanding Evaluation

随机抽 PaperCard，人工或 LLM judge：

指标	问题
Method correctness	方法描述是否准确
Scope correctness	是否正确识别实验范围
Limitation extraction	是否提取真实 limitation
Benchmark extraction	数据集/指标是否完整
Claim-evidence alignment	claim 是否有证据支持

15.3 Graph Evaluation

指标	问题
Method family purity	cluster 内是否真同类
Relation accuracy	extends/contrasts/evaluates_on 是否正确
Bridge discovery	是否发现跨 cluster 联系
Benchmark connectivity	方法和 benchmark 是否连通

15.4 Synthesis Evaluation

对应 SurveyBench 和 DeepSurvey-Bench：

维度	指标
Breadth	覆盖多少核心主题
Depth	是否解释方法细节和差异
Reasoning	是否建立跨概念联系
Academic value	是否有清晰目标、比较、批判、gap
Reader utility	是否能回答 quiz / nugget questions

SurveyBench 使用 quiz-driven evaluation 来测试综述是否满足读者信息需求，这一点非常适合你的系统。 ￼

15.5 Trustworthiness Evaluation

指标	问题
Citation precision	引用是否支持对应句子
Claim coverage	多少 claim 有证据
Overclaim rate	是否扩大论文结论
Contradiction rate	是否和引用相矛盾
Unsupported comparison rate	是否做了不可比比较

⸻

16. MVP 实现路线：不要一开始做全自动 survey，先做 Research Map

我建议分三期做。

Phase 1：Survey Discovery Engine

目标：解决广度。

实现：

* QueryPlanner；
* 多源 search；
* citation graph expand；
* dedup/rank；
* coverage report；
* 输出候选论文列表和 cluster。

MVP 产物：

给定 topic，输出：
1. 关键论文列表；
2. 方法族聚类；
3. benchmark / dataset / metric 列表；
4. 缺失方向；
5. 推荐下一轮 search query。

这个阶段不追求写完整 survey，只要比普通 keyword search 找得更全、更准。

Phase 2：Paper Understanding Engine

目标：解决深度。

实现：

* PDF/full-text parsing；
* PaperCard；
* EvidenceCard；
* claim/scope extraction；
* benchmark table extraction；
* code link extraction。

MVP 产物：

给定 20 篇核心论文，输出：
1. 每篇 PaperCard；
2. 方法比较表；
3. dataset/metric matrix；
4. limitation matrix；
5. claim-evidence database。

这个阶段可以很好地支撑你说的“理解能力”。

Phase 3：Survey Synthesis Engine

目标：解决综合和可信度。

实现：

* TaxonomyBuilder；
* SynthesisPlanner；
* evidence-constrained Writer；
* Verifier；
* Refiner。

MVP 产物：

给定 Research Map，输出：
1. survey outline；
2. taxonomy；
3. comparison sections；
4. gap analysis；
5. verified draft。

⸻

17. 我认为最终框架可以命名为：Coverage-Guided Evidence-Constrained Survey Engine

可以概括为：

A survey generation framework that first constructs a structured literature graph through query expansion, citation expansion, and coverage diagnosis, then performs evidence-constrained synthesis with claim-level verification.

一句话版：

不是让 LLM 直接写综述，而是让系统先构建一个可审计的领域知识地图，再从这个地图中生成有证据约束的综述。

⸻

18. 最终推荐的系统结构图

User Topic / Anchor Paper
        │
        ▼
[0] Anchor Modeling
        │
        ▼
SurveySpec
        │
        ▼
[1] Query Program Generation
        │
        ▼
Structured Queries
        │
        ▼
[2] Multi-source Search ─────┐
        │                    │
        ▼                    │
Seed Papers                  │
        │                    │
        ▼                    │
[3] Citation / Semantic Expansion
        │
        ▼
Candidate Literature Universe
        │
        ▼
[4] Dedup + Influence/Diversity Ranking
        │
        ▼
Ranked Paper Pool
        │
        ▼
[5] Full-text Understanding
        │
        ├── PaperCards
        ├── EvidenceCards
        └── ScopeCards
        │
        ▼
[6] LiteratureGraph Construction
        │
        ├── Method Graph
        ├── Benchmark Graph
        ├── Claim-Evidence Graph
        └── Limitation Graph
        │
        ▼
[7] Coverage Critic
        │
        ├── Missing Concepts
        ├── Weak Method Clusters
        ├── Missing Benchmarks
        ├── Comparison Readiness
        └── Gap Readiness
        │
        ├── if insufficient → back to Search/Expand
        ▼
[8] Taxonomy + Synthesis Plan
        │
        ▼
[9] Evidence-constrained Writing
        │
        ▼
Draft Survey
        │
        ▼
[10] Claim-Citation Verification
        │
        ├── unsupported claims
        ├── overclaims
        ├── wrong-scope comparisons
        └── missing citations
        │
        ▼
Verified Survey + Research Map

⸻

19. 最核心的设计原则

我会把这个系统的核心原则总结成 8 条：

1. 先建图，再写作：Survey 是 LiteratureGraph 的叙事投影。
2. 先 coverage，再 synthesis：没有覆盖诊断，不允许生成最终综述。
3. 先证据，再 claim：不允许写没有 EvidenceCard 支撑的比较和 gap。
4. 读全文，而不是只读摘要：至少核心论文必须解析 method / experiment / limitation。
5. 比较必须 comparison-ready：同任务、同 benchmark、同 metric 才能强比较。
6. gap 必须 evidence-ready：future work 来自 limitation 和 contradiction，而不是想象。
7. 引用必须 claim-level verification：不是段末堆引用，而是每个关键 claim 对齐证据。
8. 检索必须闭环：Search → Coverage → Search，而不是一次性关键词检索。

⸻

20. 最简洁的实现优先级

如果你现在要开始做，我建议优先做这 5 个模块：

1. SurveySpec + QueryPlanner
    先把用户 topic 结构化，生成 typed queries。
2. Search + Expand + DedupRanker
    多源搜索 + 引用图扩展 + 重要性/多样性排序。
3. PaperCard Extractor
    对核心论文抽 method、benchmark、metric、limitation、scope。
4. CoverageCritic
    输出 missing concepts、weak clusters、benchmark gaps、comparison readiness。
5. Claim Verifier
    对生成内容做 claim-citation-scope 检查。

先别急着做华丽的最终写作。只要这五个模块做好，你的系统已经会比普通 deep research agent 更像一个“研究助理”，而不是“长文生成器”。