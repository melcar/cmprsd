pub struct Frequency {
    character: char,
    frequency: i32,
}

pub enum Huffman {
    Encoded {
        frequencies: Vec<Frequency>,
        encoded: String,
    },
    Decoded(String),
}

pub fn compress(data: &str) -> crate::huffman::Huffman {
    todo!("todo")
}
