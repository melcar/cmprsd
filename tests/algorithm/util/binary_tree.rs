use crate::algorithm::util::binary_tree::huffman::{build_huffman_tree, compute_frequencies};
use cmprsd::huffman;
use cmprsd::huffman::combine_nodes;
use cmprsd::util::{
    binary_tree::{
        Direction::{Left, Right},
        Tree,
    },
    frequency::Frequency,
};
use std::cmp::Reverse;
use std::collections::HashMap;

fn check_leaf(node: &Tree<Frequency>, expected_char: char, expected_frequency: usize) {
    match node {
        Tree::Leaf(content) => {
            assert_eq!(content.character, Some(expected_char));
            assert_eq!(content.count, expected_frequency,)
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

fn compare<T: std::cmp::Ord>(result: Vec<T>, expected_result: Vec<T>) {
    assert_eq!(result.len(), expected_result.len());
    result
        .iter()
        .zip(expected_result.iter())
        .all(|(result, expected)| result == expected);
}

#[test]
pub fn binary_tree_bfs_1_node() {
    let tree = Tree::<u16>::Leaf(0);
    assert_eq!(tree.height(), 1);
    let bfs = tree.to_breadth_first_search();
    let expected_bfs: Vec<u16> = vec![1];
    compare(bfs, expected_bfs)
}

#[test]
pub fn binary_tree_bfs_3_nodes() {
    let left_node = Tree::<u16>::Leaf(1);
    let right_node = Tree::<u16>::Leaf(2);
    let tree = Tree::<u16>::Node {
        content: 0,
        left: Box::new(left_node),
        right: Box::new(right_node),
    };
    assert_eq!(tree.height(), 2);
    let bfs = tree.to_breadth_first_search();
    let expected_bfs: Vec<u16> = vec![0, 1, 2];
    compare(bfs, expected_bfs)
}

#[test]
pub fn binary_tree_bfs_4_nodes() {
    let l1_left_node = Tree::<u16>::Leaf(3);
    let l1_right_node = Tree::<u16>::Leaf(4);
    let l0_left_node = Tree::<u16>::Node {
        content: 1,
        left: Box::new(l1_left_node),
        right: Box::new(l1_right_node),
    };
    let l0_right_node = Tree::<u16>::Leaf(2);
    let tree = Tree::<u16>::Node {
        content: 0,
        left: Box::new(l0_left_node),
        right: Box::new(l0_right_node),
    };
    assert_eq!(tree.height(), 3);
    let bfs = tree.to_breadth_first_search();
    let expected_bfs: Vec<u16> = vec![0, 1, 2, 3, 4];
    compare(bfs, expected_bfs)
}

#[test]
pub fn binary_tree_bfs_4_nodes_symmetric() {
    let l1_left_node = Tree::<u16>::Leaf(3);
    let l1_right_node = Tree::<u16>::Leaf(4);
    let l0_right_node = Tree::<u16>::Node {
        content: 1,
        left: Box::new(l1_left_node),
        right: Box::new(l1_right_node),
    };
    let l0_left_node = Tree::<u16>::Leaf(2);
    let tree = Tree::<u16>::Node {
        content: 0,
        left: Box::new(l0_left_node),
        right: Box::new(l0_right_node),
    };
    assert_eq!(tree.height(), 3);
    let bfs = tree.to_breadth_first_search();
    let expected_bfs: Vec<u16> = vec![0, 2, 1, 3, 4];
    compare(bfs, expected_bfs)
}

#[test]
pub fn binary_tree_directions_1_node() {
    let tree = Tree::<u16>::Leaf(0);
    assert_eq!(tree.height(), 1);
    let directions = tree.leaf_paths();
    let expected_directions = vec![(vec![], 1)];
    assert_eq!(directions.len(), expected_directions.len());
    compare(directions, expected_directions)
}

#[test]
pub fn binary_tree_directions_3_nodes() {
    let left_node = Tree::<u16>::Leaf(1);
    let right_node = Tree::<u16>::Leaf(2);
    let tree = Tree::<u16>::Node {
        content: 0,
        left: Box::new(left_node),
        right: Box::new(right_node),
    };
    assert_eq!(tree.height(), 2);
    let directions = tree.leaf_paths();
    let expected_directions = vec![(vec![Left], 1), (vec![Right], 2)];
    assert_eq!(directions.len(), expected_directions.len());
    compare(directions, expected_directions)
}

#[test]
pub fn binary_tree_directions_4_nodes() {
    let l1_left_node = Tree::<u16>::Leaf(3);
    let l1_right_node = Tree::<u16>::Leaf(4);
    let l0_left_node = Tree::<u16>::Node {
        content: 1,
        left: Box::new(l1_left_node),
        right: Box::new(l1_right_node),
    };
    let l0_right_node = Tree::<u16>::Leaf(2);
    let tree = Tree::<u16>::Node {
        content: 0,
        left: Box::new(l0_left_node),
        right: Box::new(l0_right_node),
    };
    assert_eq!(tree.height(), 3);
    let directions = tree.leaf_paths();
    let expected_directions = vec![
        (vec![Right], 2),
        (vec![Left, Left], 3),
        (vec![Left, Right], 4),
    ];
    assert_eq!(directions.len(), expected_directions.len());
    compare(directions, expected_directions)
}

#[test]
pub fn binary_tree_directions_4_nodes_symmetric() {
    let l1_left_node = Tree::<u16>::Leaf(3);
    let l1_right_node = Tree::<u16>::Leaf(4);
    let l0_right_node = Tree::<u16>::Node {
        content: 1,
        left: Box::new(l1_left_node),
        right: Box::new(l1_right_node),
    };
    let l0_left_node = Tree::<u16>::Leaf(2);
    let tree = Tree::<u16>::Node {
        content: 0,
        left: Box::new(l0_left_node),
        right: Box::new(l0_right_node),
    };
    assert_eq!(tree.height(), 3);
    let directions = tree.leaf_paths();
    let expected_directions = vec![
        (vec![Left], 2),
        (vec![Right, Right], 4),
        (vec![Right, Left], 3),
    ];
    assert_eq!(directions.len(), expected_directions.len());
    compare(directions, expected_directions)
}

#[test]
pub fn combine_2_nodes_50_50_alphabetical_order() {
    let frequencies = huffman::compute_frequencies("ab")
        .into_iter()
        .map(Tree::Leaf)
        .map(Reverse)
        .collect();

    let mut nodes = combine_nodes(frequencies);
    assert_eq!(nodes.len(), 1);
    let combines_node = nodes.pop().unwrap().0;
    assert_eq!(combines_node.len(), 3);
    assert_eq!(combines_node.get_count(), 2);
    check_internal_node(
        &combines_node,
        Frequency::build_frequency(2, None),
        Tree::Leaf(Frequency::build_frequency(1, Some('a'))),
        Tree::Leaf(Frequency::build_frequency(1, Some('b'))),
    );
}

#[test]
pub fn combine_2_nodes_75_25() {
    let frequencies = huffman::compute_frequencies("abbb")
        .into_iter()
        .map(Tree::Leaf)
        .map(Reverse)
        .collect();

    let mut nodes = combine_nodes(frequencies);
    assert_eq!(nodes.len(), 1);
    let combines_node = nodes.pop().unwrap().0;
    assert_eq!(combines_node.len(), 3);
    assert_eq!(combines_node.get_count(), 4);

    match combines_node {
        Tree::Node {
            content: _,
            left,
            right,
        } => {
            check_leaf(&left, 'a', 1);
            check_leaf(&right, 'b', 3);
        }
        _ => unreachable!("should be internal node. Not a leaf."),
    }
}

#[test]
pub fn combine_5_nodes() {
    let input = "aaaaabbbbcccdde";
    let frequencies = compute_frequencies(input);
    assert_eq!(frequencies.len(), 5);

    let tree = build_huffman_tree(&frequencies);
    assert_eq!(tree.len(), 9);
    assert_eq!(tree.height(), 4);
    let bfs = tree.to_breadth_first_search();
    assert_eq!(bfs.len(), tree.len());
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

    let expected_bfs: Vec<(Option<char>, usize)> = vec![
        (None, 15),
        (None, 6),
        (None, 9),
        (None, 3),
        (Some('c'), 3),
        (Some('b'), 4),
        (Some('a'), 5),
        (Some('e'), 1),
        (Some('d'), 2),
    ];

    bfs.iter()
        .zip(expected_bfs.iter())
        .for_each(|(node, (expected_char, expected_frequency))| {
            assert_eq!(node.character, *expected_char);
            assert_eq!(node.count, *expected_frequency);
        })
}
