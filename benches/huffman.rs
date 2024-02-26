use std::fs::File;

use cmprsd::algorithm::huffman::{self, compress, decompress};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::io::Read;

const LOREM_IPSUM : &str ="Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

fn get_from_file(path: &str) -> std::io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn compressing_hello_world(c: &mut Criterion) {
    let text = "Hello world!";
    let mut group = c.benchmark_group("100 samples");
    group.sample_size(100);
    group.bench_function("Hello worlds", |b| {
        b.iter(|| huffman::compress(black_box(text)))
    });
}

fn compressing_Lorem_Ipsum(c: &mut Criterion) {
    let text = LOREM_IPSUM;
    let mut group = c.benchmark_group("100 samples");
    group.sample_size(100);
    group.bench_function("Lorem Ipsum", |b| {
        b.iter(|| huffman::compress(black_box(text)))
    });
}

fn compressing_proust(c: &mut Criterion) {
    let text = get_from_file("ressources/text/Du côté de chez Swann by Marcel Proust")
        .expect("file should be found");
    let mut group = c.benchmark_group("10 samples");
    group.sample_size(10);
    group.bench_function("Proust", |b| b.iter(|| huffman::compress(black_box(&text))));
}

fn compressing_japanese_author(c: &mut Criterion) {
    let text = get_from_file("ressources/text/Du côté de chez Swann by Marcel Proust")
        .expect("file should be found");
    let mut group = c.benchmark_group("10 samples");
    group.sample_size(10);
    group.bench_function("Proust", |b| b.iter(|| huffman::compress(black_box(&text))));
}

fn decompressing_hello_world(c: &mut Criterion) {
    let text = "Hello world!";
    let compressed_text = huffman::compress(text);
    let mut group = c.benchmark_group("100 samples");
    group.sample_size(100);
    group.bench_function("Hello worlds", |b| {
        b.iter(|| huffman::decompress(black_box(&compressed_text)))
    });
}

fn decompressing_Lorem_Ipsum(c: &mut Criterion) {
    let text = LOREM_IPSUM;
    let compressed_text = huffman::compress(text);
    let mut group = c.benchmark_group("100 samples");
    group.sample_size(100);
    group.bench_function("Lorem Ipsum", |b| {
        b.iter(|| huffman::decompress(black_box(&compressed_text)))
    });
}

fn decompressing_proust(c: &mut Criterion) {
    let text = get_from_file("ressources/text/Du côté de chez Swann by Marcel Proust")
        .expect("file should be found");
    let compressed_text = huffman::compress(&text);
    let mut group = c.benchmark_group("10 samples");
    group.sample_size(10);
    group.bench_function("Proust", |b| {
        b.iter(|| huffman::decompress(black_box(&compressed_text)))
    });
}

fn decompressing_japanese_author(c: &mut Criterion) {
    let text = get_from_file("ressources/text/Du côté de chez Swann by Marcel Proust")
        .expect("file should be found");
    let compressed_text = huffman::compress(&text);
    let mut group = c.benchmark_group("10 samples");
    group.sample_size(10);
    group.bench_function("Proust", |b| {
        b.iter(|| huffman::decompress(black_box(&compressed_text)))
    });
}

criterion_group!(
    benches,
    compressing_hello_world,
    compressing_Lorem_Ipsum,
    compressing_japanese_author,
    compressing_proust,
    decompressing_hello_world,
    decompressing_Lorem_Ipsum,
    decompressing_japanese_author,
    decompressing_proust
);

criterion_main!(benches);
