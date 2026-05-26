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
    let err = ctx.insert(999, Fragment::user("x")).unwrap_err();
    assert_eq!(err, ContextIdNotFound(999));
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
    let err = ctx.replace(999, Fragment::user("x")).unwrap_err();
    assert_eq!(err, ContextIdNotFound(999));
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
    let err = ctx.remove(999).unwrap_err();
    assert_eq!(err, ContextIdNotFound(999));
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

#[test]
fn swap_unknown_returns_error() {
    let mut ctx = Context::new();
    let id = ctx.append(Fragment::system("a"));
    let err = ctx.swap(id, 999).unwrap_err();
    assert_eq!(err, ContextIdNotFound(999));
}

#[test]
fn swap_works() {
    let mut ctx = Context::new();
    let a = ctx.append(Fragment::system("a"));
    let b = ctx.append(Fragment::user("b"));
    ctx.swap(a, b).unwrap();
    assert_eq!(ctx.position_of(a), Some(1));
    assert_eq!(ctx.position_of(b), Some(0));
    assert_eq!(ctx.fragments()[0].id(), b);
    assert_eq!(ctx.fragments()[1].id(), a);
}

#[test]
fn display_not_found() {
    let err = ContextIdNotFound(42);
    assert_eq!(err.to_string(), "fragment id 42 not found in context");
}
