//!Huffman coding and decoding
use super::util::frequency::Frequency;
use super::util::binary_tree::{
    Direction,
    Direction::{Left, Right},
    Tree,
};
use std::{
    cmp::Reverse,
    collections::{BTreeMap, BinaryHeap, HashMap},
};

pub enum Huffman {
    Compressed {
        frequencies: Vec<Frequency>,
        compressed_data: CompressedData,
    },
}

/// Struct that represents compressed data.
#[derive(Clone)]
pub struct CompressedData {
    /// Vector of compressed bytes. each compressed block is stored at a bit level.  
    /// TODO: allow to use u8, u16, u32, u64 :
    pub bits: Vec<u8>,
    /// As we store blocks of 8 bits, not all 8 bits are used at all times.
    /// We therefore need a way to store how many bits are used or not used in the last byte.
    /// Currently I store how many of these bits are used.
    pub meaningful_bits: u8,
}

impl CompressedData {
    /// Returns a compressed data entity which stores 0 bits.
    pub fn get_empty() -> CompressedData {
        CompressedData {
            bits: vec![0],
            meaningful_bits: 0,
        }
    }

    /// Take as input a vector of 0s and 1s and store them as bits.
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

    /// size of the compressed data in bits
    pub fn len(&self) -> usize {
        self.bits.len() * 8 - (8 - self.meaningful_bits as usize)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Helper function to pad a block of bits. Usefull when combining blocks.
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

    /// Append bits of the other compressed data to self.
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

/// Enum responsible for the compression and decompression of our input.
impl Huffman {
    /// Compress takes as input the data as a string and returns the data compressed in bits.
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

    /// A compressed huffman data can be decompressed to find back the original text using this method.
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

/// Represents errors that can occure during compression.
/// So far only two are represented.
#[derive(Debug, Clone)]
pub enum CompressionError {
    /// Is what happens when you try to compress and empty string.
    NoDataToCompress,
    /// It happens when the text to compressed is made of only one repeated character.
    DataCannotBeCompressed,
}


// take an huffman tree and create a map of character to encoding
pub fn huffman_tree_to_map() {}

pub fn combine_nodes(
    mut frequency_nodes: BinaryHeap<Reverse<Tree<Frequency>>>,
) -> BinaryHeap<Reverse<Tree<Frequency>>> {
    let smallest = frequency_nodes
        .pop()
        .expect("binary tree shouls not be empty");
    let second_smallest = frequency_nodes
        .pop()
        .expect("binary tree should not be empty");

    let new_node = Tree::build_internal_node(
        Frequency {
            count: smallest.0.get_value().count + second_smallest.0.get_value().count,
            character: None,
        },
        smallest.0,
        second_smallest.0,
    );
    frequency_nodes.push(Reverse(new_node));
    frequency_nodes
}

pub fn build_huffman_tree(frequencies: &[Frequency]) -> Tree<Frequency> {
    let mut frequency_nodes = frequencies
        .iter()
        .copied()
        .map(Tree::Leaf)
        .map(Reverse)
        .collect::<BinaryHeap<Reverse<Tree<Frequency>>>>();
    for _ in 1..frequencies.len() {
        frequency_nodes = combine_nodes(frequency_nodes);
    }
    assert!(frequency_nodes.len() == 1);
    frequency_nodes.pop().unwrap().0
}

pub fn compute_frequencies(data: &str) -> Vec<Frequency> {
    let mut char_counts: BTreeMap<char, usize> = BTreeMap::new();
    data.chars().for_each(|c| {
        char_counts.entry(c).and_modify(|n| *n += 1).or_insert(1);
    });

    Vec::from_iter(char_counts.into_iter().map(|(c, f)| Frequency {
        count: f,
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
