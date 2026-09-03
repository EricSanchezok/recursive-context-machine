// seed-basic — captain 确定性脚手架的结构等价移植 (E1 种子脚本)。
//
// 沙箱: 无 IO/无时钟 (env 块不含 time 行 — 与 captain 的已知分歧, 断言 shape);
// 无状态: 同 obs 同动作 (ScriptedPolicy 每步全新 scope)。
// 每步只补第一件缺失项 (captain 同款: 一次 decide 至多发一个 Action);
// 脚手架完备后 done() 终态 — 不 Halt (Halt 会触发 completion, 无模型环境
// 下测试/演化回放均以 Done 为无成本退出)。

fn has_anchor(view, anchor) {
    for row in view.directory {
        if row.anchor == anchor {
            return true;
        }
    }
    false
}

fn has_tag(view, tag) {
    for row in view.directory {
        if row.tag == tag {
            return true;
        }
    }
    false
}

fn tool_active(view, name) {
    for active in view.active_tools {
        if active == name {
            return true;
        }
    }
    false
}

fn decide(view) {
    // 1. @agent 具名槽: 缺失才建。文本 = view.prompt
    //    (prompt_key 默认 "captain", 与 captain agent.rs 同源)。
    //    仅建不刷: view 只有 preview (80 字符截断), 全文等价无法比较。
    if !has_anchor(view, "@agent") {
        return set("@agent", view.prompt, "system", "agent");
    }

    // 2. @env 具名槽: cwd/platform/run_dir, 无 time 行 (沙箱无时钟)。
    if !has_anchor(view, "@env") {
        let text = "cwd: " + view.cwd + "\nplatform: " + view.platform;
        if view.run_dir != "" {
            text = text + "\nrun_dir: " + view.run_dir;
        }
        return set("@env", text, "system", "env");
    }

    // 3. purpose 尾插 (captain append 语义; 去重用 tag 存在性近似)。
    if view.purpose != "" && !has_tag(view, "purpose") {
        return insert_end(view.purpose, "user", "purpose");
    }

    // 4. 激活 model_order 首个 (captain resources::activate 同款)。
    if view.active_model == "" && view.model_order.len() > 0 {
        return model(view.model_order[0]);
    }

    // 5. 逐个激活未激活工具 (排序序 — captain 的 HashMap 序不确定,
    //    等价断言按集合)。
    for name in view.tool_names {
        if !tool_active(view, name) {
            return activate(name);
        }
    }

    // 6. 完备: 终态。
    done()
}
