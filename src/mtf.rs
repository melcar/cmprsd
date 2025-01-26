use crate::encoder::EncoderDecoder;
use itertools::Itertools;

/// Move To Font Transform algorith
/// This algorithm can be used as a preprocing step begore applying a compression algorithm. Ex
/// huffman
pub struct MTF {
    alphabet: Vec<char>,
    indices: Vec<usize>,
}

impl MTF {
    pub fn transform(data: &str) -> Self {
        let alphabet: Vec<char> = data.chars().unique().collect();
        let mut running_alphabet = alphabet.clone();
        let indices = data
            .chars()
            .map(|c| {
                let idx = running_alphabet.iter().position(|cc| c == *cc).unwrap();
                running_alphabet.remove(idx);
                running_alphabet.insert(0, c);
                idx
            })
            .collect();
        MTF { alphabet, indices }
    }

    pub fn inverse_transform(&self) -> String {
        todo!()
    }
}

impl EncoderDecoder for MTF {
    fn encode(data: &str) -> Self {
        MTF::transform(data)
    }

    fn decode(&self) -> String {
        self.inverse_transform()
    }
}
