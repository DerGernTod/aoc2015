use std::{cmp::Ordering, rc::Rc};

#[derive(Clone, PartialEq, Eq)]
pub struct Node<'a> {
    pub current_path: Rc<Vec<&'a String>>,
    pub happiness: i32,
}

impl<'a> Ord for Node<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.happiness.cmp(&other.happiness)
            .then_with(|| self.current_path.cmp(&other.current_path))
    }
}
impl<'a> PartialOrd for Node<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}