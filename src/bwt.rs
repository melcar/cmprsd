use crate::encoder::EncoderDecoder;

pub struct BWT {
    //L in the paper
    pub transformed_string: String,
    //I in the paper
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

    fn get_k_to_i(k: usize, l: &str, ch: &char) -> usize {
        l.chars()
            .enumerate()
            .filter(|(_, c)| c == ch)
            .enumerate()
            .find(|(id, _)| *id == k)
            .unwrap()
            .1
             .0
    }

    fn get_k_for_j(j: usize, f: &str, ch: &char) -> usize {
        f.chars()
            .enumerate()
            .filter(|(_, c)| c == ch)
            .position(|(id, _)| id == j)
            .unwrap()
    }

    fn get_t(j: usize, f: &str, l: &str, ch: &char) -> usize {
        let k = Self::get_k_for_j(j, f, ch);
        Self::get_k_to_i(k, l, ch)
    }

    fn build_back_s(l: &str, t: &[usize], i: usize) -> String {
        let mut previous = i;
        let mut l_as_idx: Vec<usize> = vec![i; l.len()]
            .iter()
            .rev()
            .skip(1)
            .map(|_| {
                previous = t[previous];
                previous
            })
            .rev()
            .collect();
        l_as_idx.push(i);

        l_as_idx
            .into_iter()
            .map(|idx| l.chars().nth(idx).unwrap())
            .collect()
    }

    pub fn inverse_tranform(&self) -> String {
        //First Step we Get F.
        let mut f: Vec<char> = self.transformed_string.chars().collect();
        f.sort_unstable();
        let f_as_string: String = f.iter().clone().collect();
        //Second Step We Build T
        let t: Vec<usize> = f
            .iter()
            .enumerate()
            .map(|(j, ch)| Self::get_t(j, &f_as_string, &self.transformed_string, ch))
            .collect();
        // Thirds Step we build back S from T
        Self::build_back_s(&self.transformed_string, &t, self.index as usize)
    }
}

impl EncoderDecoder for BWT {
    fn encode(data: &str) -> Self {
        BWT::transform(data)
    }
    fn decode(&self) -> String {
        self.inverse_tranform()
    }
}
