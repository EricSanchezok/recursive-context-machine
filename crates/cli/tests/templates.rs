use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use cli::cmd::dispatch::{DispatchConfig, collect_substitutions, render_template};

fn project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .expect("workspace root")
        .to_path_buf()
}

fn load_dispatch_config() -> DispatchConfig {
    let config_path = project_root().join("examples/project-maintainer/dispatch.toml");
    let text = fs::read_to_string(&config_path).expect("read dispatch.toml");
    toml::from_str(&text).expect("parse dispatch.toml")
}

fn render_route(event_name: &str, action: Option<&str>, payload: serde_json::Value) -> String {
    let config = load_dispatch_config();
    let route = config
        .routes
        .iter()
        .find(|route| route.event == event_name && route.action.as_deref() == action)
        .or_else(|| {
            config
                .routes
                .iter()
                .find(|route| route.event == event_name && route.action.is_none())
        })
        .unwrap_or_else(|| panic!("no route for {event_name}/{action:?}"));

    let substitutions = collect_substitutions(route, &payload).expect("collect substitutions");

    let template_path = project_root()
        .join("examples/project-maintainer")
        .join(&config.templates_dir)
        .join(&route.template);
    let template_text = fs::read_to_string(&template_path).expect("read template");
    render_template(&template_text, &substitutions).expect("render template")
}

fn assert_rendered_parses(rendered: &str, label: &str) {
    cli::rcm::parse(rendered).unwrap_or_else(|err| {
        panic!("rendered {label} template failed to parse: {err}\n--- rendered ---\n{rendered}")
    });
}

fn assert_no_unrendered_placeholders(rendered: &str, label: &str) {
    if rendered.contains("{{") || rendered.contains("}}") {
        panic!("rendered {label} contains leftover braces:\n{rendered}");
    }
}

#[test]
fn dispatch_config_parses() {
    let config = load_dispatch_config();
    assert!(!config.routes.is_empty(), "dispatch.toml has no routes");
}

#[test]
fn issue_triage_template_renders_and_parses() {
    let payload = serde_json::json!({
        "action": "opened",
        "repository": {"full_name": "acme/widget"},
        "issue": {"number": 42, "title": "Crash on empty input"}
    });
    let rendered = render_route("issues", Some("opened"), payload);
    assert!(rendered.contains("acme/widget"));
    assert!(rendered.contains("#42") || rendered.contains("42"));
    assert_no_unrendered_placeholders(&rendered, "issue_triage");
    assert_rendered_parses(&rendered, "issue_triage");
}

#[test]
fn pr_review_template_renders_and_parses_on_opened() {
    let payload = serde_json::json!({
        "action": "opened",
        "repository": {"full_name": "acme/widget"},
        "pull_request": {"number": 7, "title": "Fix off-by-one"}
    });
    let rendered = render_route("pull_request", Some("opened"), payload);
    assert!(rendered.contains("acme/widget"));
    assert!(rendered.contains("7"));
    assert_no_unrendered_placeholders(&rendered, "pr_review (opened)");
    assert_rendered_parses(&rendered, "pr_review (opened)");
}

#[test]
fn pr_review_template_renders_and_parses_on_synchronize() {
    let payload = serde_json::json!({
        "action": "synchronize",
        "repository": {"full_name": "acme/widget"},
        "pull_request": {"number": 7}
    });
    let rendered = render_route("pull_request", Some("synchronize"), payload);
    assert_no_unrendered_placeholders(&rendered, "pr_review (synchronize)");
    assert_rendered_parses(&rendered, "pr_review (synchronize)");
}

#[test]
fn mention_handler_template_renders_for_issue_comment() {
    let payload = serde_json::json!({
        "action": "created",
        "repository": {"full_name": "acme/widget"},
        "issue": {"number": 42},
        "comment": {
            "id": 123456,
            "body": "@maintainer can you look at this?",
            "user": {"login": "alice"}
        }
    });
    let rendered = render_route("issue_comment", Some("created"), payload);
    assert!(rendered.contains("acme/widget"));
    assert!(rendered.contains("alice"));
    assert!(rendered.contains("can you look at this"));
    assert!(
        rendered.contains("issue#42") || rendered.contains("issue 42") || rendered.contains("#42")
    );
    assert_no_unrendered_placeholders(&rendered, "mention_handler (issue)");
    assert_rendered_parses(&rendered, "mention_handler (issue)");
}

#[test]
fn mention_handler_template_renders_for_pr_review_comment() {
    let payload = serde_json::json!({
        "action": "created",
        "repository": {"full_name": "acme/widget"},
        "pull_request": {"number": 88},
        "comment": {
            "id": 999,
            "body": "@maintainer please review the auth flow",
            "user": {"login": "bob"}
        }
    });
    let rendered = render_route("pull_request_review_comment", Some("created"), payload);
    assert!(rendered.contains("bob"));
    assert!(rendered.contains("pr"));
    assert!(rendered.contains("88"));
    assert_no_unrendered_placeholders(&rendered, "mention_handler (pr)");
    assert_rendered_parses(&rendered, "mention_handler (pr)");
}

#[test]
fn health_check_template_renders_and_parses_on_workflow_dispatch() {
    let payload = serde_json::json!({
        "repository": {"full_name": "acme/widget"}
    });
    let rendered = render_route("workflow_dispatch", None, payload);
    assert!(rendered.contains("acme/widget"));
    assert_no_unrendered_placeholders(&rendered, "health_check");
    assert_rendered_parses(&rendered, "health_check");
}

#[test]
fn health_check_template_renders_and_parses_on_schedule() {
    let payload = serde_json::json!({
        "repository": {"full_name": "acme/widget"}
    });
    let rendered = render_route("schedule", None, payload);
    assert_no_unrendered_placeholders(&rendered, "health_check (schedule)");
    assert_rendered_parses(&rendered, "health_check (schedule)");
}

#[test]
fn every_template_substitution_has_a_template_placeholder() {
    let config = load_dispatch_config();
    for route in &config.routes {
        let template_path = project_root()
            .join("examples/project-maintainer")
            .join(&config.templates_dir)
            .join(&route.template);
        let template_text = fs::read_to_string(&template_path)
            .unwrap_or_else(|err| panic!("could not read {}: {err}", template_path.display()));
        for placeholder in route.fields.keys() {
            let token = format!("{{{{{placeholder}}}}}");
            let token_padded = format!("{{{{ {placeholder} }}}}");
            assert!(
                template_text.contains(&token) || template_text.contains(&token_padded),
                "route '{}' declares field '{}' but template '{}' does not reference it",
                route.event,
                placeholder,
                route.template
            );
        }
    }
}

#[test]
fn every_template_placeholder_has_a_route_substitution() {
    let config = load_dispatch_config();
    let mut templates_seen: HashMap<String, &str> = HashMap::new();
    for route in &config.routes {
        let placeholders: Vec<String> = route.fields.keys().cloned().collect();
        templates_seen
            .entry(route.template.clone())
            .or_insert_with(|| route.event.as_str());

        let template_path = project_root()
            .join("examples/project-maintainer")
            .join(&config.templates_dir)
            .join(&route.template);
        let template_text = fs::read_to_string(&template_path).unwrap();
        let referenced = extract_placeholders(&template_text);
        for name in &referenced {
            assert!(
                placeholders.contains(name),
                "template '{}' uses {{{{ {} }}}} but route for event '{}' doesn't supply it",
                route.template,
                name,
                route.event
            );
        }
    }
    assert!(!templates_seen.is_empty());
}

fn extract_placeholders(template: &str) -> Vec<String> {
    let mut names = Vec::new();
    let mut rest = template;
    while let Some(start) = rest.find("{{") {
        let after = &rest[start + 2..];
        let Some(end) = after.find("}}") else {
            break;
        };
        let raw = after[..end].trim().to_string();
        if !raw.is_empty()
            && raw.chars().all(|character| {
                character.is_ascii_uppercase() || character.is_ascii_digit() || character == '_'
            })
        {
            names.push(raw);
        }
        rest = &after[end + 2..];
    }
    names.sort();
    names.dedup();
    names
}
