use machine::{Context, Fragment};

#[test]
fn new_context_is_empty() {
    let ctx = Context::new();
    assert!(ctx.is_empty());
    assert_eq!(ctx.len(), 0);
}

#[test]
fn first_id_is_one() {
    let ctx = Context::new();
    assert_eq!(ctx.next_id(), 1);
}

#[test]
fn append_assigns_monotonic_ids() {
    let mut ctx = Context::new();
    let id1 = ctx.append(Fragment::system("a"));
    let id2 = ctx.append(Fragment::user("b"));
    let id3 = ctx.append(Fragment::assistant("c"));
    assert_eq!(id1, 1);
    assert_eq!(id2, 2);
    assert_eq!(id3, 3);
}

#[test]
fn append_updates_fragment_id() {
    let mut ctx = Context::new();
    let f = Fragment::system("test");
    assert_eq!(f.id, 0);
    let id = ctx.append(f);
    assert_eq!(id, 1);
    assert_eq!(ctx.get(1).unwrap().id, 1);
}

#[test]
fn append_grows_context() {
    let mut ctx = Context::new();
    ctx.append(Fragment::system("a"));
    ctx.append(Fragment::user("b"));
    assert_eq!(ctx.len(), 2);
    assert!(!ctx.is_empty());
}

#[test]
fn insert_after_first() {
    let mut ctx = Context::new();
    let id1 = ctx.append(Fragment::system("a"));
    let id2 = ctx.insert(id1, Fragment::user("b"));
    assert_eq!(ctx.len(), 2);
    // Order: system(a), user(b)
    assert_eq!(ctx.get(id1).unwrap().as_text(), Some("a"));
    assert_eq!(ctx.get(id2).unwrap().as_text(), Some("b"));
}

#[test]
fn insert_after_last() {
    let mut ctx = Context::new();
    let id1 = ctx.append(Fragment::system("a"));
    let id2 = ctx.append(Fragment::user("b"));
    let id3 = ctx.insert(id2, Fragment::assistant("c"));
    assert_eq!(ctx.len(), 3);
    // Order: system(a), user(b), assistant(c)
    let frags = ctx.fragments();
    assert_eq!(frags[0].id, id1);
    assert_eq!(frags[1].id, id2);
    assert_eq!(frags[2].id, id3);
}

#[test]
fn insert_assigns_new_id() {
    let mut ctx = Context::new();
    let id1 = ctx.append(Fragment::system("a"));
    let id2 = ctx.insert(id1, Fragment::user("b"));
    assert_ne!(id1, id2);
    assert_eq!(id2, 2);
}

#[test]
#[should_panic(expected = "not found")]
fn insert_panics_on_unknown_id() {
    let mut ctx = Context::new();
    ctx.insert(999, Fragment::user("x"));
}

#[test]
fn replace_preserves_id() {
    let mut ctx = Context::new();
    let id = ctx.append(Fragment::system("old"));
    ctx.replace(id, Fragment::system("new"));
    assert_eq!(ctx.get(id).unwrap().as_text(), Some("new"));
    assert_eq!(ctx.get(id).unwrap().id, id);
}

#[test]
#[should_panic(expected = "not found")]
fn replace_panics_on_unknown_id() {
    let mut ctx = Context::new();
    ctx.replace(999, Fragment::user("x"));
}

#[test]
fn remove_deletes_fragment() {
    let mut ctx = Context::new();
    let id = ctx.append(Fragment::system("a"));
    ctx.remove(id);
    assert!(ctx.get(id).is_none());
    assert_eq!(ctx.len(), 0);
}

#[test]
fn remove_shrinks_context() {
    let mut ctx = Context::new();
    let id1 = ctx.append(Fragment::system("a"));
    let id2 = ctx.append(Fragment::user("b"));
    ctx.remove(id1);
    assert_eq!(ctx.len(), 1);
    assert_eq!(ctx.get(id2).unwrap().as_text(), Some("b"));
}

#[test]
#[should_panic(expected = "not found")]
fn remove_panics_on_unknown_id() {
    let mut ctx = Context::new();
    ctx.remove(999);
}

#[test]
fn find_returns_position() {
    let mut ctx = Context::new();
    ctx.append(Fragment::system("a"));
    let id = ctx.append(Fragment::user("b"));
    ctx.append(Fragment::assistant("c"));
    assert_eq!(ctx.find(id), Some(1));
}

#[test]
fn find_returns_none_for_unknown() {
    let ctx = Context::new();
    assert_eq!(ctx.find(999), None);
}

#[test]
fn find_returns_none_after_remove() {
    let mut ctx = Context::new();
    let id = ctx.append(Fragment::system("a"));
    ctx.remove(id);
    assert_eq!(ctx.find(id), None);
}

#[test]
fn get_returns_reference() {
    let mut ctx = Context::new();
    let id = ctx.append(Fragment::user("hello"));
    assert_eq!(ctx.get(id).unwrap().as_text(), Some("hello"));
}

#[test]
fn get_returns_none_for_unknown() {
    let ctx = Context::new();
    assert!(ctx.get(999).is_none());
}

#[test]
fn fragments_returns_all_in_order() {
    let mut ctx = Context::new();
    let id1 = ctx.append(Fragment::system("s"));
    let id2 = ctx.append(Fragment::user("u"));
    let id3 = ctx.append(Fragment::assistant("a"));
    let frags = ctx.fragments();
    assert_eq!(frags.len(), 3);
    assert_eq!(frags[0].id, id1);
    assert_eq!(frags[1].id, id2);
    assert_eq!(frags[2].id, id3);
}

#[test]
fn next_id_increments_after_append() {
    let mut ctx = Context::new();
    assert_eq!(ctx.next_id(), 1);
    ctx.append(Fragment::system("a"));
    assert_eq!(ctx.next_id(), 2);
    ctx.append(Fragment::user("b"));
    assert_eq!(ctx.next_id(), 3);
}

#[test]
fn next_id_increments_after_insert() {
    let mut ctx = Context::new();
    let id = ctx.append(Fragment::system("a"));
    assert_eq!(ctx.next_id(), 2);
    ctx.insert(id, Fragment::user("b"));
    assert_eq!(ctx.next_id(), 3);
}

#[test]
fn next_id_does_not_decrement_after_remove() {
    let mut ctx = Context::new();
    let id = ctx.append(Fragment::system("a"));
    ctx.remove(id);
    assert_eq!(ctx.next_id(), 2); // still 2, ids are never reused
}

#[test]
fn ids_are_never_reused() {
    let mut ctx = Context::new();
    let id1 = ctx.append(Fragment::system("a"));
    ctx.remove(id1);
    let id2 = ctx.append(Fragment::system("b"));
    assert_ne!(id1, id2);
    assert_eq!(id2, 2);
}

#[test]
fn complex_sequence() {
    let mut ctx = Context::new();
    // Build: [system, env, user]
    let sys_id = ctx.append(Fragment::system("sys"));
    let env_id = ctx.insert(sys_id, Fragment::user("env").with_tag("env"));
    let user_id = ctx.append(Fragment::user("hello"));

    assert_eq!(ctx.len(), 3);

    // Replace user
    ctx.replace(user_id, Fragment::user("hello world"));
    assert_eq!(ctx.get(user_id).unwrap().as_text(), Some("hello world"));

    // Remove env
    ctx.remove(env_id);
    assert_eq!(ctx.len(), 2);
    assert!(ctx.get(env_id).is_none());

    // Insert after system
    let mem_id = ctx.insert(sys_id, Fragment::user("memory").with_tag("memory"));
    assert_eq!(ctx.len(), 3);

    // Verify order: system, memory, user
    let frags = ctx.fragments();
    assert_eq!(frags[0].id, sys_id);
    assert_eq!(frags[1].id, mem_id);
    assert_eq!(frags[2].id, user_id);
}
