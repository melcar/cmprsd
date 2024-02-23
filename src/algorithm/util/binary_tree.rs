//Naive binary tree implementation

#[derive(PartialEq, PartialOrd, Ord, Eq, Debug, Clone)]
pub enum Tree<T: std::cmp::Ord> {
    Node {
        content: T,
        left: Box<Tree<T>>,
        right: Box<Tree<T>>,
    },
    Leaf(T),
}

impl<T: std::cmp::Ord> Tree<T> {
    pub fn len(&self) -> usize {
        match self {
            Tree::Leaf(_) => 1,
            Tree::Node {
                content: _,
                left,
                right,
            } => 1 + left.len() + right.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn to_breadth_first_search() {
        todo!("implement breadth first earch")
    }

    pub fn build_internal_node(content: T, left: Tree<T>, right: Tree<T>) -> Tree<T> {
        Tree::Node {
            content,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn get_value(&self) -> &T {
        match self {
            Tree::Leaf(n) => n,
            Tree::Node {
                content,
                left: _,
                right: _,
            } => content,
        }
    }
}
