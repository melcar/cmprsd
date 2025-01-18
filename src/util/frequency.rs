use crate::util::binary_tree::Tree;
use core::fmt;
/// Struct that represents the frequency of the different characters found in a string.
#[derive(PartialEq, PartialOrd, Ord, Eq, Debug, Clone, Copy)]
pub struct Frequency {
    /// frequency is a value between 0 and 65536 and is equal to n/65536
    pub count: usize,
    // I could just have total count instead of frequency actually
    /// char that the frequency represents.
    /// It is optional as in when we store frequencies in the Huffman tree
    /// we have nodes that hold a combined frequency of the two children but do not hold any character.
    pub character: Option<char>,
}

impl Frequency {
    /// instanciate a frequency.
    pub fn build_frequency(count: usize, character: Option<char>) -> Frequency {
        Frequency {
            count,
            character,
        }
    }
}

impl fmt::Display for Frequency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({:.4},{})",
            self.count,
            &(match self.character {
                None => "None".to_string(),
                Some(c) => c.to_string(),
            })
        )
    }
}

impl Tree<Frequency> {
    pub fn get_count(&self) -> usize{
        match self {
            Tree::Leaf(leaf) => leaf.count,
            Tree::Node { content, .. } => content.count,
        }
    }
}
