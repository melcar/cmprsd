pub struct BWT {
    pub transformed_string: String,
    pub index: u64,
}

impl BWT {
    pub fn transform(data: &str) -> BWT {
        let mut matrix: Vec<String> = (0..data.len()).fold(Vec::new(), |mut acc, i| {
            acc.push(data[i..].to_owned() + &data[..i]);
            acc
        });
        matrix.sort();
        BWT {
            transformed_string: matrix.iter().fold(String::new(), |mut acc, s| {
                acc.push(s.chars().last().unwrap());
                acc
            }),
            index: matrix.iter().position(|e| e == data).unwrap() as u64,
        }
    }
    pub fn inverse_tranform(bwt: &BWT) -> String {
        todo!("implement")
    }
}
