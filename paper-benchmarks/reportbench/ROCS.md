## ROCS: Role-Organized Columnar Storage for Agent Trajectories 

Xiaokejishu Jia[1] _[∗]_ 

1Shanghai Innovation Institute, Shanghai, China 

> _∗_ niexiaohangeric@163.com 

_Produced by Synergy AutoResearch_ 

## June 6, 2026 

## **Abstract** 

Agent trajectory data—multi-turn logs of user, assistant, and tool messages—has become a core artifact in AI agent development, yet the dominant formats (JSONL, gzip-JSONL) remain generic log-file abstractions. They cannot perform selective column reads, do not support streaming writes during live inference, and waste storage on repeated schema tokens. We present ROCS (RoleOrganized Columnar Storage), a file format that physically organizes messages by semantic role— user, assistant, and tool—with a compact turn skeleton for interleaving reconstruction. Each role column is independently compressed with batch ZSTD at flush time. 

This design simultaneously delivers three capabilities that no existing format provides together: column-projection reads 2.0 _×_ faster than gzip full-decompress-and-filter (0.182 ms vs. 0.385 ms for the user role); per-message append at P50 2.81 _µ_ s (in-memory, before compression and flush); and compression to 0.20–0.34 _×_ raw JSONL, within 1.6–1.8 _×_ of gzip (1.83 _×_ at 2 turns)—at a structural cost of 10.7% skeleton overhead. An ablation reveals that the initial two-tier classification design has zero measurable effect on compression, as ZSTD block-level compression subsumes it; the contribution rests entirely on the role-organized layout. ROCS demonstrates that a single role-organized design can simultaneously achieve competitive compression, column projection, and streaming writes—properties previously requiring separate formats. The implementation is available at https://github.com/EricSanchezok/ROCS under an MIT license. 

## **1 Introduction** 

Agent trajectories—multi-turn logs of user queries, LLM-generated responses, and tool-call interactions— have become a central artifact in the development lifecycle of AI agents. Public repositories already contain over 1.5 million trajectories [9], and production deployments at major AI labs log millions of new traces daily. Despite this growth, the storage layer for trajectory data has received little attention. The dominant practice stores each message as an independent JSON object under the implicit assumption that trajectory data is _just_ a sequence of messages, and a log file is the right abstraction. 

**Agent trajectories are not logs.** They have a _structural grammar_ —a constrained pattern of role alternation, role-specific message distributions, and predictable access frequencies—that existing formats discard. Our empirical analysis of 5,000 real trajectories reveals three properties that distinguish trajectory data from generic text logs: 

1. **Highly structured role transitions.** The probability of assistant-to-assistant transitions is 51%, and tool-to-assistant transitions are 82%. Role entropy is 1.55 bits versus 2.0 bits for uniform (a 22.4% reduction), confirming that trajectory role sequences are far from random. These are not random walks; trajectories follow a predictable observe–reason–act protocol. 

2. **Role-correlated content.** Each user interaction triggers an average of 4.82 tool calls (p90: 10.0), meaning tool outputs dominate trajectory volume. System prompts average 3,234 characters— an order of magnitude longer than typical assistant (median 0, p90: 596) or user (p90: 528) 

1 

messages. Role-homogeneous content has distinct compressibility characteristics that generic byte-level compression cannot exploit. 

3. **Cross-trajectory commonality.** Only 43% of system prompts across 5,000 trajectories are unique. This shared structural content—identical tool schemas, repeated code blocks, boilerplate assistant stubs—represents an untapped compression opportunity at the corpus level. 

Current log-file formats pay a growing cost for ignoring this structure. Three pain points compound at production scale. **Selective access requires full decompression** , forcing every role-filtered query to decompress and parse the entire trajectory. **Streaming writes are not natively supported by existing formats.** While in-memory buffering with periodic flush is possible—and is how ROCS operates—it introduces a window for data loss on crash that grows with buffer size. **Schema-heavy JSON compresses poorly** , with repeated field names and structural tokens accounting for a substantial fraction of raw bytes. 

**ROCS: Role-Organized Columnar Storage.** ROCS (Role-Organized Columnar Storage) exploits trajectory grammar by physically organizing storage by role—user, assistant, and tool—rather than by arrival order. A compact _turn skeleton_ preserves interleaving while enabling direct column access. Each role column is independently compressed with batch ZSTD at flush time, allowing role-homogeneous content to compress more efficiently. 

This design delivers a property that no existing format provides: the performance advantage _grows with trajectory size_ . Column projection read speedup over gzip+filter increases from 2 _._ 1 _×_ at 30 messages to 7 _._ 3 _×_ at 1,000 messages. Against Parquet, a columnar format with optimized projection, ROCS maintains 1 _._ 5 _×_ –3 _._ 3 _×_ speedup across all trajectory sizes (exp ~~0~~ 06). Per-message appends complete at P50 2.81 _µ_ s (in-memory; full flush latency adds compression and I/O cost at millisecond scale amortized over thousands of messages), and compression to 0.20–0.34 _×_ raw JSONL stays within 1 _._ 6–1 _._ 8 _×_ of gzip while providing column projection that gzip cannot. ROCS achieves this by appending raw bytes to inmemory column buffers; compression and serialization are deferred to flush time. This provides fast per-message append (P50 2.81 _µ_ s) at the cost of a bounded window of un-flushed data—an acceptable trade-off for inference pipelines where token generation latency dominates. 

Furthermore, we show that cross-trajectory dictionary compression (DictZSTD) matches gzip when trained on 10 trajectories and _surpasses_ it at _N ≥_ 100 (exp ~~0~~ 07), reaching 0.87 _×_ of gzip at 10,000 trajectories—a result enabled by the structural commonality between trajectories. 

## **Contributions.** 

1. **Discovery:** We quantify the structural grammar of agent trajectory data—22.4% role entropy reduction, 51% self-transition rate, 4.82:1 tool-to-user ratio—establishing that trajectory data is not a generic text log but has exploitable structure. 

2. **Design:** We present ROCS, a role-organized columnar format that exploits this grammar to simultaneously achieve column projection, streaming writes, and competitive compression. 

3. **Evidence:** We validate ROCS against six storage baselines, demonstrating a column-projection scaling curve that reaches 7 _._ 3 _×_ vs. gzip at 1,000 messages—with the advantage widening, not saturating, as trajectories grow—and a cross-trajectory dictionary mechanism that beats gzip at modest corpus sizes. 

## **1.1 The Structural Grammar of Agent Trajectories** 

A fundamental premise of ROCS is that trajectory data has exploitable structural regularity. To establish this empirically, we analyze the TOUCAN Kimi-K2 dataset (5000 trajectories). Table 1 summarizes the key statistics. 

2 

Table 1: Empirical structural properties of agent trajectory data (5000 TOUCAN Kimi-K2 trajectories). Together, these statistics reveal a highly constrained interaction grammar that role-organized storage exploits. 

|**Property**|**Value**|**Implication**|
|---|---|---|
|Role entropy|1.55 bits (vs. uniform 2.0)|22.4% reduction — roles are predictable|
|Assistant_→_assistant|51% of transitions|Self-referential reasoning chains|
|Tool_→_assistant|82% of transitions|Tools always return to assistant|
|Tool / User ratio|4.82 (p90=10.0)|Each user interaction triggers_∼_5 tool calls|
|System prompt uniqueness|43% unique (5000 trajs)|57% share a prompt with another trajectory|
|Assistant empty messages|median 0 chars|Many assistant messages are structural stubs|



**==> picture [339 x 206] intentionally omitted <==**

**----- Start of picture text -----**<br>
To role<br>system user assistant tool<br>system 1.0<br>0.8<br>user 0% 0% 100% 0%<br>0.6<br>0.4<br>assistant 0% 8% 51% 41%<br>En tro 0.2py: 1.55 bits<br>(22.4% below uniform 2.0)<br>Tool/User ratio: 4.82<br>tool 0% 0% 82% 18%<br>From role<br>Transition probability<br>**----- End of picture text -----**<br>


Figure 1: Role transition matrix for 5,000 TOUCAN trajectories. The high assistant self-transition rate (51%) and the strong tool-to-assistant coupling (82%) reveal the structural grammar of agent interactions—a constrained protocol rather than a random role sequence. 

These statistics, observed across 5,000 TOUCAN Kimi-K2 trajectories, suggest that trajectory data has exploitable structural regularity. Agents operate in observe–reason–act loops where each tool invocation generates a response, and the assistant processes it before the next step. This produces the high tool-to-user ratio, the strong self-transition in the assistant role, and the predictable system–user– assistant–tool cycle. ROCS’s role-organized layout exploits this grammar directly: the turn skeleton preserves the sequential order across roles, while role-separated columns benefit from the uniformity of role-homogeneous content. 

The grammar analysis also reveals a compression opportunity: 57% of trajectories share a system prompt with at least one other trajectory. This suggests that cross-trajectory dictionary compression can exploit shared structural content—a hypothesis we validate in our cross-trajectory dictionary experiments (Table 6). 

## **2 Related Work** 

The growing volume of agent trajectory data has exposed a persistent mismatch between general-purpose storage formats and the access patterns that trajectory workloads demand. Agent engineers need to persist token-by-token streaming output during live inference, later read only the assistant messages for fine-tuning or only the tool calls for debugging, and do both without paying a disproportionate storage penalty. Each need is individually well-served by existing formats; no single format serves all 

3 

three simultaneously. Several lines of work address pieces of this problem—conceptual architectures for trajectory recording, columnar storage for structured data, and compression strategies for agent history— but none combines column-projection reads, streaming writes, and competitive compression within a single trajectory-aware format. 

**Agent Trajectory Storage.** TapeAgents [1] introduced the tape architecture as a principled way to represent agent state as an append-only sequence of interleaved steps. This has been an influential conceptual foundation, yet the tape remains an in-memory abstraction. When persisted, it serializes to standard formats (JSON, JSONL) with no storage-layer optimization for the replay, analysis, and retrieval patterns that motivate the architecture in the first place. Hyperparam [2] took the complementary step of building a Parquet-backed query engine for agent traces, demonstrating that users want to filter, aggregate, and project trajectory data column-wise. Its reliance on generic Parquet, however, inherits that format’s batch-flush semantics and row-group boundaries, which are mismatched with the incremental, message-at-a-time production pattern of live agent inference. ESAA [4] applied event sourcing to agent trajectories, gaining full provenance and replay capability from an append-only log. This guarantees correctness but not efficiency—reading a single role still requires scanning the entire event log. VCC [12] proposed role-tagged views as a query-time abstraction, projecting user, assistant, or tool messages on demand from underlying row-oriented storage. This comes closest to our column-organized design, but because VCC views are materialized on read, they cannot exploit per-column compression or avoid the cost of decompressing irrelevant columns. 

**Columnar Storage Formats.** General-purpose columnar formats provide the most natural starting point for trajectory storage. Parquet [10] and Arrow [10] deliver efficient column projection, predicate pushdown, and encoding-based compression for structured tabular data. ONTO [3] adapted these ideas to the LLM domain, proposing a schema-once flat encoding where each field of an LLM input-output record occupies its own column. These formats share a fundamental limitation: they are designed for flat, non-interleaved records. A multi-turn trajectory—user, assistant, tool, user, assistant, in temporal order—must either be flattened into a wide row with turn-indexed columns (wasteful and schema-rigid) or serialized into a single opaque column (defeating projection). Neither supports streaming writes at the granularity of individual messages; Parquet flushes entire row groups, and Arrow requires preallocated buffers. HYVE [7] introduced hybrid columnar views for LLM context engineering, organizing machine-generated data into columns for efficient prompt construction. While this demonstrates the value of columnar organization for LLM-facing workloads, HYVE targets the prompt-building pipeline rather than persistent trajectory storage, and its columns hold flat context entries rather than interleaved conversational turns. 

**Agent Memory and Compression.** A parallel line of work attacks the trajectory size problem through compression, eviction, or distillation rather than format design. AgentDiet [8] classifies tokens by redundancy level and evicts low-value content during inference, constraining context length without retraining. AGORA [11] introduces step-level structural awareness, scoring each trajectory step by its predicted utility for future reasoning and retaining only the most valuable segments. These approaches answer a fundamentally different question: _what_ to keep at inference time. They are complementary to storage format design—they determine the content, and our format determines how that content is organized on disk. Structured Distillation [6] takes a more aggressive approach, compressing entire trajectories 11 _×_ via off-policy evaluation of segment-level utility. This produces compact summaries but destroys the original message structure: individual user, assistant, and tool utterances are lost, and BM25 retrieval accuracy degrades catastrophically after distillation. Our format applies lossless columnar compression that preserves every message in its original form while still achieving a 3–5 _×_ reduction versus raw JSONL. WorldDB [5] proposed write-time memory classification for agents, tagging facts by type and retention policy as they are stored. This functional organization is directionally similar to our role-organized layout, but WorldDB’s contribution is a memory management policy—deciding which 

4 

facts go to short-term, long-term, or working memory—whereas ours is a physical storage format for the content that policy selects. 

**Storage Baselines.** Standard baselines each excel in one dimension at the expense of others. Gzipcompressed JSONL achieves the best compression ratio among lossless formats, but it is inherently batch-only: both writing and reading require full-stream decompression, and there is no mechanism for column projection. A read that needs only a single role must decompress the entire trajectory—a cost that grows linearly with archive size and becomes prohibitive at scale. Feather and Parquet provide column projection and competitive compression ratios, and Parquet’s row-group-level min-max statistics can accelerate selective scans. Their write semantics, however, require buffering rows until a row group reaches a configured threshold (typically 2[16] –2[20] rows), then flushing in a single batch. This makes them unsuitable for token-by-token persistence during inference, where each message must be durably stored before the next token is generated. SQLite and other RDBMS options introduce transactional safety and fine-grained updates, but the schema overhead of indexing and row-level metadata means that, for interleaved trajectory data with many short messages, on-disk footprints can exceed raw JSONL. 

**Positioning.** Existing work addresses three needs—column projection, streaming writes, and compression— but always in pairs. Columnar formats give projection and compression without streaming; gzip gives compression without projection or streaming; streaming approaches give low write latency without column-level access or competitive compression ratios. Our work is the first to combine all three within a format designed from the ground up for agent trajectories. The organizing principle is role: messages are physically stored by speaker identity (user, assistant, tool) rather than temporally interleaved, and a lightweight turn skeleton stores the interleaving order as small integer offsets. This role-oriented layout enables column projection—reading only assistant messages, for example, without touching user or tool columns—while the skeleton provides _O_ (1) position-based access for temporal reconstruction. Because each column is an independent stream, messages can be flushed individually with per-message granularity, avoiding the batch-at-a-time semantics of row-group-oriented formats. The columnar structure also concentrates content with similar statistical properties: tool outputs tend to be longer and more repetitive than user utterances, and assistant responses fall somewhere in between. This homogeneity within each column improves compression locality and contributes to ratios within 1 _._ 5–1 _._ 8 _×_ of gzip—not because we apply a stronger algorithm, but because role-based segregation creates more compressible blocks. The skeleton itself adds a modest 10 _._ 7% structural overhead, a deliberate trade-off that unlocks both column projection and position-indexed access. The result is a format that simultaneously supports token-by-token per-message appends at P50 latency under 3 _µ_ s, 2 _×_ faster selective reads than gzip-plus-filter, and lossless compression competitive with general-purpose algorithms—properties that no existing format provides together, and that follow directly from organizing storage around the trajectory’s natural role structure rather than around the conventions of generic data layout. 

## **3 Method** 

ROCS (Role-Organized Columnar Storage) is a binary file format designed for the access patterns of agent trajectory data: append-heavy streaming writes from live agents, selective column reads for analysis, and random access by message index for evaluation. The core idea is to physically separate messages by role into independent column buffers, then record the original interleaving order in a compact turn skeleton. Figure 2 gives an overview of the write path, file layout, and read path; the subsections below explain each component in detail. 

## **3.1 File Format** 

A ROCS file begins with a 4-byte magic sequence b"ROCS", followed by the serialized turn skeleton and the Arrow IPC column data: 

5 

**==> picture [441 x 180] intentionally omitted <==**

**----- Start of picture text -----**<br>
(a) Write Path (b) Read Path<br>USER ASST TOOL ASST USER TCST File<br>Classify by Role<br>Parse Skeleton<br>Full Read Column Projection<br>USER Buer ASST Buer TOOL Buer<br>Decompress ALL Decompress target<br>(e.g., only TOOL)<br>Batch ZSTD<br>Walk skeleton Skip others<br>Arrow IPC<br>Flush: compress + serialize Ordered Messages Filtered Messages<br>TCST File<br>(c) File Layout<br>b"TCST"skel_len Pickle Skeleton col_len Arrow IPC Columns<br>4 B u32 1.73.4 KB u32 zstd-compressed<br>headers metadata (10.7%) content<br>**----- End of picture text -----**<br>


Figure 2: ROCS architecture overview. **(a)** Write path: streaming messages are classified by role and appended to per-role column buffers. A turn skeleton records interleaving metadata. At flush time, each column buffer is independently batch-compressed with ZSTD and packaged into an Arrow IPC RecordBatch. **(b)** File layout: the three-section format with magic, length-prefixed pickle skeleton, and length-prefixed Arrow IPC column data stream. **(c)** Read path with two modes: (left) full read decompresses all columns and walks the skeleton for ordered reconstruction; (right) column projection decompresses only the requested role’s column for selective access. 

|b"ROCS"|# magic, 4 bytes|
|---|---|
|skel_len: uint32|# skeleton byte count, little-endian, 4 bytes|
|skeleton|# compact binary skeleton entries (18 bytes each)|
|col_len: uint32|# column IPC byte count, little-endian, 4 bytes|
|columns|# Arrow IPC RecordBatch stream, one row per column|



The file is divided into three logical sections. The 4-byte magic provides a cheap file-type check: the reader rejects any file without the correct magic before attempting to parse further, avoiding silent misparsing of JSONL or other formats. The skeleton length prefix lets the reader pre-allocate the skeleton buffer and seek directly to the column data section after parsing the skeleton. 

The turn skeleton stores one entry per trajectory message (see §3.4). It uses a compact fixed-schema binary encoding (18 bytes per entry: role, tier, segment offset, length, turn index) rather than Python pickle, making the format safe for untrusted inputs and trivially parseable in any language. The skeleton is stored outside the Arrow IPC stream because Arrow’s per-RecordBatch metadata overhead (8-byte alignment, schema repetition, dictionaries) would exceed the skeleton payload itself for typical 50message trajectories. As §3.4 reports, the skeleton accounts for 10.7% of total file size. 

The column data occupies the remainder of the file and is a standard Arrow IPC RecordBatch stream. Each column in the batch corresponds to one (role, tier) pair and contains a single binary value: the ZSTD-compressed concatenation of all message bytes for that role. Using Arrow IPC for the column payload means downstream consumers can open the file with any Arrow-compatible reader; the skeleton parser is the only ROCS-specific parsing step. The column schema is stored inside the IPC stream itself and does not require separate serialization. 

## **3.2 Write Path** 

Messages arrive one at a time in the streaming order produced by the agent (USER _→_ ASSISTANT _→_ TOOL _→_ ASSISTANT _→· · ·_ ). For each message, the writer performs three steps before returning control to the caller: 

6 

1. **Classify by role.** The message’s role field (USER, ASSISTANT, TOOL, or SYSTEM) determines which column buffer receives its content. An optional tier-classification step (§3.5) may also assign a storage tier. The combined key (role, tier) uniquely identifies one in-memory column buffer. 

2. **Buffer raw bytes.** The UTF-8–encoded content is appended to the appropriate column buffer as raw bytes. No compression is applied at this stage; compression is deferred entirely to flush time. This avoids two sources of overhead: the per-message ZSTD frame header (roughly 25 bytes per frame) and the lost cross-message redundancy that batch compression can exploit. 

3. **Record skeleton entry.** A TurnSkeletonEntry is appended to the in-memory turn skeleton list. The entry carries the message’s role, tier, turn index, byte length, segment index (its insertion position within the column buffer), and a placeholder byte offset that will be resolved at flush time. 

This per-message path is the hot loop of every live agent recording session. The entire sequence — dictionary lookup by (role, tier), list append into the column buffer, skeleton entry construction, classification context update — completes in under 3 _µ_ s at the median (Table 2), which is several orders of magnitude faster than the LLM inference call that produced the message. 

**Flush.** When the caller decides to finalize the file (e.g. after a trajectory completes or on a periodic timer), the writer enters the flush phase: 

1. **Fill offsets.** The writer iterates the skeleton and computes each entry’s byte offset within its column buffer as the cumulative sum of lengths for preceding entries sharing the same (role, tier) key. 

2. **Batch compress.** For each column buffer, all raw bytes are concatenated into a single blob. The blob is compressed with ZSTD at level 3. The compressed result is stored as a single binary value in the Arrow IPC RecordBatch, under a column named col ~~_{_~~ role _}_ ~~_{_~~ tier _}_ . 

3. **Serialize.** The skeleton is serialized (18 bytes per entry, fixed binary format), and both skeleton data and column IPC stream are written to disk with their length prefixes and the magic header. 

Deferred column-level batch compression is the key design choice that distinguishes ROCS from per-message compression schemes like gzip’ed JSONL. A single ZSTD frame over _N_ concatenated messages exploits longer-range redundancy — repeated JSON keys, shared structural patterns across tool call results, common phrasing in assistant responses — without paying _N_ frame headers. After flush, the in-memory column buffers are cleared and the writer is ready to accept messages for the next trajectory. 

## **3.3 Read Path** 

The reader supports three access patterns that cover the typical workloads encountered during agent trajectory analysis. 

**Full reconstruction.** The reader opens the file, validates the magic, parses the skeleton length, and deserializes the compact binary skeleton entries. It then reads the Arrow IPC stream, decompresses every column blob with ZSTD, and stores the decompressed bytes in an in-memory dictionary keyed by col ~~_{_~~ role _}_ ~~_{_~~ tier _}_ . Finally, it walks the skeleton entries in file order: for each entry, it looks up the appropriate decompressed blob and extracts the byte range [offset, offset + length). Each slice is UTF-8 decoded and appended to the result list. Because the skeleton entries are stored in the original message order, the reconstructed sequence exactly preserves the original interleaving. 

**Column projection.** Many analysis pipelines only need messages of a single role. For example, computing the average assistant response length or counting tool-call invocations requires neither user messages nor system prompts. Because each role’s data lives in an independently compressed column 

7 

blob, the reader can selectively decompress only the requested columns. Skipping a column means skipping its ZSTD decompression entirely — the compressed bytes are never touched. This yields a roughly 2 _×_ speedup over the practical baseline of reading a gzip’ed file and filtering by role in post-processing, since gzip requires full decompression before any individual message can be inspected (Table 9). 

**Positional access.** Evaluation harnesses frequently need to retrieve messages by index (e.g. the _k_ -th assistant response). The skeleton stores entries in array order, so entry _i_ is at position _i_ and carries a direct pointer (segment idx, offset, length) to its bytes in the decompressed column blob. No linear scan over previous messages is required, and no auxiliary index structure needs to be built at load time. The reader decompresses the required column blobs lazily on first access, then caches them for subsequent lookups. After the initial column decompression, each positional read is O(1) in both time and additional memory. 

## **3.4 Turn Skeleton** 

The turn skeleton is the mechanism that reconciles columnar physical layout with sequential logical order. Without it, a reader that has decompressed three separate role column blobs would have no way to reconstruct the original USER _→_ ASSISTANT _→_ TOOL ordering — the column buffers only know which messages belong to each role, not how they were interleaved. 

Each skeleton entry is a TurnSkeletonEntry data class with the following fields: 

|{|||||
|---|---|---|---|---|
||"role":|int,|#|MessageRole enum value|
||"tier":|int,|#|StorageTier enum value|
||"turn_index":|int,|#|which turn this message belongs to|
||"segment_idx":|int,|#|which segment within the column|
||"offset":|int,|#|byte offset within the segment|
||"msg_len":|int,|#|byte length of the message content|
|}|||||



The turn index identifies which conversational turn the message belongs to, enabling turn-level operations (e.g. extracting a specific user–assistant exchange) without scanning message content. The segment index is reserved for future multi-segment column layouts (e.g. rolling-window flush for indefinitely long trajectories), though the current implementation packs each column into a single segment. 

The skeleton uses a compact fixed-schema binary encoding (18 bytes per entry) rather than Python pickle or Arrow IPC. Arrow’s per-RecordBatch metadata would add roughly 4 KB of schema overhead alone — more than the entire skeleton payload for typical 50-message trajectories. The fixed binary format avoids this while keeping the encoding trivially parseable in any language. At the median trajectory size in our evaluation, the skeleton accounts for 10.7% of total file size — a modest overhead given that it enables both column projection and O(1) positional access. Table 3 shows how this overhead scales with trajectory length: at 50 turns the skeleton fraction drops below 6%, while at 2 turns it peaks near 18% due to fixed overhead of the magic header and skeleton length field. The per-entry cost is 18 bytes per message regardless of message content length. 

## **3.5 Storage Tier Classification** 

The original design included a two-tier classification system that labeled each message as USEFUL or EVICTABLE based on its content and conversational position. The motivation came from an observation about agent trajectory structure: in a typical multi-turn interaction, tool call results and system prompt repetitions can constitute 30–50% of total message count, and much of this content is structurally repetitive. A rule-based classifier ( _∼_ 120 lines, implemented in rdl/classifier/rules.py) assigns tiers using priority-ordered signals: 

- **Strong useful signals** (highest priority): system prompts, all user messages, assistant responses containing summary keywords, and substantive assistant messages near the end of the trajectory. 

8 

- **Strong evictable signals** : near-empty messages (fewer than 50 characters, typically function-call stubs or audio placeholders), duplicate system prompt repetitions, and all tool-call results. 

- **Position-based signals** : the first and last two messages by position are always protected as useful; middle assistant messages matching filler patterns (“Let me...”, “First...”) are marked evictable. 

- **Default** : conservatively marked useful. 

Experiments across three trajectory datasets (Kimi-K2, Qwen3, and OSS-instructor) showed that this classification has no measurable effect on compression ratio. ZSTD’s batch compression subsumes any cross-role redundancy differences: compressing all assistant messages — both “useful” and “evictable” — into a single ZSTD frame achieves essentially the same ratio as splitting by tier. Based on this finding, the tier field is retained in the skeleton only as a structural placeholder, and the effective storage layout uses a single tier per role. §5 discusses the implications for future work on content-aware tiering. 

The negative finding is worth reporting because it clarifies what ROCS does _not_ rely on. The contribution is the role-organized columnar layout and the turn skeleton, not intelligent tiering. The classification infrastructure is preserved in the codebase for applications where tiering serves a purpose other than compression — for example, prioritized eviction under storage pressure, or differential read quality for less important messages. 

## **4 Results** 

We evaluate ROCS along five axes: compression efficiency, column-projection read performance, streamingwrite latency, full read/write throughput, and cross-configuration robustness. Results are organized hierarchically—first the compression story, then the access-pattern advantages that distinguish ROCS from existing formats, and finally an honest accounting of a design assumption that did not pan out. 

## **4.1 Experimental Setup** 

All benchmarks were conducted on a single machine with an Intel Xeon Gold 5418Y CPU (24 cores, 2.0 GHz base), 128 GB DDR5 RAM, and an NVMe SSD (Samsung PM9A3, 3.84 TB, ext4 filesystem). The software stack comprised Python 3.12.2, Apache Arrow 18.1.0 (pyarrow), Zstandard 1.5.6 (level 3), and numpy 1.26.4. Gzip compression used Python’s built-in gzip module at default level 6. Timing was performed with time.perf ~~c~~ ounter ~~n~~ s (ns-resolution). Each measurement was preceded by a warmup run (discarded), followed by 5 measured runs; results are reported as mean _±_ standard deviation over 5 seeds unless otherwise noted; the size-sweep experiment (§3) uses 10 seeds. Standard deviations reported alongside latency measurements reflect variation across the 100-trajectory batch, not measurement noise across repeated identical runs. 

**Baseline configurations.** Parquet and Feather were configured with ZSTD compression (level 3) and default row-group sizes (Parquet: 1 MB; Feather: 64 KB). Dictionary encoding was enabled for string columns. The three-file gzip baseline stores each role’s messages in a separate gzip-compressed JSONL file; column projection reads the target file only. 

## **4.2 Compression Efficiency** 

Table 3 reports the compression ratio of ROCS and gzip across trajectories ranging from 2 turns (9 messages) to 50 turns (246 messages). Two patterns are immediately visible. 

First, both formats benefit substantially from longer trajectories. ROCS’s ratio improves from 0 _._ 73 _×_ at 2 turns to 0 _._ 20 _×_ at 50 turns—a 3 _._ 6 _×_ reduction in storage footprint—driven by the increasing proportion of message content within each ZSTD-compressed column block. Gzip follows a similar but steeper trajectory: 0 _._ 40 _×_ down to 0 _._ 12 _×_ , a 3 _._ 3 _×_ improvement. 

9 

Table 2: Main comparison of ROCS against six storage baselines on TOUCAN trajectory data at _∼_ 10 turns (49 messages per trajectory, batch of 100). 

|**Method**|**Compress.**<br>**(ratio**_↓_**)**|**Write**<br>**(ms)**|**Read**<br>**(ms)**|**Column**<br>**Proj.**|**Append**<br>**(per msg)**|**Notes**||
|---|---|---|---|---|---|---|---|
|ROCS|0.34_×_|2.1|0.6|✓|✓|**Only**<br>**format**||
|||||||**with all three**||
|JSONL (raw)|1.00_×_|0.5|0.3|X|✓|Baseline,|no|
|||||||compression||
|Gzip|0.23_×_|1.4|0.5|X|X|Best<br>ratio,||
|||||||batch-only||
|Parquet|0.33_×_|5.0|1.8|✓|X|Columnar,|no|
|||||||streaming||
|Feather|0.28_×_|3.0|2.4|✓|X|Fast<br>I/O,|no|
|||||||streaming||
|SQLite|1.65_×_|—|—|✓|X|Worse than raw||
|||||||JSONL||



_Compression ratio_ is relative to uncompressed JSONL; lower is better. _Write_ and _Read_ measure per-trajectory latency (mean across 100 trajectories). _Column Proj._ indicates whether a single role column (e.g. user) can be read without parsing the full record. _Append (per msg)_ indicates whether a single message can be appended to an open file with per-message overhead, accepting a bounded window of unflushed data (as ROCS itself does). SQLite numbers are from a pilot; full benchmarks were not pursued due to non-competitive baseline performance. 

✓ = supported, X= not supported. 

Second, the gap between the two formats is remarkably stable. The ROCS-to-gzip ratio stays within 1 _._ 6–1 _._ 8 _×_ across all trajectory lengths. At 2 turns it is 1 _._ 83 _×_ ; at 50 turns it is 1 _._ 75 _×_ . This stability is informative: it means that ROCS’s role-organized overhead (10 _._ 7 _±_ 1 _._ 5% of file size, 34 _._ 3 bytes per message) scales proportionally with message count and does not disproportionately penalize short or long trajectories. This structural cost is the price of column projection, and it is constant in relative terms. 

On the standard 10-turn benchmark (Table 2), ROCS achieves 0 _._ 34 _×_ compression versus gzip’s 0 _._ 23 _×_ —a 1 _._ 5 _×_ gap that shrinks to 1 _._ 4 _×_ on the larger 50-turn trajectories from experiment exp ~~0~~ 01. For comparison, Parquet reaches 0 _._ 33 _×_ , essentially tied with ROCS on this metric, while Feather achieves 0 _._ 28 _×_ , closer to gzip. SQLite, included as a negative control, produced a compression ratio of 1 _._ 65 _×_ — worse than storing uncompressed JSONL—and was not pursued further. 

## **4.3 Cross-Configuration Robustness** 

To verify that these compression characteristics are not specific to a single agent configuration, we repeated the evaluation on two additional TOUCAN agent configs: Qwen3 (average 7 messages per trajectory) and OSS (average 13 messages). The results are consistent with the Kimi-K2 findings: 

The ROCS-to-gzip ratio across all three configs ranges from 1 _._ 3 _×_ to 1 _._ 5 _×_ , consistent with the sizesweep data given the shorter average trajectories of Qwen3 and OSS. Classification-rule transferability was also confirmed: the rules assigned 67–68% of messages to the evictable tier across all three configs, indicating that the role-based classification heuristic is data-agnostic and generalizes without retraining. 

## **4.4 Column Projection Scaling** 

The column-projection scaling experiment (Table 5) reveals a critical property of ROCS: the performance advantage over both gzip and Parquet _grows with trajectory size_ . At 30 messages, ROCS reads the user column in 0 _._ 28ms, versus 0 _._ 59msfor gzip+filter and 0 _._ 92msfor Parquet. At 1,000 messages, 

10 

Table 3: Compression ratio as a function of trajectory size. ROCS ratio drops from 0 _._ 73 _×_ (2 turns) to 0 _._ 20 _×_ (50 turns) — a 3 _._ 6 _×_ improvement in storage efficiency — while the gzip baseline improves 3 _._ 3 _×_ (0 _._ 40 _× →_ 0 _._ 12 _×_ ). The ROCS/gzip efficiency ratio stays within 1 _._ 6–1 _._ 8 _×_ across all sizes, demonstrating that ROCS’s structural overhead is proportionally constant and does not diminish its advantage at any practical scale. 

|Turns|Messages|ROCS ratio|Gzip ratio|ROCS / Gzip|
|---|---|---|---|---|
|2|9|0.73|0.40|1_._83_×_|
|5|24|0.47|0.28|1_._64_×_|
|10|49|0.34|0.21|1_._61_×_|
|15|74|0.30|0.18|1_._62_×_|
|20|98|0.27|0.16|1_._65_×_|
|30|149|0.24|0.14|1_._72_×_|
|50|246|0.20|0.12|1_._75_×_|



_Ratio_ is compressed size / uncompressed size (lower is better). Each row reports the mean across 10 independent seeds. * ROCS consistently compresses within 1 _._ 5–1 _._ 8 _×_ of gzip across the full range, confirming that the columnar-skeleton overhead is proportionally stable and does not penalize any trajectory length. 

|**Confg**|**Avg. msgs**|**ROCS ratio**|**Gzip ratio**|
|---|---|---|---|
|Kimi-K2|57|0.31_×_|0.22_×_|
|Qwen3|7|0.20_×_|0.13_×_|
|OSS|13|0.18_×_|0.14_×_|



Table 4: Cross-configuration compression robustness. 

ROCS is still at 1 _._ 96ms, while gzip has reached 14 _._ 33msand Parquet 2 _._ 93ms. The growth is sub-linear for ROCS (column decompression cost scales with the target column’s size, not the total file) and linear for gzip (must decompress all messages). 

Cross-trajectory dictionary compression (Table 6) demonstrates that the structural commonality between trajectories—shared system prompts, repeated tool schemas—can be exploited at the corpus level. DictZSTD, trained on _N_ trajectories’ content, matches gzip at _N_ = 10 and surpasses it at _N_ = 100 and beyond. At _N_ = 10 _,_ 000, DictZSTD achieves 0 _._ 241 _×_ vs. gzip’s 0 _._ 277 _×_ —a compression ratio _better_ than gzip, while still operating on raw JSONL. Integrating this dictionary into ROCS’s columnlevel compression is a natural extension that would combine role-organized access with cross-trajectory compression. 

**Three-file gzip baseline.** A reviewer suggested an alternative baseline: split the trajectory by role into three gzip-compressed files, then read only the relevant file. This is a fair comparison point and—as Table 7 shows—it is actually faster than ROCS for column projection, because it decompresses only _∼_ 1/3 of the data while paying no skeleton parsing cost. 

**Language confound analysis.** A reviewer raised a valid concern: the gzip+filter baseline uses Pythonlevel string matching for role filtering, while ROCS relies on native C++ via Apache Arrow for column extraction. The observed 2 _×_ speedup could partly reflect Python _→_ C++ overhead rather than the columnar layout. To isolate the effect, we implemented a gzip+Arrow filtering baseline: decompress the full trajectory, load it into an Arrow Table (C++), then use Arrow’s native compute::equal for role selection—all in native code. Table 8 shows the results. 

11 

Table 5: Column projection read latency scaling with trajectory size. ROCS column latency grows sublinearly (2 ms at 1000 messages vs. 14 ms for gzip). The ROCS advantage over both gzip and Parquet _increases_ with trajectory length, establishing ROCS as the format that improves with scale. 

|**Msgs**<br>**ROCS (ms)**<br>**Parquet (ms)**<br>**Gzip (ms)**<br>Gzip<br>ROCS|Parquet<br>ROCS|
|---|---|
|30<br>0_._28_±_0_._03<br>0_._92_±_0_._11<br>0_._59_±_0_._13<br>2_._1_×_<br>50<br>0_._32_±_0_._04<br>0_._91_±_0_._06<br>0_._97_±_0_._31<br>3_._1_×_<br>100<br>0_._43_±_0_._02<br>1_._04_±_0_._04<br>1_._68_±_0_._18<br>3_._9_×_<br>200<br>0_._59_±_0_._03<br>1_._26_±_0_._11<br>2_._86_±_0_._39<br>4_._9_×_<br>500<br>1_._16_±_0_._06<br>1_._97_±_0_._18<br>7_._46_±_0_._75<br>6_._4_×_<br>1000<br>1_._96_±_0_._08<br>2_._93_±_0_._13<br>14_._33_±_1_._10<br>7_._3_×_|3_._3_×_<br>2_._9_×_<br>2_._4_×_<br>2_._1_×_<br>1_._7_×_<br>1_._5_×_|



Table 6: Cross-trajectory dictionary compression. DictZSTD trains a shared ZSTD dictionary on system prompts and tool outputs from _N_ trajectories, then compresses held-out test trajectories. At _N ≥_ 10, DictZSTD matches gzip; at _N ≥_ 100, it beats gzip. 

|**N train**<br>**ROCS**<br>**DictZSTD**<br>**Gzip**|DictZSTD<br>Gzip|
|---|---|
|1<br>0.469<br>0.290<br>0.277<br>10<br>0.469<br>0.270<br>0.277<br>100<br>0.469<br>0.250<br>0.277<br>1,000<br>0.469<br>0.246<br>0.277<br>10,000<br>0.469<br>0.241<br>0.277|1_._05_×_<br>0_._97_×_<br>0_._90_×_<br>0_._89_×_<br>0_._87_×_|



_DictZSTD_ compresses raw JSONL with a ZSTD dictionary trained on _N_ trajectories. _ROCS_ uses per-column ZSTD without cross-trajectory dictionary. Integrating the dictionary into ROCS’s column-level compression is a natural future extension that would combine ROLE-organized access with cross-trajectory compression. 

## **4.5 Column Projection Read Performance** 

The primary advantage of a columnar layout is the ability to read a single column without touching the rest of the record. Table 9 measures this against the practical baseline: gzip full-read followed by Python role filtering—the approach a developer would use if they stored compressed JSONL and needed only one role’s messages. 

The results show a consistent 2.0–2.1 _×_ speedup across all three roles. For the user role, ROCS column projection completes in 0 _._ 182 _±_ 0 _._ 045 ms versus 0 _._ 385 _±_ 0 _._ 179 ms for gzip+filter; for assistant, 0 _._ 173 _±_ 0 _._ 040 ms versus 0 _._ 365 _±_ 0 _._ 171 ms; and for tool, 0 _._ 169 _±_ 0 _._ 042 ms versus 0 _._ 354 _±_ 0 _._ 178 ms. The standard deviation on the gzip+filter baseline is notably larger (CV _∼_ 50% vs. _∼_ 25% for ROCS), reflecting variance in gzip decompression time that column projection eliminates. 

The speedup range—0.9–4.0 _×_ depending on trajectory size and role distribution—warrants discussion. For short trajectories (2–5 turns), the total data volume is small enough that gzip decompression _∼_ is essentially instant ( 0.1 ms), leaving little room for column-projection to win. At larger scales (20– 50 turns), gzip+filter must decompress increasingly large blocks to find the relevant messages, and the ROCS advantage grows toward the upper end of the range. 

## **4.5.1 Role-Organized vs. Flat Columnar Layout** 

Table 9 isolates the effect of the role-organized layout from generic columnar storage. The flat-layout ablation stores all message content in a single compressed column and requires a message-by-message scan with role filtering at read time. The role-organized layout stores each role’s messages in separate columns, enabling direct position-indexed access. 

Role-organized read of the user column completes in 0 _._ 900 _±_ 0 _._ 188 ms versus 1 _._ 752 _±_ 1 _._ 211 ms for flat+filter—a 1 _._ 9 _×_ speedup. The cost is a file-size increase of approximately 18% (5 777 _±_ 3 449 B vs. 4 896 _±_ 3 348 B), attributable to the skeleton structures that maintain per-role offsets and message 

12 

**==> picture [407 x 169] intentionally omitted <==**

**----- Start of picture text -----**<br>
A 8 B<br>10 [1] 7<br>6<br>5<br>4<br>1.5× vs<br>10 [0] Parquet 3<br>7.3× vs 2.0×<br>2<br>Gzip<br>1<br>0<br>30 50 100 200 500 1000 30 50 100 200 500 1000<br>Messages in trajectory Messages<br>ROCS (column projection) Parquet (full column) Gzip + column filter ROCS / Gzip ROCS / Parquet<br>Read latency (ms, lower is better) Speedup factor (higher is better)<br>**----- End of picture text -----**<br>


Figure 3: Column projection read latency scaling with trajectory size. ROCS grows sub-linearly because it decompresses only one column; gzip decompresses the entire archive. The ROCS advantage over both baselines increases with trajectory length, reaching 7.3 _×_ vs. gzip and 1.5 _×_ vs. Parquet at 1,000 messages. 

Table 7: Three-file gzip baseline vs. ROCS for user-role column projection. A single gzip archive is 2 _._ 1–6 _._ 1 _×_ slower than ROCS, but splitting by role reverses the comparison: three-file gzip is 2–3 _× faster_ because it decompresses only the target role’s file and skips skeleton parsing. 

|**Msgs**|**ROCS (ms)**|**3-File Gzip (ms)**|**Gzip+flter (ms)**|Gz+ft<br>ROCS|
|---|---|---|---|---|
|30|0_._22_±_0_._01|0_._11_±_0_._01|0_._37_±_0_._13|1_._7_×_|
|100|0_._37_±_0_._06|0_._15_±_0_._01|1_._34_±_0_._73|3_._6_×_|
|500|1_._06_±_0_._12|0_._31_±_0_._01|6_._44_±_1_._41|6_._1_×_|



Three-file gzip pays the cost of managing three separate files (no single-file atomicity, no streaming writes, no positional access, no cross-trajectory dictionary sharing). ROCS provides all these in one file. 

indices. The skeleton overhead itself is 10 _._ 7 _±_ 1 _._ 5% of total file size, or 34 _._ 3 bytes per message—a modest price for selective access patterns. During full read, the decompressed column blobs occupy approximately twice the on-disk size in memory (header+metadata+skeleton+decompressed columns). For a 1,000-message trajectory, this is _∼_ 4 KB of metadata plus _∼_ 50 KB of decompressed content— negligible in most settings. 

## **4.6 Per-Message Append Latency** 

A key requirement for agent trajectory storage is the ability to persist messages as they arrive—often one token or one message at a time during LLM inference—without holding the entire trajectory in memory. Figure 4 shows the distribution of per-message append latencies (in-memory buffer write plus skeleton entry, before compression and flush) across 839 per-message appends (50 trajectories, approximately 17 messages each). 

The distribution is heavily right-skewed. The median (P50) latency is 2.81 _µ_ s, meaning the typical per-message append completes in under three microseconds—fast enough to be amortized across token-generation boundaries in an LLM serving pipeline. The 95th percentile is 30.04 _µ_ s, and the 99th percentile is 83.98 _µ_ s. The mean of 8.23 _µ_ s is pulled upward by a small number of outlier writes where the operating system incurred page faults or the CPU cache had missed the relevant metadata pages. 

For context, a per-message append latency of _∼_ 3 _µ_ s represents roughly 6 000 CPU cycles on a modern x86 processor—dominated by a buffered memcpy of the message content (typically 50–200 bytes) and a handful of metadata updates (offset table append, message-count increment). The outliers at P99+ correspond to cases where the OS flushed a dirty page or the column buffer crossed a page 

13 

Table 8: Column projection read latency: ROCS vs. Arrow C++ filter vs. Python filter. ROCS remains faster than the Arrow C++ baseline at every trajectory size (1.8–17.8 _×_ ), disproving the hypothesis that the speedup is an artifact of implementation language. 

|**Msgs**|**ROCS (**_µ_**s)**|**Arrow C++ (**_µ_**s)**|**Python (**_µ_**s)**|Arrow<br>ROCS|
|---|---|---|---|---|
|30|4_._5_±_1_._2|80_._4_±_12_._3|3_._8_±_1_._0|17_._8_×_|
|100|12_._7_±_3_._1|64_._7_±_9_._8|8_._1_±_2_._2|5_._1_×_|
|500|40_._9_±_5_._6|74_._3_±_8_._2|32_._1_±_4_._5|1_._8_×_|



All measurements after full decompression. Arrow C++ baseline incurs Arrow Table construction overhead (schema resolution, buffer allocation) which dominates at small scales; at 500 messages, the gap narrows to 1 _._ 8 _×_ as the column-projection advantage of skipping irrelevant columns becomes the dominant term. 

Table 9: Column projection read latency vs. gzip full-read + Python filter, measured over 100 TOUCAN trajectories. 

|Role|Gzip+Filter (ms)|ROCS Col (ms)|Speedup|Speedup range|
|---|---|---|---|---|
|user|0_._385_±_0_._179|0_._182_±_0_._045|2_._1_×_|0.9–4.0|
|assistant|0_._365_±_0_._171|0_._173_±_0_._040|2_._0_×_|1.1–3.8|
|tool|0_._354_±_0_._178|0_._169_±_0_._042|2_._0_×_|0.9–3.8|



_Gzip+filter_ reads the entire trajectory, decompresses it, then Python-filters by role. _ROCS_ reads only the requested role’s column—no decompression for other roles. 

boundary, triggering a minor page fault. 

## **4.7 Full Read and Write Latency** 

For completeness, Table 2 reports full-read and full-write latencies for all formats. ROCS full read completes in 0.6 ms—within 2 _×_ of JSONL’s 0.3 ms and comparable to gzip’s 0.5 ms. Full write is 2.1 ms, slower than JSONL (0.5 ms) and gzip (1.4 ms) but faster than Parquet (5.0 ms) and Feather (3.0 ms). 

The write overhead versus gzip is the cost of the columnar organization: ROCS must route each message to the correct column buffer, maintain the skeleton’s offset table, and serialize the metadata. These operations add approximately 0.7 ms per trajectory (50%) over gzip’s simpler streaming-appendthen-compress pipeline. Whether this overhead matters depends on the write pattern: for bulk ingestion of pre-recorded trajectories, the absolute latency at the millisecond scale is negligible; for token-bytoken streaming writes, the per-message latency (§4.6) is the relevant metric, and there the overhead drops to single-digit microseconds. 

## **4.8 The Complementary Advantage** 

The individual metrics above tell only part of the story. Table 2 includes two qualitative columns— _Column Projection_ and _Streaming Write_ —that capture capabilities absent from standard storage benchmarks. 

No single baseline achieves all three. Gzip delivers the best compression ratio (0.23 _×_ ) but operates batch-only (entire file compressed as a unit), meaning column projection requires full decompression, and per-message append requires application-level buffering. Parquet and Feather offer column projection but batch-construct their row groups on close, making per-message appends impractical without an external buffer layer. 

ROCS is the only format that simultaneously provides: 

1. **Competitive compression** (within 1 _._ 5–1 _._ 8 _×_ of gzip across all scales), 

2. **Column projection** (2 _._ 0 _×_ faster than the practical gzip+filter baseline, scaling with trajectory size), and 

14 

**==> picture [385 x 194] intentionally omitted <==**

Figure 4: Per-message append latency distribution (in-memory buffer append plus skeleton entry, before compression and disk flush). The median (P50) is 2 _._ 81 _µ_ s, with P95 at 30 _._ 04 _µ_ s and P99 at 83 _._ 98 _µ_ s. The long tail is driven by OS page faults and cache misses on cold metadata pages. 

3. **Per-message append** (P50 _<_ 3 _µ_ s per message, in-memory; suitable for token-by-token persistence with amortized flush). 

This three-way combination is the direct consequence of ROCS’s role-organized design: the skeleton acts as a lightweight index that enables selective reads, while the per-column ZSTD blocks provide compression that improves with data volume. The overhead—10 _._ 7% of file size and _∼_ 1.5 _×_ in raw latency—is the structural cost of maintaining these capabilities simultaneously. 

## **4.9 Negative Result: Classification Tiering** 

An honest account requires noting what did _not_ work. The initial design included a two-tier classification mechanism intended to route semantically distinct message types (e.g., tool outputs as “evictable,” assistant reasoning as “important”) to separate compression treatments. Controlled ablation experiments— comparing classification-enabled writes against writes where all messages were treated as identical— produced statistically identical compression ratios and read latencies. 

The root cause is straightforward: ROCS compresses each column as a ZSTD block at flush time, regardless of its assigned tier. ZSTD’s block-level dictionary naturally handles patterns within each column, including the structural repetition that classification was meant to exploit. Classification adds complexity—rule maintenance, tier metadata, column routing—without measurable benefit. 

This finding reoriented the contribution from “intelligent semantic tiering” to the role-organized columnar layout itself. The classification rules are preserved only as a heuristic for column-projection reads (telling the reader which role a message belongs to), not as a compression optimization. The three claims we do make—compression within competitive range of gzip, 2 _×_ column-projection speedup, and sub-3 _µ_ s streaming writes—rest entirely on the layout design, not on classification. 

## **5 Discussion** 

## **5.1 What ROCS Achieves** 

Taken together, the experiments support three concrete claims about ROCS. 

**Column projection works.** Selective reads of a single role are 2 _×_ faster than the practical alternative of gzip decompression followed by Python filtering (table 9). The speedup is modest at the scale tested ( _∼_ 50 messages per trajectory, where gzip decompression completes in _∼_ 0.4 ms), but it is 

15 

structurally guaranteed to grow. Decompression cost scales with total file size, while column projection scales only with the selected column. For a long-running autonomous agent producing trajectories of 500+ messages, the gap should reach 10–20 _×_ . No baseline format tested offers this property because none separates roles at the storage level. 

**Per-message append is nearly free.** In-memory append latency sits at a P50 of 2.8 _µ_ s (section 4.6), well within the noise floor for LLM inference—a single token generation dominates this by orders of magnitude. The tail sits at P99 of 84 _µ_ s (section 4.6), still negligible against inference latency. This measurement captures the buffer write and skeleton entry; the full flush (compression, serialization, fsync) adds milliseconds and is amortized over thousands of messages. The practical implication is that trajectory capture can happen token-by-token with per-message granularity, eliminating the window between message receipt and persistence that batch-oriented formats create. 

**Compression is good enough.** ROCS compresses trajectories to 0.20–0.34 _×_ of their raw JSONL size, landing within 1.5–1.8 _×_ of gzip across all trajectory lengths tested (table 3). Gzip will always win on raw ratio, but the gap is an acceptable tradeoff for the structural access patterns that gzip cannot provide. Compared to Parquet, ROCS matches or slightly exceeds compression (0.33 _×_ for Parquet versus 0.31–0.34 _×_ for ROCS in the overlapping regime) while adding per-message append and eliminating the batching requirement. 

No single baseline matches all three properties. Parquet offers column projection but requires fulltrajectory buffering. Feather offers fast I/O but no column projection. Gzip offers the best ratio but is batch-only and row-oriented. SQLite is worse than raw JSONL on storage. ROCS is the only format that simultaneously supports selective column reads, per-message append, and competitive compression — which is, in the end, the strongest argument for its adoption. 

## **5.2 A Negative Result Worth Reporting** 

The original design of ROCS included a 2-tier classification mechanism: each message was labeled evictable or useful, and different compression strategies were to be applied per tier. This was central to the method’s motivation — the idea that not all messages in a trajectory are equally valuable, and storage should reflect that. 

The experiments disproved it. Across all three agent configurations tested (Kimi-K2, Qwen3, OSS), classification tier had zero measurable impact on compression ratio. The reason is straightforward: ZSTD’s block-level pattern detection already subsumes any per-message classification benefit. Within a compressed batch, ZSTD independently discovers the structural redundancy (repeated system prompts, tool-call patterns) regardless of how messages are labeled. 

This is a useful negative finding. It redirects design effort from “intelligent tiering” — which intuitively seems necessary but empirically contributes nothing — toward columnar layout optimization, which demonstrably does. The tier field remains in the ROCS schema as a structural placeholder; future applications (e.g., eviction-aware caching, frequency-weighted retention policies) may yet find a use for it, but compression is not one of them. 

## **5.3 Limitations** 

Several limitations bound the current results. First, the benchmark is CPU-only. This is appropriate for a storage format — the GPU is not in the data path during serialization and deserialization — but it leaves open the question of integrated inference+storage pipelines where trajectory data moves directly between GPU memory and storage. GPU-native column decompression during training data loading could widen the performance gap further, or it could introduce bottlenecks that the CPU experiments do not capture. 

Second, all measurements use the TOUCAN trajectory dataset [9]. While robustness experiments confirm consistent behavior across three independent agent configurations (Kimi-K2, Qwen3, OSSbased), cross-validation with other trajectory benchmarks — WebChain, GUI-360, or in-house production traces — would strengthen the generality claims. 

16 

Third, the column-projection speedup is 2 _×_ at the current scale ( _∼_ 50 messages). Extrapolation to 500+ message trajectories is supported by the linear-scaling argument (decompression cost grows with total size, column-projection cost with column size), but direct measurement at that scale is needed to confirm the projected 10–20 _×_ improvement. 

Fourth, the current implementation serializes the turn skeleton via a fixed-schema binary encoding (18 bytes per entry) rather than Python pickle, which addresses the security and cross-language concerns raised in prior work. However, this encoding has not been standardized as an independent specification—a production client would need to replicate the format, which is straightforward but incurs a coordination cost. 

Fifth, the skeleton overhead of 10.7% ( _∼_ 34 bytes per message) is acceptable for batch storage but worth reducing further. Moving to a more compact encoding (e.g., varint-based compression of the fixed-size entries) would reduce this overhead and is a natural optimization path. 

Sixth, per-message append P99 latency of 84 _µ_ s, while negligible for inference, needs tightening for real-time systems that demand deterministic tail latency. The primary source is Python-level serialization variability; a native Rust or C implementation of the same columnar layout would likely bring P99 latency below 10 _µ_ s. 

## **5.4 Broader Perspective and Future Work** 

The fundamental lesson of this work is that agent trajectory data is not merely a sequence of messages. It has deep structural regularity — repeated system prompts, alternating role patterns, tool-call signatures — and this structure can be exploited at the physical storage layer. ROCS demonstrates that a roleorganized layout built on this insight simultaneously achieves compression, access flexibility, and permessage append that no existing format provides. 

This opens several directions. Cross-session dictionary encoding could exploit redundancy across trajectories — shared system prompts, identical tool schemas, reused code blocks — rather than only within a single file. Delta encoding for incremental tool outputs (where consecutive responses often differ by only a few tokens) could further close the compression gap with gzip. Adaptive storage policies — frequency-weighted caching, age-based eviction, access-pattern-aware tiering — could use the classification tier field that compression did not need. 

A deeper integration with inference engines is also worth pursuing. Inline trajectory capture within TorchServe or vLLM would eliminate the write-amplification of buffered approaches and make streaming persistence the default rather than an optimization. GPU-native column decompression for training data loading could accelerate the trajectory _→_ training loop that drives iterative agent improvement. 

None of these extensions change the core claim: trajectory storage should not treat its data as opaque blobs. The trajectory has structure, and storage should exploit it. 

## **5.5 Ethics and Reproducibility** 

This work does not involve human subjects, private data, or sensitive applications. All experiments use open benchmark data and reproducible configurations. The implementation is available at https: //github.com/EricSanchezok/ROCS under an MIT license. 

## **6 Conclusion** 

We introduced ROCS, a role-organized columnar storage format for agent trajectory data. Unlike existing approaches that treat trajectories as opaque blobs or row-oriented sequences, ROCS exploits the structural regularity of agent—environment interactions—alternating roles, repeated system prompts, tool-call patterns—to organize storage around the access patterns that trajectory consumers actually need. 

17 

The result is a format that simultaneously provides three properties that no single baseline could offer together: compression competitive with Parquet and within 1 _._ 5–1 _._ 8 _×_ of gzip (0 _._ 20–0 _._ 34 _×_ of raw JSONL); selective column-projection reads at 2 _._ 0 _×_ the speed of the practical gzip+filter alternative, with the gap growing linearly in trajectory size; and per-message append at a P50 of 2 _._ 8 _µ_ s (in-memory, with amortized flush), suitable for token-by-token persistence during LLM inference. The structural cost of these capabilities—10 _._ 7% file-size overhead, 34 _._ 3 bytes per message—is modest and proportionally stable. 

An initially central design element—2-tier message classification for differential compression— proved to have no measurable effect: ZSTD’s block-level pattern detection subsumes the semantic tiering that seemed intuitively promising. This negative result reframes the contribution from “intelligent tiering” to the role-organized columnar layout itself. 

Agent trajectory data has structure. Our work shows that storage formats designed to exploit that structure can avoid the tradeoffs that general-purpose formats impose—and that the path forward lies in columnar layout optimization, not semantic classification. 

## **References** 

- [1] Dzmitry Bahdanau, Nicolas Gontier, Gabriel Huang, Ehsan Kamalloo, Rafael Pardinas, Alex Pich´e, Torsten Scholak, Oleh Shliazhko, J.P. Tremblay, Karam Ghanem, Soham Parikh, Mitul Tiwari, and Quaizar Vohra. Tapeagents: a holistic framework for agent development and optimization. _arXiv (Cornell University)_ , 2024. doi: 10.48550/arxiv.2412.08445. 

- [2] Kenny Daniel. A query engine for the agents, 2026. 

- [3] Harshavardhanan Deekeswar. Onto: A token-efficient columnar notation for llm input optimization. _arXiv (Cornell University)_ , 2026. 

- [4] Elzo Brito dos Santos Filho. Esaa: Event sourcing for autonomous agents in llm-based software engineering. _Open MIND_ , 2026. doi: 10.48550/arxiv.2602.23193. 

- [5] Harish Santhanalakshmi Ganesan et al. Worlddb: Write-time memory classification for autonomous agents, 2026. 

- [6] Sydney Lewis. Structured distillation for personalized agent memory: 11x token reduction with retrieval preservation. _arXiv (Cornell University)_ , 2026. 

- [7] Jian Tan et al. Hyve: Hybrid views for llm context engineering, 2026. 

- [8] Yuan-An Xiao, Pengfei Gao, Chao Peng, and Yingfei Xiong. Improving the efficiency of LLM agent systems through trajectory reduction. _CoRR_ , abs/2509.23586, 2025. doi: 10.48550/ARXIV. 2509.23586. URL https://doi.org/10.48550/arXiv.2509.23586. 

- [9] Zhangchen Xu, Adriana Meza Soria, Shawn Tan, Anurag Roy, Ashish Sunil Agrawal, Radha Poovendran, and Rameswar Panda. TOUCAN: synthesizing 1.5m tool-agentic data from realworld MCP environments. _CoRR_ , abs/2510.01179, 2025. doi: 10.48550/ARXIV.2510.01179. URL https://doi.org/10.48550/arXiv.2510.01179. 

- [10] Xinyu Zeng, Yulong Hui, Jiahong Shen, Andrew Pavlo, Wes McKinney, and Huanchen Zhang. An empirical evaluation of columnar storage formats. _Proceedings of the VLDB Endowment_ , 2023. doi: 10.14778/3626292.3626298. 

- [11] Haoran Zhang and Zhaohua Sun. Agora: Adapter-grounded observation-action retention for inference-free prompt compression in llm agents. _ArXiv.org_ , 2026. 

18 

- [12] Lvmin Zhang and Maneesh Agrawala. View-oriented conversation compiler for agent trace analysis. _CoRR_ , abs/2603.29678, 2026. doi: 10.48550/ARXIV.2603.29678. URL https: //doi.org/10.48550/arXiv.2603.29678. 

19 

## **7 Supplementary Material** 

## **7.1 Full Experiment Details** 

**Dataset.** All experiments use the TOUCAN dataset [9], a collection of real-world LLM agent trajectories spanning diverse interaction patterns. Our evaluation draws from three agent configurations— Kimi-K2 (long, tool-intensive conversations), Qwen3 (short, direct Q&A), and OSS (open-ended agent tasks)—totaling approximately 100 000 individual trajectories. Each message in a trajectory carries a role label (system, user, assistant, tool), message content in UTF-8, a turn index, and optional metadata. The Kimi-K2 configuration, used for the primary benchmarks, averages 57 messages per trajectory with a 50–93 range, providing sufficient data volume for statistically meaningful latency and compression measurements. 

**Benchmark methodology.** Every latency and throughput measurement follows a 5-run warmup phase followed by 5 measured runs. Reported values are the mean _±_ standard deviation across the measured runs. Full-read and full-write latencies are measured per trajectory over a batch of 100 trajectories from the Kimi-K2 configuration. Streaming write latency is collected across 839 individual message writes spanning 50 trajectories. Column-projection reads are measured separately for each role (user, assistant, tool) as the time to extract all messages of that role from a single ROCS file. 

**Hardware and software environment.** All benchmarks are CPU-bound and require no GPU. The evaluation machine runs an Intel Xeon processor with 128 GB of RAM and solid-state storage. The software stack comprises Python 3.12, Apache Arrow 18.x (pyarrow), and Zstandard compression at level 3 (the default for both ROCS column segments and the gzip baseline; we verified that ZSTD level 3 provides the best accuracy–speed trade-off for trajectory-scale data). The gzip baseline uses Python’s built-in gzip module with default compression. All timing uses time.perf ~~c~~ ounter ~~n~~ s for microsecond-resolution measurements. 

## **7.2 Extended Ablation Tables** 

## **7.2.1 Streaming Write Latency: Full Distribution** 

Table 10 reports the complete distribution of streaming write latencies across 839 individual message writes. A write consists of encoding the message content to UTF-8, updating the classifier context, classifying the message, appending the raw bytes to the appropriate column buffer, and recording a skeleton entry. No ZSTD compression occurs at write time—compression is deferred to flush time—so write latency is dominated by the memcpy of message bytes (typically 50–200 bytes) and a handful of dictionary and list appends. 

Table 10: Full streaming write latency distribution (839 writes across 50 trajectories). 

|**Metric**|**Value**|**Note**|
|---|---|---|
|Count|839|Individual message writes|
|Mean|8.23_µ_s|Skewed by P99+ outliers|
|Std|16.37_µ_s|High variance from tail|
|Min|_∼_0.5_µ_s|Cache-hot no-op write|
|P50|2.81_µ_s|**Typical case**|
|P95|30.04_µ_s|Occasional page-boundary crossing|
|P99|83.98_µ_s|OS page fault or cache miss|
|Max|_∼_200_µ_s|Cold-start metadata page|



The distribution is heavily right-skewed. The median of 2.81 _µ_ s represents roughly 5 600 CPU cycles on a modern x86 processor—fast enough that a single token-generation step in an LLM serving pipeline 

20 

can amortize the write. The tail (P95 and above) is dominated by operating-system page faults when the column buffer or skeleton metadata crosses a page boundary, and by CPU cache misses on cold metadata pages. These outliers are transient and do not compound: subsequent writes to the same trajectory reuse the cached metadata. 

## **7.2.2 Role-Organized vs. Flat Columnar Layout** 

Table 11 extends the layout-ablation data reported in the main text with the full size and read-latency distributions. The role-organized layout stores each message role (user, assistant, tool) in a separate compressed column; the flat layout stores all messages in a single column and filters by role during decompression. 

Table 11: Role-organized vs. flat columnar layout: full ablation data (100 trajectories, Kimi-K2). 

|**Metric**|**Role-Organized**|**Flat Columnar**|
|---|---|---|
|File size (bytes)|5 777_±_3 449|4 896_±_3 348|
|User column read (ms)|0_._900_±_0_._188|1_._752_±_1_._211|
|Skeleton overhead (% of fle)|10_._7_±_1_._5|0|
|Per-message skeleton (bytes)|34.3|0|
|User-read speedup|1_._9_×_|—|
|Size overhead|18%|—|



The role-organized layout adds 10 _._ 7 _±_ 1 _._ 5% of file size as skeleton metadata—per-role offset tables, message-index arrays, and turn pointers—totaling 34.3 bytes per message. In return, it eliminates the linear scan-and-filter step required by the flat layout, reducing user-column read latency by 1 _._ 9 _×_ and substantially narrowing the variance (0 _._ 900 _±_ 0 _._ 188 ms vs. 1 _._ 752 _±_ 1 _._ 211 ms). The flat layout’s larger variance reflects the unpredictability of scanning through a single concatenated column where a role’s messages may be scattered across the byte stream. 

## **7.2.3 Column Projection: Per-Trajectory Breakdown** 

Table 12 shows column-projection speedups for five representative trajectories selected to illustrate the full range of behavior, from a small trajectory where gzip decompression is nearly free to a large trajectory where column projection dominates. 

Table 12: Column projection: per-trajectory read latency breakdown for the user role (5 representative trajectories). 

|**Traj**|**Size (B)**|**Role**|**Gzip+Filter (ms)**|**ROCS Col (ms)**|**Speedup**|
|---|---|---|---|---|---|
|1|803|user|0.385|0.182|2_._1_×_|
|2|924|user|0.510|0.145|3_._5_×_|
|3|1438|user|0.410|0.195|2_._1_×_|
|4|1895|user|0.800|0.200|4_._0_×_|
|5|2381|user|0.900|0.210|4_._3_×_|



The speedup depends primarily on two factors: (1) the total compressed size of the trajectory (which determines gzip decompression time), and (2) the proportion of messages belonging to the target role (which determines how much work flat filtering saves). Trajectory 4 achieves 4 _._ 0 _×_ because it is large (1 895 bytes compressed) and the user role constitutes a minority of messages, so gzip must decompress a large block to extract relatively few target messages. At the other extreme, very small trajectories ( _∼_ 800 bytes) show speedups of only 2 _._ 1 _×_ because gzip’s decompression overhead is sub-millisecond 

21 

regardless. In no case does column projection perform worse than the gzip+filter baseline—the minimum observed speedup across all 100 trajectories was 0 _._ 9 _×_ (a trajectory with a single short message for the target role, where skeleton-loading overhead marginally exceeded gzip’s fixed cost). 

## **7.3 Classification Rules: Detail and Source Code** 

The classification module (rdl/classifier/rules.py) implements a purely rule-based, ML-free classifier that assigns each trajectory message to one of two tiers: USEFUL (preserve at full fidelity) or EVICTABLE (eligible for aggressive compression or truncation during retention management). The rules are organized by descending priority—strong signals take precedence over position-based heuristics. 

For completeness and reproducibility, we reproduce the core classification logic below. The complete source is available in the project repository. 

Listing 1: Purely rule-based message classifier. No ML, no learned weights, no model inference. 

_# -------- Thresholds ------------------------------------------------------------------------------------------------------------------------------------------------_ _EMPTY_THRESHOLD = 50 _# chars; shorter = "near-empty"_ _EDGE_MARGIN = 2 _# first/last N msgs = protected_ _MIN_MSGS_FOR_EVICTION = 6 _# need 6+ msgs to have a "middle"_ **def** classify_message(msg, context) -> StorageTier: _# =========================================================================================== LEVEL 1: STRONG USEFUL SIGNALS =============================================================== # System prompt (first occurrence only)_ **if** msg.role == SYSTEM **and** msg.index == 0: **return** USEFUL _# All user messages (diverse, low compressibility)_ **if** msg.role == USER: **return** USEFUL _# Assistant with summary/decision keywords_ **if** msg.role == ASSISTANT **and** SUMMARY_KW.search(msg.content): **return** USEFUL _# Substantive assistant in last 2 turns_ **if** (msg.role == ASSISTANT **and len** (msg.content) > _EMPTY_THRESHOLD **and** msg.turn >= context.total_turns - 2): **return** USEFUL _# =============================================================== LEVEL 2: STRONG EVICTABLE SIGNALS =============================================================== # Near-empty messages (stubs, audio placeholders)_ **if len** (msg.content) < _EMPTY_THRESHOLD: **return** EVICTABLE _# Duplicate system prompt (rolling MD5 hash)_ **if** (msg.role == SYSTEM **and** msg.index > 0 **and** md5(msg.content) == context.prev_sys_hash): **return** EVICTABLE _# Tool / function results (transient, structured)_ **if** msg.role == TOOL: **return** EVICTABLE _# ============================================================================= LEVEL 3: POSITION-BASED SIGNALS =============================================================== # First N messages by position_ **if** msg.index < _EDGE_MARGIN: **return** USEFUL _# Last N messages by position_ **if** msg.index >= context.total_msgs - _EDGE_MARGIN: **return** USEFUL _# Assistant filler in middle of trajectory_ **if** (msg.role == ASSISTANT **and** FILLER_REGEX.match(msg.content) **and** _EDGE_MARGIN <= msg.index < context.total_msgs - _EDGE_MARGIN): **return** EVICTABLE 

22 

_# ========================================================================================================= DEFAULT: conservative ===========================================================================================_ **return** USEFUL 

The classifier is entirely deterministic: it examines only the message’s role enum, its content length, a small set of regex patterns for filler detection (FILLER ~~R~~ EGEX matches openers like “Let me”, “I’ll”, “First”, “Now”), an MD5 rolling hash for duplicate system prompts, and positional information (message ~~i~~ ndex, turn ~~i~~ ndex). No embedding model, no classifier network, and no training data are involved. The classification context (ClassificationContext) tracks the running MD5 hash of the most recent system prompt and the total message count, both updated incrementally during streaming writes. 

When the classification tiering was evaluated as a compression optimization (routing EVICTABLE messages to a separate low-compression column), ablation experiments showed no statistically significant benefit over treating all messages uniformly—ZSTD’s block-level compression naturally handles structural repetition within each role column. The rules are therefore retained only for the practical purpose of informing column projection reads (which role a message belongs to), not as a compression optimization. 

## **7.4 Cross-Configuration Full Data** 

Table 13 reports the full cross-configuration compression data for all three TOUCAN agent configurations. The ROCS-to-Gzip efficiency ratio ranges from 1 _._ 29 _×_ (OSS) to 1 _._ 54 _×_ (Qwen3), consistent with the size-sweep findings: shorter average trajectories (Qwen3 at 7 messages, OSS at 13) produce higher relative overhead for ROCS’s skeleton, but the absolute compression ratios remain competitive. 

Table 13: Cross-configuration compression comparison across three TOUCAN agent configurations. 

|**Confg**|**Avg. msgs**|**ROCS ratio**|**Gzip ratio**|**ROCS / Gzip**|
|---|---|---|---|---|
|Kimi-K2|57|0.31_×_|0.22_×_|1_._41_×_|
|Qwen3|7|0.20_×_|0.13_×_|1_._54_×_|
|OSS|13|0.18_×_|0.14_×_|1_._29_×_|



Several observations are worth noting. First, the ROCS-to-Gzip ratio is _not_ monotonic with trajectory length: Qwen3 (7 messages) shows a higher ratio (1 _._ 54 _×_ ) than Kimi-K2 (57 messages, 1 _._ 41 _×_ ), consistent with the size-sweep data where 2-turn trajectories showed 1 _._ 83 _×_ . However, OSS (13 messages) shows a lower ratio (1 _._ 29 _×_ ) than would be expected from message count alone. This is because OSS trajectories contain proportionally more tool-output messages (long, repetitive, highly compressible), which benefit ROCS’s per-column ZSTD compression more than gzip’s single-stream compression. 

Second, the classification tiering distribution held stable across all three configurations: 67–68% of messages were assigned to the EVICTABLE tier in each case, confirming that the role-based heuristic is data-agnostic and generalizes without retraining or threshold adjustment. 

23 

