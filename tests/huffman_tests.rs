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
#[ignore = "not implemented yet"]
pub fn huffman_empty_string() {
    let compressed = huffman::compress("");
}

#[test]
#[ignore = "not implemented yet"]
pub fn huffman_repeating_string() {
    let compressed = huffman::compress("");
}

#[test]
#[ignore = "not implemented yet"]
pub fn huffman_two_characters() {
    let compressed = huffman::compress("");
}

#[test]
#[ignore = "not implemented yet"]
pub fn huffman_random() {
    let compressed = huffman::compress("");
}

#[test]
#[ignore = "not implemented yet"]
pub fn huffman_lorem() {
    let compressed = huffman::compress(LOREM_IPSUM);
}

#[test]
#[ignore = "not implemented yet"]
pub fn compute_frequencies_empty_string() {
    assert!(huffman::compute_frequencies("").is_empty());
}

#[test]
pub fn compute_frequencies_one_character() {
    assert_eq!(
        huffman::compute_frequencies("a")[0],
        huffman::Frequency {
            character: Some('a'),
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
            character: Some('a'),
            frequency: std::u16::MAX / 2
        },
    );
    assert_eq!(
        frequencies[1],
        huffman::Frequency {
            character: Some('b'),
            frequency: std::u16::MAX / 2
        },
    );
}

#[test]
pub fn compute_frequencies_two_character_25_75() {
    let frequencies = huffman::compute_frequencies("bbab");
    assert_eq!(frequencies[0].character, Some('a'));
    assert!(close_to(frequencies[0].get_frequency(), 0.25, EPSILON));

    assert_eq!(frequencies[1].character, Some('b'));
    assert!(close_to(frequencies[1].get_frequency(), 0.75, EPSILON));
}

fn check_frequencies(frequencies: Vec<Frequency>, string: &str) {
    for frequency in frequencies.iter() {
        assert!(close_to(
            frequency.get_frequency(),
            string
                .chars()
                .filter(|&c| c == frequency.character.expect("should not be None"))
                .count() as f64
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

fn check_leaf(node: &Tree<Frequency>, expected_char: char, expected_frequency: f64) {
    match node {
        Tree::Leaf(content) => {
            assert_eq!(content.character, Some(expected_char));
            assert!(close_to(
                content.get_frequency(),
                expected_frequency,
                EPSILON
            ))
        }
        _ => unreachable!(),
    }
}

fn check_internal_node(
    node: &Tree<Frequency>,
    expected_content: Frequency,
    expected_left_node: Tree<Frequency>,
    expected_right_node: Tree<Frequency>,
) {
    match node {
        Tree::Node {
            content,
            left,
            right,
        } => {
            assert_eq!(*content, expected_content);
            assert_eq!(*left.as_ref(), expected_left_node);
            assert_eq!(*right.as_ref(), expected_right_node);
        }
        _ => unreachable!(),
    }
}

#[test]
pub fn combine_2_nodes_50_50_alphabetical_order() {
    let frequencies = huffman::compute_frequencies("ab")
        .into_iter()
        .map(Tree::Leaf)
        .collect();

    let nodes = combine_nodes(frequencies);
    assert_eq!(nodes.len(), 1);
    let combines_node = &nodes[0];
    assert_eq!(combines_node.len(), 3);
    assert!(close_to(combines_node.get_frequency(), 1.0, EPSILON));
    check_internal_node(
        combines_node,
        Frequency::build_frequency(std::u16::MAX - 1, None),
        Tree::Leaf(Frequency::build_frequency(std::u16::MAX / 2, Some('b'))),
        Tree::Leaf(Frequency::build_frequency(std::u16::MAX / 2, Some('a'))),
    );
}

#[test]
pub fn combine_2_nodes_75_25() {
    let frequencies = huffman::compute_frequencies("abbb")
        .into_iter()
        .map(Tree::Leaf)
        .collect();

    let nodes = combine_nodes(frequencies);
    assert_eq!(nodes.len(), 1);
    let combines_node = &nodes[0];
    assert_eq!(combines_node.len(), 3);
    assert!(close_to(combines_node.get_frequency(), 1.0, EPSILON));

    match combines_node {
        Tree::Node {
            content,
            left,
            right,
        } => {
            check_leaf(left, 'b', 0.75);
            check_leaf(right, 'a', 0.25);
        }
        _ => unreachable!("should be internal node. Not a leaf."),
    }
}
