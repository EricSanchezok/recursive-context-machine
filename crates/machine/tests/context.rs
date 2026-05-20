use machine::{Context, ContextIdNotFound, Fragment};

#[test]
fn empty_context() {
    let ctx = Context::new();
    assert!(ctx.is_empty());
    assert_eq!(ctx.next_id(), 1);
}

#[test]
fn append_assigns_ids() {
    let mut ctx = Context::new();
    let id1 = ctx.append(Fragment::system("a"));
    let id2 = ctx.append(Fragment::user("b"));
    assert_eq!(id1, 1);
    assert_eq!(id2, 2);
    assert_eq!(ctx.get(1).unwrap().id(), 1);
}

#[test]
fn ids_never_reused_after_remove() {
    let mut ctx = Context::new();
    let id1 = ctx.append(Fragment::system("a"));
    ctx.remove(id1).unwrap();
    let id2 = ctx.append(Fragment::system("b"));
    assert_ne!(id1, id2);
    assert_eq!(id2, 2);
}

#[test]
fn insert_after_id() {
    let mut ctx = Context::new();
    let sys = ctx.append(Fragment::system("a"));
    let user = ctx.insert(sys, Fragment::user("b")).unwrap();
    assert_eq!(ctx.len(), 2);
    assert_eq!(ctx.get(sys).unwrap().as_text(), Some("a"));
    assert_eq!(ctx.get(user).unwrap().as_text(), Some("b"));
}

#[test]
fn insert_unknown_id_returns_error() {
    let mut ctx = Context::new();
    let result = ctx.insert(999, Fragment::user("x"));
    assert_eq!(result, Err(ContextIdNotFound(999)));
    // Context unchanged after failed insert.
    assert!(ctx.is_empty());
}

#[test]
fn replace_preserves_id() {
    let mut ctx = Context::new();
    let id = ctx.append(Fragment::system("old"));
    ctx.replace(id, Fragment::system("new")).unwrap();
    assert_eq!(ctx.get(id).unwrap().as_text(), Some("new"));
    assert_eq!(ctx.get(id).unwrap().id(), id);
}

#[test]
fn replace_unknown_returns_error() {
    let mut ctx = Context::new();
    let result = ctx.replace(999, Fragment::user("x"));
    assert_eq!(result, Err(ContextIdNotFound(999)));
}

#[test]
fn remove_single_and_verify() {
    let mut ctx = Context::new();
    let id = ctx.append(Fragment::system("a"));
    ctx.remove(id).unwrap();
    assert!(ctx.get(id).is_none());
    assert_eq!(ctx.len(), 0);
}

#[test]
fn remove_unknown_returns_error() {
    let mut ctx = Context::new();
    let result = ctx.remove(999);
    assert_eq!(result, Err(ContextIdNotFound(999)));
}

#[test]
fn swap_unknown_first_id_returns_error_and_leaves_context_unchanged() {
    let mut ctx = Context::new();
    let id1 = ctx.append(Fragment::system("a"));
    let id2 = ctx.append(Fragment::user("b"));
    let snapshot: Vec<u64> = ctx.fragments().iter().map(|f| f.id()).collect();

    let result = ctx.swap(999, id2);
    assert_eq!(result, Err(ContextIdNotFound(999)));
    let after: Vec<u64> = ctx.fragments().iter().map(|f| f.id()).collect();
    assert_eq!(
        after, snapshot,
        "context must be unchanged after failed swap"
    );

    // Swapping in the reverse missing position also fails.
    let result = ctx.swap(id1, 888);
    assert_eq!(result, Err(ContextIdNotFound(888)));
}

#[test]
fn swap_succeeds_returning_unit() {
    let mut ctx = Context::new();
    let id1 = ctx.append(Fragment::system("a"));
    let id2 = ctx.append(Fragment::user("b"));
    ctx.swap(id1, id2).unwrap();
    assert_eq!(ctx.fragments()[0].id(), id2);
    assert_eq!(ctx.fragments()[1].id(), id1);
}

#[test]
fn error_display_includes_id() {
    let error = ContextIdNotFound(42);
    let text = error.to_string();
    assert!(text.contains("42"));
    assert!(text.contains("not found"));
}

#[test]
fn position_by_id() {
    let mut ctx = Context::new();
    ctx.append(Fragment::system("a"));
    let id = ctx.append(Fragment::user("b"));
    ctx.append(Fragment::assistant("c"));
    assert_eq!(ctx.position_of(id), Some(1));
    assert_eq!(ctx.position_of(999), None);
}

#[test]
fn get_invalid_is_none() {
    let ctx = Context::new();
    assert!(ctx.get(999).is_none());
}

#[test]
fn fragments_in_order() {
    let mut ctx = Context::new();
    let id1 = ctx.append(Fragment::system("a"));
    let id2 = ctx.append(Fragment::user("b"));
    let id3 = ctx.append(Fragment::assistant("c"));
    let frags = ctx.fragments();
    assert_eq!(frags.len(), 3);
    assert_eq!(frags[0].id(), id1);
    assert_eq!(frags[1].id(), id2);
    assert_eq!(frags[2].id(), id3);
}
