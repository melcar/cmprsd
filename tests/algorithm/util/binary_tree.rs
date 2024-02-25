use cmprsd::algorithm::util::binary_tree::Tree;

fn compare_bfs<T: std::cmp::Ord>(bfs: Vec<T>, expected_bfs: Vec<T>) {
    assert_eq!(bfs.len(), expected_bfs.len());
    bfs.iter()
        .zip(expected_bfs.iter())
        .all(|(result, expected)| result == expected);
}

#[test]
pub fn binary_tree_bfs_1_node() {
    let tree = Tree::<u16>::Leaf(0);
    assert_eq!(tree.height(), 1);
    let bfs = tree.to_breadth_first_search();
    let expected_bfs: Vec<u16> = vec![1];
    compare_bfs(bfs, expected_bfs)
}

#[test]
pub fn binary_tree_bfs_3_node() {
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
    compare_bfs(bfs, expected_bfs)
}

#[test]
pub fn binary_tree_bfs_4_node() {
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
    compare_bfs(bfs, expected_bfs)
}

#[test]
pub fn binary_tree_bfs_4_node_symmetric() {
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
    compare_bfs(bfs, expected_bfs)
}
