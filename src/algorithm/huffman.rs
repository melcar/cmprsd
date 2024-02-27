use super::util::binary_tree::{
    Direction,
    Direction::{Left, Right},
    Tree,
};
use core::fmt;
use std::collections::{BTreeMap, HashMap};

pub enum Huffman {
    Compressed {
        frequencies: Vec<Frequency>,
        compressed: Vec<u8>, //type should be variable depending on number of different character possible
        cursor: u8,          // how many bits do discard in the last byte we read
    },
}

impl Huffman {
    pub fn compress(data: &str) -> Result<crate::huffman::Huffman, CompressionError> {
        //mapping of char to Compressed value. as string for now
        if data.is_empty() {
            return Err(CompressionError);
        }
        let mut to_compressed: HashMap<char, Vec<Direction>> = HashMap::new();
        let frequencies = compute_frequencies(data);
        build_huffman_tree(frequencies.clone())
            .leaf_paths()
            .iter()
            .for_each(|(directions, node)| {
                to_compressed.insert(
                    node.character
                        .expect("Leaves of humman tree should all contain a character."),
                    directions.clone(),
                );
            });

        let mut compressed_message = data
            .chars()
            .map(|character| {
                to_compressed
                    .get(&character)
                    .expect("Compressed character should be part of the huffman tree.")
            })
            .fold((vec![0], 0_u8), |(compressed_data, cursor), directions| {
                let mut cursor = cursor;
                let mut compressed_data = compressed_data.clone();
                directions.iter().for_each(|direction| {
                    if cursor == 8 {
                        cursor = 0;
                        compressed_data.push(0);
                    }
                    let mut current_byte = *compressed_data
                        .last()
                        .expect("compressed data is not empty.");
                    current_byte <<= 1;
                    match *direction {
                        Left => current_byte |= 1_u8,
                        Right => (),
                    };
                    cursor += 1;
                    *compressed_data.last_mut().unwrap() = current_byte;
                });
                (compressed_data, cursor)
            });
        *compressed_message.0.last_mut().unwrap() =
            compressed_message.0.last().unwrap() << (8 - compressed_message.1);
        Ok(Huffman::Compressed {
            frequencies,
            compressed: compressed_message.0,
            cursor: compressed_message.1,
        })
    }

    pub fn decompress(&self) -> String {
        let Huffman::Compressed {
            frequencies,
            compressed,
            cursor,
        } = self;
        let tree = build_huffman_tree(frequencies.to_owned());
        let mut directions: Vec<Direction>;
        if compressed.len() >= 2 {
            directions = compressed[0..=compressed.len() - 2]
                .iter()
                .flat_map(|byte| bytes_to_direction(byte, &8))
                .collect();
        } else {
            directions = Vec::new();
        }
        directions.append(&mut bytes_to_direction(
            compressed.last().expect("should hold data to uncompress"),
            cursor,
        ));
        directions_to_string(directions, &tree)
    }
}

#[derive(Debug, Clone)]
pub struct CompressionError;

#[derive(PartialEq, PartialOrd, Ord, Eq, Debug, Clone, Copy)]
pub struct Frequency {
    pub frequency: u16, // frequency is a value between 0 and 65536 and is equal to n/65536
    // I could just have total count instead of frequency actually
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

pub fn bytes_to_direction(byte: &u8, cursor: &u8) -> Vec<Direction> {
    let mut directions: Vec<Direction> = Vec::new();
    (1..=*cursor).for_each(|i| match (byte >> (8_u8 - i)) % 2 {
        0 => directions.push(Right),
        1 => directions.push(Left),
        _ => unreachable!(),
    });
    directions
}

pub fn directions_to_string(directions: Vec<Direction>, root: &Tree<Frequency>) -> String {
    directions
        .iter()
        .fold(
            ("".to_string(), root),
            |(decompressed_data, current_node), direction| match &current_node
                .get_value_from_directions(*direction)
                .expect("data corrupted")
            {
                Tree::Leaf(frequency) => (
                    decompressed_data
                        + &frequency
                            .character
                            .expect("leaf must hold a character")
                            .to_string(),
                    root,
                ),
                current_node => (decompressed_data, current_node),
            },
        )
        .0
}
