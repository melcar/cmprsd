//Naive binary tree implementation
use std::{collections::VecDeque, ops::Deref};
#[derive(PartialEq, PartialOrd, Ord, Eq, Debug, Clone)]
pub enum Tree<T: std::cmp::Ord + Copy> {
    Node {
        content: T,
        left: Box<Tree<T>>,//should be optionals
        right: Box<Tree<T>>,
    },
    Leaf(T),
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

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn to_breadth_first_search(&self) -> Vec<T> {
        let mut visited_nodes: VecDeque<&Tree<T>> = VecDeque::new();
        let mut bfs_nodes: Vec<T> = Vec::new();
        visited_nodes.push_back(self);
        while !visited_nodes.is_empty() {
            let node = visited_nodes
                .pop_front()
                .expect("can't be empty at this point");

            match node {
                Tree::Node {
                    content: _,
                    left,
                    right,
                } => {
                    visited_nodes.push_back(left);
                    visited_nodes.push_back(right)
                }
                _ => (),
            }

            bfs_nodes.push(*node.get_value());
        }
        bfs_nodes
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
