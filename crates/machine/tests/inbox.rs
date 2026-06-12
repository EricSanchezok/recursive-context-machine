use machine::{Fragment, Inbox};

#[test]
fn fifo_order() {
    let mut inbox = Inbox::new();
    inbox.push(Fragment::system("first"));
    inbox.push(Fragment::user("second"));
    inbox.push(Fragment::assistant("third"));

    assert_eq!(inbox.pop().unwrap().fragment.as_text(), Some("first"));
    assert_eq!(inbox.pop().unwrap().fragment.as_text(), Some("second"));
    assert_eq!(inbox.pop().unwrap().fragment.as_text(), Some("third"));
    assert!(inbox.is_empty());
}

#[test]
fn pop_empty_is_none() {
    let mut inbox = Inbox::new();
    assert!(inbox.pop().is_none());
}

#[test]
fn peek_preserves_length() {
    let mut inbox = Inbox::new();
    inbox.push(Fragment::user("q"));
    assert_eq!(inbox.peek().unwrap().fragment.as_text(), Some("q"));
    assert_eq!(inbox.len(), 1);
}

#[test]
fn length_tracks_push_pop() {
    let mut inbox = Inbox::new();
    inbox.push(Fragment::system("a"));
    inbox.push(Fragment::user("b"));
    assert_eq!(inbox.len(), 2);
    inbox.pop();
    assert_eq!(inbox.len(), 1);
    inbox.pop();
    assert_eq!(inbox.len(), 0);
}
