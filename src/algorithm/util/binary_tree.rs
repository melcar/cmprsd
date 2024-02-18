//Naive binary tree implementation
pub enum Tree<T: std::cmp::Ord> {
    Node {
        left: Box<Tree<T>>,
        right: Box<Tree<T>>,
    },
    Leaf(Option<T>),
}

