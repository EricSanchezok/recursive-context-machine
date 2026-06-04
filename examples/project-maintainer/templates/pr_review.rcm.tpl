name = "PR Review ({{REPO}}#{{PR_NUMBER}})"

model kimi-k2-6 {
    protocol = "openai"
    endpoint = "https://api.kimi.com/coding/v1"
    credentials = { env = "KIMI_CODING_API_KEY" }
    headers = { User-Agent = "KimiCLI/1.5" }
    limit = { context = "262144", output = "32768" }
    modalities = { input = ["text"], output = ["text"] }
    thinking = "true"
}

graph {
    accelerator fetch {
        purpose = "拉取 PR #{{PR_NUMBER}} 来自 {{REPO}}:1) `gh pr view {{PR_NUMBER}} --repo {{REPO}} --json number,title,body,author,additions,deletions,changedFiles,files,labels,baseRefName,headRefName`;2) `gh pr diff {{PR_NUMBER}} --repo {{REPO}}` 获取完整 diff。如有需要可用 fs 工具读改前文件作对照。仅 fetch,不要修改任何文件。"
        models = ["kimi-k2-6"]
        policy = "captain"
        tools = ["shell", "fs"]
    }

    accelerator analyze {
        purpose = "基于 PR 元数据 + diff 内容,产出 review checklist。评估维度:1) 测试覆盖(是否新增/修改了 tests/,逻辑变更但无测试为 risk);2) diff 规模(small <100 行 / medium <500 行 / large ≥500 行);3) PR 描述质量(是否说清 why,只描述 what 为不充分);4) 安全嗅味(新依赖、用户输入处理、unwrap/panic、unsafe block、SQL/shell 拼接);5) 与基础规范的明显偏离(命名、错误处理、测试位置)。每项给一句话判断。不调用工具,不修改文件。"
        models = ["kimi-k2-6"]
        policy = "captain"
        tools = []
    }

    accelerator respond {
        purpose = "把 review 评论写成 markdown:开头一句总体判断(可合并/需调整);用 checklist 列出 analyze 的每项发现(正面 ✓ 负面 ⚠);末尾给 1-3 条具体修改建议。中文撰写,代码标识符保持英文。\n发评论只能用带引号 heredoc 经 --body-file 传入,严禁内联 -b——正文里的反引号、$、引号会被 shell 解释甚至注入:\ngh pr comment {{PR_NUMBER}} --repo {{REPO}} --body-file - <<'RCM_BODY'\n<在此写完整评论正文>\nRCM_BODY\n闭合标记 RCM_BODY 必须独占一行、行首顶格,且正文里不得出现这一行。\n发完必须自检确实发出去了:1) 上面命令要 exit 0 并打印评论 URL;2) 再跑 `gh pr view {{PR_NUMBER}} --repo {{REPO}} --json comments -q '.comments[-1].url'` 确认最新评论就是本次所发。若失败(如 fork PR 的只读 token 报 403、网络错误)原样重试一次;仍失败不要谎报成功。只用 shell+gh。\nhandoff 末尾:成功则写 status: ok 与 comment_url: <URL>;最终没发出去则写 status: blocked 与一行原因。"
        models = ["kimi-k2-6"]
        policy = "captain"
        tools = ["shell"]
    }

    flux fetch_to_analyze {
        channel = context
        mode = digest
        arity = 1
    }

    flux analyze_to_respond {
        channel = context
        mode = digest
        arity = 1
    }

    fetch.context -> fetch_to_analyze.slot(0)
    fetch_to_analyze.out -> analyze.context
    fetch.done -> analyze.trigger

    analyze.context -> analyze_to_respond.slot(0)
    analyze_to_respond.out -> respond.context
    analyze.done -> respond.trigger

    respond.done -> output.done
    respond.context -> output.context
    respond.purpose -> output.purpose
}
