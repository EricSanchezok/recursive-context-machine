use std::collections::HashMap;

use cli::cmd::dispatch::{
    FieldSource, Filter, Route, collect_substitutions, passes_filter, render_template,
    resolve_json_path, select_route,
};
use serde_json::json;

fn route_simple(event: &str, action: Option<&str>, template: &str) -> Route {
    Route {
        event: event.to_string(),
        action: action.map(str::to_string),
        template: template.to_string(),
        filter: None,
        fields: HashMap::new(),
    }
}

#[test]
fn select_route_prefers_exact_action_match() {
    let routes = vec![
        route_simple("issues", None, "fallback.tpl"),
        route_simple("issues", Some("opened"), "opened.tpl"),
    ];
    let chosen = select_route(&routes, "issues", Some("opened")).unwrap();
    assert_eq!(chosen.template, "opened.tpl");
}

#[test]
fn select_route_falls_back_to_action_less_route() {
    let routes = vec![route_simple("workflow_dispatch", None, "health.tpl")];
    let chosen = select_route(&routes, "workflow_dispatch", None).unwrap();
    assert_eq!(chosen.template, "health.tpl");
}

#[test]
fn select_route_returns_none_when_no_event_matches() {
    let routes = vec![route_simple("issues", Some("opened"), "opened.tpl")];
    assert!(select_route(&routes, "pull_request", Some("opened")).is_none());
}

#[test]
fn select_route_returns_none_when_action_differs_and_no_fallback() {
    let routes = vec![route_simple("issues", Some("opened"), "opened.tpl")];
    assert!(select_route(&routes, "issues", Some("closed")).is_none());
}

#[test]
fn passes_filter_when_no_filter_present() {
    let route = route_simple("issues", Some("opened"), "any.tpl");
    let payload = json!({});
    assert!(passes_filter(&route, &payload));
}

#[test]
fn passes_filter_when_substring_present() {
    let mut route = route_simple("issue_comment", Some("created"), "mention.tpl");
    route.filter = Some(Filter {
        field: "comment.body".to_string(),
        contains: "@maintainer".to_string(),
    });
    let payload = json!({"comment": {"body": "hey @maintainer help"}});
    assert!(passes_filter(&route, &payload));
}

#[test]
fn passes_filter_rejects_when_substring_absent() {
    let mut route = route_simple("issue_comment", Some("created"), "mention.tpl");
    route.filter = Some(Filter {
        field: "comment.body".to_string(),
        contains: "@maintainer".to_string(),
    });
    let payload = json!({"comment": {"body": "thanks for the fix"}});
    assert!(!passes_filter(&route, &payload));
}

#[test]
fn passes_filter_rejects_when_field_missing() {
    let mut route = route_simple("issue_comment", Some("created"), "mention.tpl");
    route.filter = Some(Filter {
        field: "comment.body".to_string(),
        contains: "@maintainer".to_string(),
    });
    let payload = json!({"comment": {}});
    assert!(!passes_filter(&route, &payload));
}

#[test]
fn resolve_json_path_walks_nested_objects() {
    let payload = json!({"repository": {"full_name": "owner/repo"}});
    assert_eq!(
        resolve_json_path(&payload, "repository.full_name").as_deref(),
        Some("owner/repo")
    );
}

#[test]
fn resolve_json_path_handles_numbers_and_bools() {
    let payload = json!({"issue": {"number": 42, "locked": false}});
    assert_eq!(
        resolve_json_path(&payload, "issue.number").as_deref(),
        Some("42")
    );
    assert_eq!(
        resolve_json_path(&payload, "issue.locked").as_deref(),
        Some("false")
    );
}

#[test]
fn resolve_json_path_returns_none_for_missing_field() {
    let payload = json!({"issue": {}});
    assert!(resolve_json_path(&payload, "issue.number").is_none());
    assert!(resolve_json_path(&payload, "unknown.path").is_none());
}

#[test]
fn collect_substitutions_pulls_paths_and_literals() {
    let mut route = route_simple("issue_comment", Some("created"), "mention.tpl");
    route.fields.insert(
        "REPO".into(),
        FieldSource::Path("repository.full_name".into()),
    );
    route
        .fields
        .insert("TRIGGER_NUMBER".into(), FieldSource::Path("issue.number".into()));
    route.fields.insert(
        "TRIGGER_KIND".into(),
        FieldSource::Literal {
            value: "issue".into(),
        },
    );

    let payload = json!({
        "repository": {"full_name": "acme/widget"},
        "issue": {"number": 7}
    });

    let subs = collect_substitutions(&route, &payload).unwrap();
    assert_eq!(subs.get("REPO").map(String::as_str), Some("acme/widget"));
    assert_eq!(subs.get("TRIGGER_NUMBER").map(String::as_str), Some("7"));
    assert_eq!(subs.get("TRIGGER_KIND").map(String::as_str), Some("issue"));
}

#[test]
fn collect_substitutions_errors_when_required_field_missing() {
    let mut route = route_simple("issues", Some("opened"), "triage.tpl");
    route
        .fields
        .insert("ISSUE_NUMBER".into(), FieldSource::Path("issue.number".into()));

    let payload = json!({"issue": {}});
    let result = collect_substitutions(&route, &payload);
    let error = result.unwrap_err().to_string();
    assert!(
        error.contains("issue.number"),
        "expected error to mention the missing path, got: {error}"
    );
}

#[test]
fn render_template_substitutes_known_placeholders() {
    let template = "issue {{ISSUE_NUMBER}} in {{REPO}}";
    let mut subs = HashMap::new();
    subs.insert("ISSUE_NUMBER".into(), "42".into());
    subs.insert("REPO".into(), "acme/widget".into());
    assert_eq!(
        render_template(template, &subs).unwrap(),
        "issue 42 in acme/widget"
    );
}

#[test]
fn render_template_handles_repeated_placeholders() {
    let template = "{{REPO}} again {{REPO}}";
    let mut subs = HashMap::new();
    subs.insert("REPO".into(), "x/y".into());
    assert_eq!(render_template(template, &subs).unwrap(), "x/y again x/y");
}

#[test]
fn render_template_passes_through_text_with_no_placeholders() {
    let template = "no placeholders here";
    let subs = HashMap::new();
    assert_eq!(render_template(template, &subs).unwrap(), template);
}

#[test]
fn render_template_errors_on_unresolved_placeholders() {
    let template = "hi {{MISSING}}";
    let subs = HashMap::new();
    let error = render_template(template, &subs).unwrap_err().to_string();
    assert!(error.contains("MISSING"), "got: {error}");
}

#[test]
fn render_template_errors_on_unbalanced_braces() {
    let template = "broken {{NAME";
    let mut subs = HashMap::new();
    subs.insert("NAME".into(), "x".into());
    let error = render_template(template, &subs).unwrap_err().to_string();
    assert!(error.contains("unbalanced"), "got: {error}");
}

#[test]
fn render_template_errors_on_invalid_placeholder_syntax() {
    let template = "bad {{lowercase}}";
    let subs = HashMap::new();
    let error = render_template(template, &subs).unwrap_err().to_string();
    assert!(error.contains("invalid placeholder"), "got: {error}");
}

#[test]
fn render_template_accepts_whitespace_inside_braces() {
    let template = "hello {{ NAME }}";
    let mut subs = HashMap::new();
    subs.insert("NAME".into(), "world".into());
    assert_eq!(render_template(template, &subs).unwrap(), "hello world");
}
