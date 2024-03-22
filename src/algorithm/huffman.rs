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
        compressed_data: CompressedData,
    },
}

#[derive(Clone)]
pub struct CompressedData {
    bits: Vec<u8>,
    meaningful_bits: u8,
}

impl CompressedData {
    pub fn get_empty() -> CompressedData {
        CompressedData {
            bits: vec![0],
            meaningful_bits: 0,
        }
    }

    pub fn from_bits_as_u8(bits_as_u8: &[u8]) -> CompressedData {
        let mut bits = vec![0; (bits_as_u8.len() - 1) / 8 + 1];
        let mut meaningful_bits: u8 = (bits_as_u8.len() % 8) as u8;
        if meaningful_bits == 0 {
            meaningful_bits = 8
        }
        bits_as_u8.iter().enumerate().for_each(|(index, bit)| {
            *bits.get_mut(index / 8).expect("should be in bound") |=
                bit * 2_u8.pow((7 - (index % 8)).try_into().unwrap())
        });

        CompressedData {
            bits,
            meaningful_bits,
        }
    }

    pub fn len(&self) -> usize {
        self.bits.len() * 8 - (8 - self.meaningful_bits as usize)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    // add some padding with 0s to the left
    pub fn pad(&mut self, padding_size: u8) {
        match padding_size {
            0 => (),
            i if i < 8 => {
                if padding_size + self.meaningful_bits > 8 {
                    self.bits.push(0);
                }
                if padding_size + self.meaningful_bits == 8 {
                    self.meaningful_bits = 8;
                } else {
                    self.meaningful_bits = (padding_size + self.meaningful_bits) % 8;
                }

                (1..self.bits.len()).rev().for_each(|i| {
                    let previous_byte = *self.bits.get(i - 1).unwrap();
                    let byte = self.bits.get_mut(i).unwrap();
                    *byte = (*byte >> padding_size) | (previous_byte << (8 - padding_size));
                });
                self.bits[0] >>= padding_size;
            }
            _ => unimplemented!(),
        }
    }

    pub fn add(&mut self, other: &CompressedData) {
        // potention to use map and to directly add all the "bytes" to self and compute startinf from the end all the bytes that need data to be added and what the data is.
        match self.meaningful_bits {
            8 => {
                self.meaningful_bits = other.meaningful_bits;
                self.bits.append(&mut other.bits.clone())
            }
            _ => {
                let mut other = other.clone();
                other.pad(self.meaningful_bits);
                self.bits
                    .resize(self.bits.len() + other.bits.len() - 1, 0_u8);
                self.meaningful_bits = other.meaningful_bits;

                let len = self.bits.len();
                let (_, to_write_to) = self.bits.split_at_mut(len - other.bits.len());

                to_write_to
                    .iter_mut()
                    .zip(other.bits.iter())
                    .for_each(|(to_write_to_, to_write)| *to_write_to_ |= to_write);
            }
        }
    }
}

impl Huffman {
    pub fn compress(data: &str) -> Result<crate::huffman::Huffman, CompressionError> {
        //mapping of char to Compressed value. as string for now
        if data.is_empty() {
            return Err(CompressionError::NoDataToCompress);
        }

        let mut compressed_mapping: HashMap<char, CompressedData> = HashMap::new();
        let frequencies = compute_frequencies(data);
        if frequencies.len() == 1 {
            return Err(CompressionError::DataCannotBeCompressed);
        }
        build_huffman_tree(&frequencies.clone())
            .leaf_paths()
            .iter()
            .for_each(|(directions, node)| {
                compressed_mapping.insert(
                    node.character
                        .expect("Leaves of huffman tree should all contain a character."),
                    CompressedData::from_bits_as_u8(
                        &directions.iter().map(|d| (*d).into()).collect::<Vec<u8>>(),
                    ),
                );
            });

        Ok(Huffman::Compressed {
            frequencies,
            compressed_data: data.chars().fold(
                CompressedData::get_empty(),
                |mut compressed_data, character| {
                    let compressed_char = compressed_mapping.get(&character).unwrap();
                    compressed_data.add(compressed_char);
                    compressed_data
                },
            ),
        })
    }

    pub fn decompress(&self) -> String {
        let Huffman::Compressed {
            frequencies,
            compressed_data,
        } = self;
        let tree = build_huffman_tree(frequencies);
        let mut directions: Vec<Direction>;
        if compressed_data.bits.len() >= 2 {
            directions = compressed_data.bits[0..=compressed_data.bits.len() - 2]
                .iter()
                .flat_map(|byte| bytes_to_direction(*byte, 8))
                .collect();
        } else {
            directions = Vec::new();
        }
        directions.append(&mut bytes_to_direction(
            *compressed_data
                .bits
                .last()
                .expect("should hold data to uncompress"),
            compressed_data.meaningful_bits,
        ));
        directions_to_string(&directions, &tree)
    }
}

#[derive(Debug, Clone)]
pub enum CompressionError {
    NoDataToCompress,
    DataCannotBeCompressed,
}

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
            Tree::Node { content, .. } => content.get_frequency(),
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

pub fn build_huffman_tree(frequencies: &[Frequency]) -> Tree<Frequency> {
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

pub fn bytes_to_direction(byte: u8, cursor: u8) -> Vec<Direction> {
    let mut directions: Vec<Direction> = Vec::new();
    (1..=cursor).for_each(|i| match (byte >> (8_u8 - i)) % 2 {
        0 => directions.push(Right),
        1 => directions.push(Left),
        _ => unreachable!(),
    });
    directions
}

pub fn directions_to_string(directions: &[Direction], root: &Tree<Frequency>) -> String {
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
