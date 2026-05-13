use machine::Inbox;

#[test]
fn new_inbox_is_empty() {
    let inbox = Inbox::new();
    assert!(inbox.is_empty());
    assert_eq!(inbox.len(), 0);
}

#[test]
fn push_and_pop() {
    let mut inbox = Inbox::new();
    inbox.push(machine::Fragment::system("hello"));
    assert!(!inbox.is_empty());
    assert_eq!(inbox.len(), 1);

    let f = inbox.pop().unwrap();
    assert_eq!(f.as_text(), Some("hello"));
    assert!(inbox.is_empty());
}

#[test]
fn pop_empty_returns_none() {
    let mut inbox = Inbox::new();
    assert!(inbox.pop().is_none());
}

#[test]
fn peek_does_not_remove() {
    let mut inbox = Inbox::new();
    inbox.push(machine::Fragment::user("q"));
    assert_eq!(inbox.peek().unwrap().as_text(), Some("q"));
    assert_eq!(inbox.len(), 1);
}

#[test]
fn peek_empty_returns_none() {
    let inbox = Inbox::new();
    assert!(inbox.peek().is_none());
}

#[test]
fn fifo_order() {
    let mut inbox = Inbox::new();
    inbox.push(machine::Fragment::system("first"));
    inbox.push(machine::Fragment::user("second"));
    inbox.push(machine::Fragment::assistant("third"));

    assert_eq!(inbox.pop().unwrap().as_text(), Some("first"));
    assert_eq!(inbox.pop().unwrap().as_text(), Some("second"));
    assert_eq!(inbox.pop().unwrap().as_text(), Some("third"));
    assert!(inbox.is_empty());
}

#[test]
fn len_tracks_correctly() {
    let mut inbox = Inbox::new();
    assert_eq!(inbox.len(), 0);
    inbox.push(machine::Fragment::system("a"));
    assert_eq!(inbox.len(), 1);
    inbox.push(machine::Fragment::user("b"));
    assert_eq!(inbox.len(), 2);
    inbox.pop();
    assert_eq!(inbox.len(), 1);
    inbox.pop();
    assert_eq!(inbox.len(), 0);
}
