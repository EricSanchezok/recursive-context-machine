use machine::completion::{assemble_messages, encode_context};
use machine::{Fragment, Overlay, Role};
use rig::completion::Message;

fn sample_tape() -> Vec<Fragment> {
    vec![
        Fragment::system("agent prompt"),
        Fragment::user("task"),
        Fragment::assistant("working on it"),
    ]
}

#[test]
fn empty_overlay_is_byte_identical_to_encode_context() {
    let fragments = sample_tape();
    let plain = encode_context(&fragments, false);
    let with_empty_overlay = assemble_messages(&fragments, false, &Overlay::default());
    assert_eq!(
        plain.len(),
        with_empty_overlay.len(),
        "empty overlay must not add messages"
    );
    // rig Message implements Debug; compare through Debug for byte equality.
    assert_eq!(
        format!("{:?}", plain),
        format!("{:?}", with_empty_overlay),
        "empty overlay must produce byte-identical encoding"
    );
}

#[test]
fn overlay_prefix_lands_before_tape_and_tail_after() {
    let fragments = sample_tape();
    let overlay = Overlay {
        system_prefix: vec!["cached identity block".into()],
        tail: vec!["volatile reminder".into()],
    };

    let messages = assemble_messages(&fragments, false, &overlay);

    assert_eq!(messages.len(), fragments.len() + 2);
    let Message::System { content } = &messages[0] else {
        panic!("first message must be the overlay prefix");
    };
    assert_eq!(content, "cached identity block");

    // Middle of the list is the encoded tape, untouched and in order.
    let Message::User { .. } = &messages[2] else {
        panic!("tape messages must follow the prefix unchanged");
    };
    let Message::Assistant { .. } = &messages[3] else {
        panic!("assistant tape message must stay in position");
    };

    let Message::System { content } = &messages[4] else {
        panic!("last message must be the overlay tail");
    };
    assert_eq!(content, "volatile reminder");
}

#[test]
fn overlay_multiple_prefixes_preserve_declaration_order() {
    let fragments = vec![Fragment::user("task")];
    let overlay = Overlay {
        system_prefix: vec!["first block".into(), "second block".into()],
        tail: vec![],
    };

    let messages = assemble_messages(&fragments, false, &overlay);
    assert_eq!(messages.len(), 3);
    let Message::System { content } = &messages[0] else {
        panic!("expected first prefix block");
    };
    assert_eq!(content, "first block");
    let Message::System { content } = &messages[1] else {
        panic!("expected second prefix block");
    };
    assert_eq!(content, "second block");
}

#[test]
fn overlay_default_is_empty() {
    let overlay = Overlay::default();
    assert!(overlay.is_empty());
    assert!(overlay.system_prefix.is_empty());
    assert!(overlay.tail.is_empty());
}

#[test]
fn overlay_survives_serde_round_trip() {
    let overlay = Overlay {
        system_prefix: vec!["a".into()],
        tail: vec!["b".into(), "c".into()],
    };
    let encoded = serde_json::to_string(&overlay).unwrap();
    let decoded: Overlay = serde_json::from_str(&encoded).unwrap();
    assert_eq!(decoded, overlay);
}

#[test]
fn empty_overlay_with_hitch_tape_matches_plain_encoding() {
    // Hitch filtering is encode_context's job; the overlay wrapper must not
    // disturb it — verify on a tape containing a filtered assistant hitch.
    let fragments = vec![
        Fragment::user("request"),
        Fragment::hitch(
            "HTTP 504 Gateway Timeout",
            Some(504),
            Role::Assistant,
            None::<&str>,
        ),
    ];
    let plain = encode_context(&fragments, false);
    let wrapped = assemble_messages(&fragments, false, &Overlay::default());
    assert_eq!(plain.len(), wrapped.len());
    assert_eq!(format!("{:?}", plain), format!("{:?}", wrapped));
}
