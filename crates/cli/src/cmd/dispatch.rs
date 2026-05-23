use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, anyhow};
use serde::Deserialize;
use serde_json::Value;

use crate::args::DispatchArgs;

#[derive(Debug, Deserialize)]
pub struct DispatchConfig {
    pub templates_dir: PathBuf,
    pub cache_dir: PathBuf,
    #[serde(rename = "route")]
    pub routes: Vec<Route>,
}

#[derive(Debug, Deserialize)]
pub struct Route {
    pub event: String,
    pub action: Option<String>,
    pub template: String,
    pub filter: Option<Filter>,
    #[serde(default)]
    pub fields: HashMap<String, FieldSource>,
}

#[derive(Debug, Deserialize)]
pub struct Filter {
    pub field: String,
    pub contains: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum FieldSource {
    Path(String),
    Literal {
        #[serde(rename = "literal")]
        value: String,
    },
}

pub fn run(args: DispatchArgs) -> Result<()> {
    let config_path = args.config.clone();
    let config_text = fs::read_to_string(&config_path).with_context(|| {
        format!("reading dispatch config from {}", config_path.display())
    })?;
    let config: DispatchConfig =
        toml::from_str(&config_text).context("parsing dispatch config")?;

    let event_text = fs::read_to_string(&args.event_path).with_context(|| {
        format!("reading event payload from {}", args.event_path.display())
    })?;
    let event_json: Value = serde_json::from_str(&event_text).context("parsing event JSON")?;

    let action_filter = if args.action.is_empty() {
        None
    } else {
        Some(args.action.as_str())
    };

    let Some(route) = select_route(&config.routes, &args.event_name, action_filter) else {
        if args.verbose {
            eprintln!(
                "no route matches event '{}' action '{}'; skipping",
                args.event_name, args.action
            );
        }
        return Ok(());
    };

    if !passes_filter(route, &event_json) {
        if args.verbose {
            eprintln!("filter excludes event; skipping");
        }
        return Ok(());
    }

    let substitutions = collect_substitutions(route, &event_json)?;

    let config_dir = config_path.parent().unwrap_or(Path::new("."));
    let template_path = config_dir
        .join(&config.templates_dir)
        .join(&route.template);
    let template_text = fs::read_to_string(&template_path).with_context(|| {
        format!("reading template from {}", template_path.display())
    })?;

    let rendered = render_template(&template_text, &substitutions)?;

    let cache_dir = config_dir.join(&config.cache_dir);
    fs::create_dir_all(&cache_dir).with_context(|| {
        format!("creating cache dir {}", cache_dir.display())
    })?;

    let stem = route.template.trim_end_matches(".tpl");
    let stem = Path::new(stem).file_stem().and_then(|s| s.to_str()).unwrap_or("rendered");
    let timestamp = chrono::Utc::now().format("%Y%m%dT%H%M%S%6f");
    let output_name = format!("{timestamp}-{stem}.rcm");
    let output_path = cache_dir.join(output_name);
    fs::write(&output_path, &rendered).with_context(|| {
        format!("writing rendered .rcm to {}", output_path.display())
    })?;

    println!("{}", output_path.display());
    Ok(())
}

pub fn select_route<'a>(
    routes: &'a [Route],
    event: &str,
    action: Option<&str>,
) -> Option<&'a Route> {
    let exact = routes
        .iter()
        .find(|route| route.event == event && route.action.as_deref() == action);
    if exact.is_some() {
        return exact;
    }
    routes
        .iter()
        .find(|route| route.event == event && route.action.is_none())
}

pub fn passes_filter(route: &Route, event: &Value) -> bool {
    let Some(filter) = &route.filter else {
        return true;
    };
    let Some(value) = resolve_json_path(event, &filter.field) else {
        return false;
    };
    value.contains(&filter.contains)
}

pub fn collect_substitutions(
    route: &Route,
    event: &Value,
) -> Result<HashMap<String, String>> {
    let mut substitutions = HashMap::with_capacity(route.fields.len());
    for (placeholder, source) in &route.fields {
        let value = match source {
            FieldSource::Path(path) => resolve_json_path(event, path).ok_or_else(|| {
                anyhow!("event payload missing field '{path}' (required by '{placeholder}')")
            })?,
            FieldSource::Literal { value } => value.clone(),
        };
        substitutions.insert(placeholder.clone(), value);
    }
    Ok(substitutions)
}

pub fn resolve_json_path(event: &Value, path: &str) -> Option<String> {
    let mut current = event;
    for segment in path.split('.') {
        current = current.get(segment)?;
    }
    json_value_to_string(current)
}

fn json_value_to_string(value: &Value) -> Option<String> {
    match value {
        Value::Null => None,
        Value::String(text) => Some(text.clone()),
        Value::Number(number) => Some(number.to_string()),
        Value::Bool(flag) => Some(flag.to_string()),
        _ => None,
    }
}

pub fn render_template(
    template: &str,
    substitutions: &HashMap<String, String>,
) -> Result<String> {
    let mut output = String::with_capacity(template.len());
    let mut rest = template;
    let mut missing = Vec::new();

    while let Some(start) = rest.find("{{") {
        output.push_str(&rest[..start]);
        let after_open = &rest[start + 2..];
        let Some(close_rel) = after_open.find("}}") else {
            return Err(anyhow!("unbalanced '{{{{' in template"));
        };
        let raw_name = after_open[..close_rel].trim();
        if raw_name.is_empty() || !is_valid_placeholder(raw_name) {
            return Err(anyhow!(
                "invalid placeholder syntax: '{{{{ {raw_name} }}}}'"
            ));
        }
        match substitutions.get(raw_name) {
            Some(value) => output.push_str(value),
            None => missing.push(raw_name.to_string()),
        }
        rest = &after_open[close_rel + 2..];
    }
    output.push_str(rest);

    if !missing.is_empty() {
        missing.sort();
        missing.dedup();
        return Err(anyhow!(
            "template references unresolved placeholders: {missing:?}"
        ));
    }
    Ok(output)
}

fn is_valid_placeholder(name: &str) -> bool {
    let mut chars = name.chars();
    let first = match chars.next() {
        Some(character) => character,
        None => return false,
    };
    if !(first.is_ascii_uppercase() || first == '_') {
        return false;
    }
    chars.all(|character| character.is_ascii_uppercase() || character.is_ascii_digit() || character == '_')
}
