use cli::hook::{ComponentMeta, FragmentEvent, FragmentMeta, HookEvent, HookKind};
use cli::tape_animation::snapshot_events;

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
