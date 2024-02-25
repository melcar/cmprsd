use cmprsd::algorithm::util::binary_tree::{
    Direction::{Left, Right},
    Tree,
};

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
