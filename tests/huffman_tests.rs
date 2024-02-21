use cmprsd::algorithm::huffman::{self, build_huffman_tree, combine_nodes, Frequency, Huffman};
use cmprsd::algorithm::util::binary_tree::Tree;
use rand::{distributions::Alphanumeric, thread_rng, Rng};

const LOREM_IPSUM : &str ="Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

const EPSILON: f64 = 0.00001_f64;
//make a macro out of it
fn close_to(a: f64, b: f64, delta: f64) -> bool {
    a - b < delta
}

#[test]
pub fn huffman_empty_string() {
    let compressed = huffman::compress("");
}

#[test]
pub fn huffman_repeating_string() {
    let compressed = huffman::compress("");
}

#[test]
pub fn huffman_two_characters() {
    let compressed = huffman::compress("");
}

#[test]
pub fn huffman_random() {
    let compressed = huffman::compress("");
}

#[test]
pub fn huffman_lorem() {
    let compressed = huffman::compress(LOREM_IPSUM);
}

#[test]
pub fn compute_frequencies_empty_string() {
    assert!(huffman::compute_frequencies("").is_empty());
}

#[test]
pub fn compute_frequencies_one_character() {
    assert_eq!(
        huffman::compute_frequencies("a")[0],
        huffman::Frequency {
            character: 'a',
            frequency: std::u16::MAX
        }
    )
}

#[test]
pub fn compute_frequencies_two_character_50_50() {
    let frequencies = huffman::compute_frequencies("abab");
    assert_eq!(
        frequencies[0],
        huffman::Frequency {
            character: 'a',
            frequency: std::u16::MAX / 2
        },
    );
    assert_eq!(
        frequencies[1],
        huffman::Frequency {
            character: 'b',
            frequency: std::u16::MAX / 2
        },
    );
}

#[test]
pub fn compute_frequencies_two_character_25_75() {
    let frequencies = huffman::compute_frequencies("bbab");
    assert_eq!(
        frequencies[0],
        huffman::Frequency {
            character: 'a',
            frequency: std::u16::MAX / 4
        },
    );
    assert_eq!(
        frequencies[1],
        huffman::Frequency {
            character: 'b',
            frequency: 3 * (std::u16::MAX / 4)
        },
    );
}

fn check_frequencies(frequencies: Vec<Frequency>, string: &str) {
    for frequency in frequencies.iter() {
        assert!(close_to(
            frequency.get_frequency(),
            string.chars().filter(|&c| c == frequency.character).count() as f64
                / string.len() as f64,
            EPSILON
        ));
    }
}

#[test]
pub fn compute_frequencies_lorem_ipsum() {
    check_frequencies(huffman::compute_frequencies(LOREM_IPSUM), LOREM_IPSUM);
}

#[test]
pub fn compute_frequencies_random_long_string() {
    let mut rng = thread_rng();

    let string_length: usize = rng.gen_range(500_000..1_000_000);

    let random_string: String = (&mut rng)
        .sample_iter(Alphanumeric)
        .take(string_length)
        .map(char::from)
        .collect();

    check_frequencies(huffman::compute_frequencies(&random_string), &random_string)
}

#[test]
pub fn combine_nodes_2() {
    let frequencies = huffman::compute_frequencies("ab")
        .iter()
        .copied()
        .map(Tree::Leaf)
        .map(|c| match c {
            Tree::Leaf(f) => (c, f.frequency),
            _ => unreachable!(),
        })
        .collect::<Vec<(Tree<Frequency>, u16)>>();
    let nodes = combine_nodes(frequencies);
    assert_eq!(nodes.len(), 1);
    let (combines_node, frequency) = &nodes[0];
    assert_eq!(combines_node.len(), 3);
    assert!(close_to(
        (*frequency as f64) / std::u16::MAX as f64,
        1.0,
        EPSILON
    ));
    //match combines_node {
    //    Tree::Node { left, right } => match left {
    //        Tree::Leaf(x) => assert!(),
    //        _ => panic!("supposed to be a leaf, not a node"),
    //    },
    //    _ => unreachable!("node should contain two leaves."),
    //}
}
