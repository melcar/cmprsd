use std::collections::{BTreeMap, HashMap};

use cmprsd::algorithm::huffman::{
    self, build_huffman_tree, combine_nodes, compute_frequencies, Frequency, Huffman,
};
use cmprsd::algorithm::util::binary_tree::Tree;
use rand::{distributions::Alphanumeric, thread_rng, Rng};

const LOREM_IPSUM : &str ="Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

const EPSILON: f64 = 0.0001_f64;
//make a macro out of it
fn close_to(a: f64, b: f64, delta: f64) -> bool {
    (a - b).abs() < delta
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
        Tree::Leaf(Frequency::build_frequency(std::u16::MAX / 2, Some('a'))),
        Tree::Leaf(Frequency::build_frequency(std::u16::MAX / 2, Some('b'))),
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
            check_leaf(left, 'a', 0.25);
            check_leaf(right, 'b', 0.75);
        }
        _ => unreachable!("should be internal node. Not a leaf."),
    }
}

#[test]
pub fn combine_5_nodes() {
    let input = "aaaaabbbbcccdde";
    let frequencies = compute_frequencies(input);
    assert_eq!(frequencies.len(), 5);

    let tree = build_huffman_tree(frequencies);
    assert_eq!(tree.len(), 9);
    let bfs = tree.to_breadth_first_search();
    assert_eq!(bfs.len(), 9);
    let mut expected_frequencies: HashMap<char, f64> =
        input.chars().map(|c| (c, 0_f64)).collect::<HashMap<_, _>>();

    expected_frequencies
        .clone()
        .keys()
        .copied()
        .for_each(|character| {
            expected_frequencies.insert(
                character,
                (input.chars().filter(|c| *c == character).count() as f64) / (input.len() as f64),
            );
        });

    let expected_bfs: Vec<(Option<char>, f64)> = vec![
        (None, 1.0),             // level 0
        (None, 0.6666666666666), // left level 1
        (Some('a'), 0.33333333), // right level 1
        (None, 0.4),             // left level 2
        (Some('b'), 0.26666),    // right level 2
        (None, 0.2),         //left level 2
        (Some('c'), 0.2),        // right level 2
        (Some('e'), 0.066666667),     // left level 3
        (Some('d'), 0.13333333),     // right level 3
    ];

    bfs.iter()
        .zip(expected_bfs.iter())
        .for_each(|(node, (expected_char, expected_frequency))| {
            println!("{}, {}", node.get_frequency(), expected_frequency);
            assert_eq!(node.character, *expected_char);
            assert!(close_to(node.get_frequency(), *expected_frequency, EPSILON))
        })
}

#[test]
pub fn frequency_comparison_different_frequencies_same_letter() {
    let f1 = Frequency::build_frequency(51, Some('a'));
    let f2 = Frequency::build_frequency(56, Some('a'));
    assert!(f1 < f2)
}

#[test]
pub fn frequency_comparison_same_letters_different_frequencies() {
    let f1 = Frequency::build_frequency(56, Some('a'));
    let f2 = Frequency::build_frequency(56, Some('b'));
    assert!(f1 < f2)
}

#[test]
pub fn frequency_comparison_one_letter_one_none_different_frequencies1() {
    let f1 = Frequency::build_frequency(54, None);
    let f2 = Frequency::build_frequency(56, Some('b'));
    assert!(f1 < f2)
}

#[test]
pub fn frequency_comparison_one_letter_one_none_different_frequencies2() {
    let f1 = Frequency::build_frequency(57, None);
    let f2 = Frequency::build_frequency(56, Some('b'));
    assert!(f1 > f2)
}

#[test]
pub fn frequency_comparison_one_letter_one_none_same_frequencies() {
    let f1 = Frequency::build_frequency(56, None);
    let f2 = Frequency::build_frequency(56, Some('b'));
    assert!(f1 < f2)
}

#[test]
pub fn frequency_comparison_both_none_different_frequencies() {
    let f1 = Frequency::build_frequency(53, None);
    let f2 = Frequency::build_frequency(56, None);
    assert!(f1 < f2)
}
