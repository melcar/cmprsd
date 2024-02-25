use super::util::binary_tree::Tree;
use core::fmt;
use std::{cmp::Ordering, collections::BTreeMap};

#[derive(PartialEq, PartialOrd, Ord, Eq, Debug, Clone, Copy)]
pub struct Frequency {
    pub frequency: u16, // frequency is a value between 0 and 65536 and is equal to n/65536
    pub character: Option<char>,
}
impl Frequency {
    pub fn build_frequency(frequency: u16, character: Option<char>) -> Frequency {
        Frequency {
            frequency,
            character,
        }
    }

    pub fn get_frequency(&self) -> f64 {
        (self.frequency as f64) / std::u16::MAX as f64
    }
}

impl fmt::Display for Frequency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({:.4},{})",
            self.get_frequency(),
            &(match self.character {
                None => "None".to_string(),
                Some(c) => c.to_string(),
            })
        )
    }
}

impl Tree<Frequency> {
    pub fn get_frequency(&self) -> f64 {
        match self {
            Tree::Leaf(leaf) => leaf.get_frequency(),
            Tree::Node {
                content,
                left: _,
                right: _,
            } => content.get_frequency(),
        }
    }
}
// take an huffman tree and create a map of character to encoding
pub fn huffman_tree_to_map() {}

pub fn combine_nodes(mut frequency_nodes: Vec<Tree<Frequency>>) -> Vec<Tree<Frequency>> {
    frequency_nodes.sort_by(|a, b| b.cmp(a));
    let smallest = frequency_nodes
        .pop()
        .expect("binary tree shouls not be empty");
    let second_smallest = frequency_nodes
        .pop()
        .expect("binary tree should not be empty");
    let new_node = Tree::build_internal_node(
        Frequency {
            frequency: smallest.get_value().frequency + second_smallest.get_value().frequency,
            character: None,
        },
        smallest,
        second_smallest,
    );
    frequency_nodes.push(new_node);
    frequency_nodes
}

pub fn build_huffman_tree(frequencies: Vec<Frequency>) -> Tree<Frequency> {
    let mut frequency_nodes = frequencies
        .iter()
        .copied()
        .map(Tree::Leaf)
        .collect::<Vec<Tree<Frequency>>>();
    for _ in 1..frequencies.len() {
        frequency_nodes = combine_nodes(frequency_nodes);
    }
    assert!(frequency_nodes.len() == 1);
    frequency_nodes[0].clone()
}

pub enum Huffman {
    Encoded {
        frequencies: Tree<Frequency>,
        encoded: String,
    },
}

pub fn compute_frequencies(data: &str) -> Vec<Frequency> {
    let mut char_counts: BTreeMap<char, usize> = BTreeMap::new();
    data.chars().for_each(|c| {
        char_counts.entry(c).and_modify(|n| *n += 1).or_insert(1);
    });

    Vec::from_iter(char_counts.into_iter().map(|(c, f)| Frequency {
        frequency: { (((f as f64) / (data.len() as f64)) * (std::u16::MAX as f64)) as u16 },
        character: Some(c),
    }))
}

pub fn compress(data: &str) -> crate::huffman::Huffman {
    todo!("todo")
}

pub fn decompress(encoded: &Huffman) -> String {
    todo!("todo")
}
