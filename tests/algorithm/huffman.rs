use cmprsd::util::frequency::Frequency;
use proptest::prelude::*;
use std::fs::File;
use std::io::Read;

use cmprsd::huffman::{self, compute_frequencies, CompressionError, Huffman};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

const LOREM_IPSUM : &str ="Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

fn test_compression_decompression(data: &str) {
    match Huffman::compress(data) {
        Err(CompressionError::NoDataToCompress) => assert!(data.is_empty()),
        Err(CompressionError::DataCannotBeCompressed) => {
            assert!(!data.chars().any(|c| c != data.chars().last().unwrap()))
        }
        Ok(compressed_data) => {
            let decompressed_data = compressed_data.decompress();
            assert_eq!(
                data.len(),
                decompressed_data.len(),
                " {}\n and \n{}\n have different len : {}!={}",
                data.len(),
                decompressed_data.len(),
                data,
                decompressed_data
            );
            assert_eq!(
                data, decompressed_data,
                "expected \n{}\n but got\n{}",
                data, decompressed_data
            );
        }
    }
}
proptest! {
    #[test]
    fn random_tests(s in "\\PC*") {
    test_compression_decompression(&s);
    }
}

fn test_from_file(path: &str) -> std::io::Result<()> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    test_compression_decompression(&contents);
    Ok(())
}

fn get_random_string(range: std::ops::Range<usize>) -> String {
    let mut rng = thread_rng();
    let string_length: usize = rng.gen_range(range);

    (&mut rng)
        .sample_iter(Alphanumeric)
        .take(string_length)
        .map(char::from)
        .collect()
}

#[test]
pub fn huffman_empty_string() {
    test_compression_decompression("")
}

#[test]
pub fn huffman_one_character() {
    test_compression_decompression("a")
}

#[test]
pub fn huffman_twice_same_character() {
    test_compression_decompression("aa")
}

#[test]
pub fn huffman_repeating_string() {
    test_compression_decompression("ababababababab");
}

#[test]
pub fn huffman_two_characters() {
    test_compression_decompression("ab")
}

#[test]
pub fn huffman_random() {
    let mut start = 120;
    let mut end = 130; //(start as f32 * 1.1) as usize;
    (1..10_000).for_each(|i| {
        if i % 1000 == 0 {
            start += 10;
            end = (start as f32 * 1.1) as usize;
        }
        let random_string = get_random_string(start..end);
        test_compression_decompression(&random_string)
    })
}

#[test]
pub fn huffman_hello_worlds() {
    test_compression_decompression("On offering to help the blind man, the man who then stole his car, had not, at that precise moment, had any evil intention, quite the contrary, w");
}

#[test]
pub fn huffman_hello_world() {
    test_compression_decompression("Hello world!");
}

#[test]
pub fn huffman_lorem() {
    test_compression_decompression(&LOREM_IPSUM[1..LOREM_IPSUM.len() / 3 - 6]);
}

#[test]
pub fn huffman_japanese_book() {
    let path = "ressources/text/あめりか物語 by Kafu Nagai";
    test_from_file(path).err();
}

#[test]
pub fn huffman_proust() {
    let path = "ressources/text/Du côté de chez Swann by Marcel Proust";
    test_from_file(path).err();
}

#[test]
pub fn compute_frequencies_empty_string() {
    assert!(huffman::compute_frequencies("").is_empty());
}

#[test]
pub fn compute_frequencies_one_character() {
    assert_eq!(
        huffman::compute_frequencies("a")[0],
        Frequency {
            character: Some('a'),
            count: 1
        }
    )
}

#[test]
pub fn compute_frequencies_two_character_50_50() {
    let frequencies = huffman::compute_frequencies("abab");
    assert_eq!(
        frequencies[0],
        Frequency {
            character: Some('a'),
            count: 2
        },
    );
    assert_eq!(
        frequencies[1],
        Frequency {
            character: Some('b'),
            count: 2
        },
    );
}

#[test]
pub fn compute_frequencies_two_character_25_75() {
    let frequencies = huffman::compute_frequencies("bbab");
    assert_eq!(frequencies[0].character, Some('a'));
    assert_eq!(frequencies[0].count, 1);

    assert_eq!(frequencies[1].character, Some('b'));
    assert_eq!(frequencies[1].count, 3);
}

fn check_frequencies(frequencies: &[Frequency], string: &str) {
    for frequency in frequencies.iter() {
        assert_eq!(
            frequency.count,
            string
                .chars()
                .filter(|&c| c == frequency.character.expect("should not be None"))
                .count()
        );
    }
}

#[test]
pub fn compute_frequencies_repeating_string() {
    let data = "ababababababab";
    let frequencies = compute_frequencies(data);
    check_frequencies(&frequencies, data)
}

#[test]
pub fn compute_frequencies_lorem_ipsum() {
    check_frequencies(&huffman::compute_frequencies(LOREM_IPSUM), LOREM_IPSUM);
}

#[test]
pub fn test() {
    let string ="Z8LLhYT7I5LfSQ6xOXaksWkfFWGQcsqOlJmLJtnjqbWPJEBm3FoAmf3LYNp2mmSsaXLAagsRCcl4mVtzqC0gWCNDliQBotL7EGShb4RtigBxm6CopGJnFEZFZgl567FcL8XpxxyY6wwlASqhsXuyN3xLMZJwPuOhgBCc3Ah3qyZ7t4jl9MHvoIUXHcHLZRbc6";
    test_compression_decompression(string)
}

#[test]
pub fn compute_frequencies_random_long_string() {
    let random_string = get_random_string(10_000..15_000);
    check_frequencies(
        &huffman::compute_frequencies(&random_string),
        &random_string,
    )
}
