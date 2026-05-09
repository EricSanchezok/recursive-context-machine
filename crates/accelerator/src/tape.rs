use crate::fragment::Fragment;

/// The Turing machine tape — a fixed-address sequence of cells.
///
/// Each cell may hold a Fragment or be blank (None).
/// The head position is an index into the cell array.
/// Erasing a cell leaves it blank; neighbours keep their addresses.
pub struct Tape {
    cells: Vec<Option<Fragment>>,
    position: usize,
}

impl Tape {
    pub fn new() -> Self {
        Self {
            cells: Vec::new(),
            position: 0,
        }
    }

    // ── Movement ──

    pub fn left(&mut self) {
        if self.position > 0 {
            self.position -= 1;
        }
    }

    pub fn right(&mut self) {
        self.position += 1;
    }

    pub fn goto(&mut self, pos: usize) {
        self.position = pos;
    }

    // ── Read / write ──

    pub fn read(&self) -> Option<&Fragment> {
        self.cells.get(self.position).and_then(|c| c.as_ref())
    }

    pub fn write(&mut self, f: Fragment) {
        if self.position >= self.cells.len() {
            self.cells.resize(self.position + 1, None);
        }
        self.cells[self.position] = Some(f);
    }

    pub fn erase(&mut self) {
        if self.position < self.cells.len() {
            self.cells[self.position] = None;
        }
    }

    // ── Query ──

    pub fn len(&self) -> usize {
        self.cells.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }

    pub fn is_end(&self) -> bool {
        self.position >= self.cells.len()
    }

    pub fn position(&self) -> usize {
        self.position
    }

    /// All non-blank fragments in address order.
    pub fn fragments(&self) -> Vec<&Fragment> {
        self.cells.iter().filter_map(|c| c.as_ref()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fragment::Role;

    #[test]
    fn test_write_and_read() {
        let mut tape = Tape::new();
        tape.write(Fragment::system("hello"));
        assert_eq!(tape.read().unwrap().as_text(), Some("hello"));
        assert_eq!(tape.position(), 0);
    }

    #[test]
    fn test_erase_preserves_position() {
        let mut tape = Tape::new();
        tape.write(Fragment::user_text("a"));
        tape.right();
        tape.write(Fragment::user_text("b"));
        tape.right();
        tape.write(Fragment::user_text("c"));

        // Erase middle cell
        tape.goto(1);
        tape.erase();

        // Positions preserved
        tape.goto(0);
        assert_eq!(tape.read().unwrap().as_text(), Some("a"));
        tape.goto(1);
        assert!(tape.read().is_none());
        tape.goto(2);
        assert_eq!(tape.read().unwrap().as_text(), Some("c"));
    }

    #[test]
    fn test_fragments_skips_blanks() {
        let mut tape = Tape::new();
        tape.write(Fragment::system("s"));
        tape.right();
        tape.write(Fragment::user_text("u"));
        tape.right();
        // leave pos=2 blank
        tape.right();
        tape.write(Fragment::assistant_text("a"));

        let frags = tape.fragments();
        assert_eq!(frags.len(), 3);
        assert_eq!(frags[0].role, Role::System);
        assert_eq!(frags[1].role, Role::User);
        assert_eq!(frags[2].role, Role::Assistant);
    }

    #[test]
    fn test_movement() {
        let mut tape = Tape::new();
        tape.write(Fragment::user_text("0"));
        tape.right();
        tape.write(Fragment::user_text("1"));
        tape.right();
        tape.write(Fragment::user_text("2"));

        tape.left();
        assert_eq!(tape.position(), 1);
        tape.left();
        assert_eq!(tape.position(), 0);
        tape.left(); // clamped
        assert_eq!(tape.position(), 0);
    }
}
