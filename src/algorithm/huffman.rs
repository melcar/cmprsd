use super::util::binary_tree::Tree;
use std::collections::BTreeMap;

#[derive(PartialEq, PartialOrd, Ord, Eq, Debug, Clone, Copy)]
pub struct Frequency {
    pub frequency: u16, // frequency is a value between 0 and 65536 and is equal to n/65536
    pub character: char,
}

pub fn combine_nodes(mut frequency_nodes: Vec<(Tree<Frequency>, u16)>) -> Vec<(Tree<Frequency>, u16)>{
    frequency_nodes.sort_by(|a, b| b.1.cmp(&a.1));
    let smallest = frequency_nodes
        .pop()
        .expect("binary tree shouls not be empty");
    let second_smallest = frequency_nodes
        .pop()
        .expect("binary tree should not be empty");
    let new_node = Tree::Node {
        left: (Box::new(second_smallest.0)),
        right: (Box::new(smallest.0)),
    };
    frequency_nodes.push((new_node, smallest.1 + second_smallest.1));
    frequency_nodes
}

pub fn build_huffman_tree(frequencies: Vec<Frequency>) -> Tree<Frequency> {
    let mut frequency_nodes = frequencies
        .iter()
        .copied()
        .map(Tree::Leaf)
        .map(|c| match c {
            Tree::Leaf(f) => (c, f.frequency),
            _ => unreachable!(),
        })
        .collect::<Vec<(Tree<Frequency>, u16)>>();
    for _ in 1..frequencies.len() {
        frequency_nodes = combine_nodes(frequency_nodes);
    }
    assert!(frequency_nodes.len() == 1);
    frequency_nodes[0].0.clone()
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
        character: c,
    }))
}

pub fn compress(data: &str) -> crate::huffman::Huffman {
    todo!("todo")
}

pub fn decompress(encoded: &Huffman) -> String {
    todo!("todo")
}
