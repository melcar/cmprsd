use cmprsd::algorithm::huffman;

#[test]
pub fn huffman_empty_string() {
    let compressed = huffman::compress("");
}
