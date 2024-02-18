use super::util::binary_tree::Tree;
use std::{collections::BTreeMap, ops::Add};

#[derive(PartialEq, PartialOrd, Ord, Eq, Debug)]
pub struct Frequency {
    pub frequency: u16, // frequency is a value between 0 and 65536 and is equal to n/65536
    pub character: char,
}

fn build_huffman_tree(frequencies: Vec<Frequency>) -> Tree<Frequency> {
    todo!()
}

pub enum Huffman {
    Encoded {
        frequencies: Tree<Frequency>,
        encoded: String,
    },
    Decoded(String),
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
