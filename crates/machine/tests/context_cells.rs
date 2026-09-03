use machine::context::{CellMeta, PROTECTED_ANCHORS, SLOT_ORDER};
use machine::{CellDirEntry, Context, Fragment};
#[test]
fn set_named_replaces_in_place() {
    let mut context = Context::new();
    let first = context.set_named("@agent", Fragment::system("v1"));
    let second = context.set_named("@agent", Fragment::system("v2"));

    assert_eq!(first, second, "anchor keeps a stable cell id");
    assert_eq!(context.len(), 1);
    assert_eq!(context.get(first).unwrap().as_text(), Some("v2"));
    assert_eq!(
        context.get(first).unwrap().anchor.as_deref(),
        Some("@agent")
    );
}

#[test]
fn new_slots_insert_in_slot_order() {
    let mut context = Context::new();
    // Insert deliberately out of order.
    context.set_named("@summary", Fragment::system("s"));
    context.set_named("@agent", Fragment::system("a"));

    let anchors: Vec<Option<String>> = context
        .fragments()
        .iter()
        .map(|cell| cell.anchor.clone())
        .collect();
    // @agent ranks before @summary in SLOT_ORDER.
    let agent_pos = anchors.iter().position(|a| a.as_deref() == Some("@agent"));
    let summary_pos = anchors
        .iter()
        .position(|a| a.as_deref() == Some("@summary"));
    assert!(agent_pos.unwrap() < summary_pos.unwrap());
}

#[test]
fn custom_anchor_lands_after_known_slots_before_unanchored() {
    let mut context = Context::new();
    context.append(Fragment::user("task"));
    context.set_named("@agent", Fragment::system("a"));

    // A custom anchor must slot between the anchored region and the
    // unanchored cells, not disturb either region's internal order.
    context.set_named("@custom", Fragment::system("c"));

    let sequence: Vec<String> = context
        .fragments()
        .iter()
        .map(|cell| {
            cell.anchor
                .clone()
                .unwrap_or_else(|| format!("id:{}", cell.id()))
        })
        .collect();
    let custom_pos = sequence.iter().position(|s| s == "@custom").unwrap();
    let agent_pos = sequence.iter().position(|s| s == "@agent").unwrap();
    assert!(agent_pos < custom_pos);
    // Unanchored cells stay last.
    assert!(sequence.last().unwrap().starts_with("id:"));
}

#[test]
fn find_anchor_resolves_and_none_for_missing() {
    let mut context = Context::new();
    context.set_named("@purpose", Fragment::user("goal"));
    assert!(context.find_anchor("@purpose").is_some());
    assert!(context.find_anchor("@agent").is_none());
}

#[test]
fn protected_anchors_are_the_scaffolding() {
    assert_eq!(PROTECTED_ANCHORS.len(), 3);
    for anchor in PROTECTED_ANCHORS {
        assert!(SLOT_ORDER.contains(&anchor));
    }
}

#[test]
fn metadata_lifecycle_created_seen_removed() {
    let mut context = Context::new();
    let id = context.append(Fragment::user("hello"));

    context.note_created(id, 3, None);
    context.note_seen(&[id], 7, Some(11));

    let meta = context.meta(id);
    assert_eq!(
        meta,
        CellMeta {
            created_step: 3,
            last_seen_step: 7,
            source_completion: Some(11),
        }
    );

    // Removal drops the metadata with the cell.
    context.remove(id).unwrap();
    assert_eq!(context.meta(id), CellMeta::default());
}

#[test]
fn note_seen_is_monotonic_and_never_lowers_completion() {
    let mut context = Context::new();
    let id = context.append(Fragment::user("x"));
    context.note_seen(&[id], 10, Some(2));
    context.note_seen(&[id], 4, None);
    context.note_seen(&[id], 12, None);

    let meta = context.meta(id);
    assert_eq!(meta.last_seen_step, 12);
    assert_eq!(meta.source_completion, Some(2), "completion not erased");
}

#[test]
fn replace_keeps_metadata_identity() {
    let mut context = Context::new();
    let id = context.set_named("@summary", Fragment::system("first"));
    context.note_created(id, 2, None);

    context.set_named("@summary", Fragment::system("second"));
    let meta = context.meta(id);
    assert_eq!(meta.created_step, 2, "slot identity persists across set");
}

#[test]
fn move_after_reorders_without_losing_cells() {
    let mut context = Context::new();
    let first = context.append(Fragment::user("1"));
    let second = context.append(Fragment::user("2"));
    let third = context.append(Fragment::user("3"));

    context.move_after(first, third).unwrap();

    let texts: Vec<&str> = context
        .fragments()
        .iter()
        .filter_map(|cell| cell.as_text())
        .collect();
    assert_eq!(texts, vec!["2", "3", "1"]);
    assert!(context.get(second).is_some());
}

#[test]
fn cell_bytes_covers_content_kinds() {
    assert_eq!(Context::cell_bytes(&Fragment::user("12345")), 5);
    assert_eq!(
        Context::cell_bytes(&Fragment::tool_result("c1", "hello", None)),
        5
    );
    assert!(
        Context::cell_bytes(&Fragment::image(
            machine::DataSource::Base64("x".into()),
            None
        )) > 0
    );
}

#[test]
fn directory_entries_carry_metadata_and_capped_preview() {
    let run_dir_text = "x".repeat(300);
    let mut context = Context::new();
    let id = context.set_named("@summary", Fragment::system(run_dir_text));
    context.note_created(id, 5, None);
    context.note_seen(&[id], 9, Some(4));

    let run = machine::RunState {
        context,
        ..machine::RunState::default()
    };
    let obs = machine::obs::measure(&run);

    let directory: Vec<&CellDirEntry> = obs
        .context_directory
        .iter()
        .filter(|entry| entry.id == id)
        .collect();
    assert_eq!(directory.len(), 1);
    let entry = directory[0];
    assert_eq!(entry.anchor.as_deref(), Some("@summary"));
    assert_eq!(entry.role, "system");
    assert_eq!(entry.kind, "text");
    assert_eq!(entry.tag, "system");
    assert_eq!(entry.bytes, 300);
    assert_eq!(entry.created_step, 5);
    assert_eq!(entry.last_seen_step, 9);
    assert!(entry.preview.chars().count() <= 80);
    assert!(entry.preview.starts_with('x'));
}
