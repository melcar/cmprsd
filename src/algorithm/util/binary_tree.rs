//Naive binary tree implementation
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    Left = 1,
    Right = 0,
}

impl From<Direction> for u8 {
    fn from(direction: Direction) -> u8 {
        direction as u8
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Tree<T: std::cmp::Ord> {
    Node {
        content: T,
        left: Box<Tree<T>>, //should be optionals
        right: Box<Tree<T>>,
    },
    Leaf(T),
}

impl<T: std::cmp::Ord + Copy> PartialOrd for Tree<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: std::cmp::Ord + Copy> Ord for Tree<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.get_value().cmp(other.get_value())
    }
}

impl<T: std::cmp::Ord + Copy> Tree<T> {
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

    pub fn height(&self) -> usize {
        match self {
            Tree::Leaf(_) => 1,
            Tree::Node {
                content: _,
                left,
                right,
            } => 1 + std::cmp::max(left.height(), right.height()),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn to_breadth_first_search(&self) -> Vec<T> {
        let mut visited_nodes: VecDeque<&Tree<T>> = VecDeque::new();
        let mut bfs_nodes: Vec<T> = Vec::new();
        visited_nodes.push_back(self);
        while let Some(node) = visited_nodes.pop_front() {
            if let Tree::Node {
                content: _,
                left,
                right,
            } = node
            {
                visited_nodes.push_back(left);
                visited_nodes.push_back(right)
            }

            bfs_nodes.push(*node.get_value());
        }
        bfs_nodes
    }

    pub fn leaf_paths(&self) -> Vec<(Vec<Direction>, T)> {
        let mut visited_nodes: VecDeque<(Vec<Direction>, &Tree<T>)> = VecDeque::new();
        visited_nodes.push_back((Vec::<Direction>::new(), self));
        let mut nodes_codes = Vec::<(Vec<Direction>, T)>::new();
        while let Some((direction, node)) = visited_nodes.pop_back() {
            let mut left_direction = direction.clone();
            left_direction.push(Direction::Left);
            let mut right_direction = direction.clone();
            right_direction.push(Direction::Right);
            match node {
                Tree::Node {
                    content: _,
                    left,
                    right,
                } => {
                    visited_nodes.push_back((left_direction, left));
                    visited_nodes.push_back((right_direction, right))
                }
                Tree::Leaf(n) => nodes_codes.push((direction, *n)),
            }
        }
        nodes_codes
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
            Tree::Node { content, .. } => content,
        }
    }

    pub fn get_all_values(&self) -> Vec<T> {
        match self {
            Tree::Leaf(n) => vec![*n],
            Tree::Node {
                content,
                left,
                right,
            } => {
                let mut all_values = vec![*content];
                all_values.append(&mut left.get_all_values());
                all_values.append(&mut right.get_all_values());
                all_values
            }
        }
    }

    pub fn get_value_from_directions(&self, direction: Direction) -> Option<&Tree<T>> {
        if let Tree::Node {
            content: _,
            left,
            right,
        } = self
        {
            match direction {
                Direction::Left => Some(left),
                Direction::Right => Some(right),
            }
        } else {
            None
        }
    }
}
