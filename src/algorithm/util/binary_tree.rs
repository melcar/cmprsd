//Naive binary tree implementation

#[derive(PartialEq, PartialOrd, Ord, Eq, Debug, Clone)]
pub enum Tree<T: std::cmp::Ord> {
    Node {
        left: Box<Tree<T>>,
        right: Box<Tree<T>>,
    },
    Leaf(T),
}

impl<T: std::cmp::Ord> Tree<T> {
    pub fn len(&self) -> usize {
        match self {
            Tree::Leaf(_) => 1,
            Tree::Node { left, right } => 1 + left.len() + right.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
