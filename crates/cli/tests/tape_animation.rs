use cli::hook::{ComponentMeta, FragmentEvent, FragmentMeta, HookEvent, HookKind, ResourceEvent};
use cli::tape_animation::snapshot_events;

fn resource_event(source: ComponentMeta, event: ResourceEvent) -> HookEvent {
    HookEvent {
        source: Some(source),
        kind: HookKind::Resource(event),
    }
}

fn source(name: &str, index: usize) -> ComponentMeta {
    ComponentMeta {
        graph: "test".into(),
        name: name.into(),
        index,
        kind: "accelerator".into(),
        frontier: Some(1),
    }
}

fn fragment(id: u64, role: &str, kind: &str, tag: &str) -> FragmentMeta {
    FragmentMeta {
        id,
        step: id,
        role: role.into(),
        kind: kind.into(),
        tag: tag.into(),
        preview: format!("fragment {id}"),
    }
}

fn fragment_event(source: ComponentMeta, event: FragmentEvent) -> HookEvent {
    HookEvent {
        source: Some(source),
        kind: HookKind::Fragment(event),
    }
}

#[test]
fn snapshot_uses_fragment_tags_for_cell_tones() {
    let source = source("agent", 0);
    let snapshot = snapshot_events(
        [
            fragment_event(
                source.clone(),
                FragmentEvent::Appended(fragment(1, "system", "text", "env")),
            ),
            fragment_event(
                source,
                FragmentEvent::Appended(fragment(2, "system", "text", "custom_context")),
            ),
        ],
        9,
    );

    let cells = &snapshot.tapes[0].cells;
    assert_eq!(cells[0].tone, "env");
    assert_eq!(cells[1].tone, "tag:custom_context");
}

#[test]
fn snapshot_applies_insert_remove_and_swap_by_fragment_id() {
    let source = source("worker", 0);
    let snapshot = snapshot_events(
        [
            fragment_event(
                source.clone(),
                FragmentEvent::Appended(fragment(1, "user", "text", "user")),
            ),
            fragment_event(
                source.clone(),
                FragmentEvent::Appended(fragment(2, "assistant", "text", "assistant")),
            ),
            fragment_event(
                source.clone(),
                FragmentEvent::Inserted {
                    meta: fragment(3, "system", "text", "instruction"),
                    after: 1,
                },
            ),
            fragment_event(
                source.clone(),
                FragmentEvent::Swapped {
                    first: 1,
                    second: 2,
                },
            ),
            fragment_event(source, FragmentEvent::Removed { id: 3 }),
        ],
        9,
    );

    let ids = snapshot.tapes[0]
        .cells
        .iter()
        .map(|cell| cell.id)
        .collect::<Vec<_>>();
    assert_eq!(ids, vec![2, 1]);
}

#[test]
fn snapshot_hides_overflow_tapes_with_fixed_view_capacity() {
    let events = (0..5).map(|index| {
        fragment_event(
            source(&format!("worker_{index}"), index),
            FragmentEvent::Appended(fragment(index as u64 + 1, "user", "text", "user")),
        )
    });

    let snapshot = snapshot_events(events, 9);

    assert_eq!(snapshot.tapes.len(), 2);
    assert_eq!(snapshot.hidden_tapes, 3);
}

#[test]
fn scaffolding_replace_flashes_without_moving_pointer() {
    let source = source("worker", 0);
    // Append the env scaffolding first, then a long run of content so the pointer
    // sits far from the env cell, then re-emit env (the per-round timestamp tick).
    let mut events = vec![fragment_event(
        source.clone(),
        FragmentEvent::Appended(fragment(1, "system", "text", "env")),
    )];
    for id in 2..=12 {
        events.push(fragment_event(
            source.clone(),
            FragmentEvent::Appended(fragment(id, "assistant", "text", "")),
        ));
    }
    let pointer_before = snapshot_events(events.clone(), 9).tapes[0].pointer;

    // env re-emitted with the same id but new text → a Replace of scaffolding.
    events.push(fragment_event(
        source,
        FragmentEvent::Replaced(fragment(1, "system", "text", "env")),
    ));
    let after = snapshot_events(events, 9);

    // The pointer must not have run back to the env cell at index 0.
    assert_eq!(after.tapes[0].pointer, pointer_before);
    assert!(pointer_before > 0, "pointer should be at the content end");
    // The env cell keeps its env tone (flash settles back to Written/env).
    assert_eq!(after.tapes[0].cells[0].tone, "env");
}

#[test]
fn distinct_tags_get_distinct_tones() {
    let source = source("worker", 0);
    let snapshot = snapshot_events(
        [
            fragment_event(
                source.clone(),
                FragmentEvent::Appended(fragment(1, "system", "text", "handoff")),
            ),
            fragment_event(
                source,
                FragmentEvent::Appended(fragment(2, "system", "text", "scratch")),
            ),
        ],
        9,
    );

    let cells = &snapshot.tapes[0].cells;
    assert_eq!(cells[0].tone, "tag:handoff");
    assert_eq!(cells[1].tone, "tag:scratch");
}

#[test]
fn take_settles_with_intake_status() {
    let source = source("worker", 0);
    let snapshot = snapshot_events(
        [fragment_event(
            source,
            FragmentEvent::Taken(fragment(1, "user", "text", "user")),
        )],
        9,
    );

    // Taking from the inbox runs through the intake animation; the cell still
    // lands as a normal written cell, distinct from a plain append by its verb.
    assert_eq!(snapshot.tapes[0].last_action, "intake");
    assert_eq!(snapshot.tapes[0].cells.len(), 1);
    assert_eq!(snapshot.tapes[0].cells[0].id, 1);
}

#[test]
fn resource_actions_surface_a_status_verb() {
    let source = source("worker", 0);
    let snapshot = snapshot_events(
        [
            fragment_event(
                source.clone(),
                FragmentEvent::Appended(fragment(1, "user", "text", "user")),
            ),
            resource_event(source, ResourceEvent::Activate { name: "fs".into() }),
        ],
        9,
    );

    assert_eq!(snapshot.tapes[0].last_action, "activate fs");
}
