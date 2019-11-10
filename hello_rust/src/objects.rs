/// typical class: where data fields and methods are in a single construct
/// class Door {
///     private bool is_open = false;
///
///     public void open() {
///         this.is_open = true;
///     }
/// }
/// Rust emphasizes the separation between those(objects&behaviours)
/// by declaring a ```struct``` for data and an ```impl``` part for the methods/functions

struct Door {
    is_open: bool,
}

impl Door {
    fn new(is_open: bool) -> Door {
        Door { is_open }
    }
}

trait Openable {
    fn open(&mut self);
}

impl Openable for Door {
    fn open(&mut self) {
        self.is_open = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_door() {
        let mut door = Door::new(false);
        door.open();
        assert!(door.is_open);
    }
}
