use std::{
    fs::File,
    time::{Duration, Instant},
};
const NANOS_PER_SEC: u64 = 1_000_000_000;
use cmprsd::algorithm::huffman::Huffman;
use criterion::{
    black_box, criterion_group, criterion_main,
    measurement::{Measurement, ValueFormatter},
    Criterion, Throughput,
};
use std::io::Read;
struct MeaningfullValueFormater;
struct Regular;

impl MeaningfullValueFormater {
    fn get_formatter(ns: f64) -> impl FnMut(f64) -> String {
        let scaler = MeaningfullValueFormater::get_scaler(ns);
        move |ns: f64| -> String {
            format!(
                "{:>6} {}",
                scaler(&ns),
                MeaningfullValueFormater::get_unit(ns)
            )
        }
    }

    fn get_scaler(ns: f64) -> fn(&f64) -> f64 {
        if ns < 1.0 {
            // ns = time in nanoseconds per iteration
            |ns: &f64| -> f64 { ns * 1e3 }
        } else if ns < 10f64.powi(3) {
            |ns: &f64| -> f64 { *ns }
        } else if ns < 10f64.powi(6) {
            |ns: &f64| -> f64 { ns / 1e3 }
        } else if ns < 10f64.powi(9) {
            |ns: &f64| -> f64 { ns / 1e6 }
        } else {
            |ns: &f64| -> f64 { ns / 1e9 }
        }
    }
    fn get_unit(ns: f64) -> &'static str {
        if ns < 1.0 {
            // ns = time in nanoseconds per iteration
            "ps"
        } else if ns < 10f64.powi(3) {
            "ns"
        } else if ns < 10f64.powi(6) {
            "μs"
        } else if ns < 10f64.powi(9) {
            "ms"
        } else {
            "s"
        }
    }
}

impl Measurement for Regular {
    type Intermediate = Instant;

    type Value = Duration;

    fn start(&self) -> Self::Intermediate {
        Instant::now()
    }

    fn end(&self, i: Self::Intermediate) -> Self::Value {
        i.elapsed()
    }

    fn add(&self, v1: &Self::Value, v2: &Self::Value) -> Self::Value {
        *v1 + *v2
    }

    fn zero(&self) -> Self::Value {
        Duration::from_nanos(0)
    }

    fn to_f64(&self, value: &Self::Value) -> f64 {
        let nanos = value.as_secs() * NANOS_PER_SEC + u64::from(value.subsec_nanos());
        nanos as f64
    }

    fn formatter(&self) -> &dyn ValueFormatter {
        &MeaningfullValueFormater
    }
}

// https://bheisler.github.io/criterion.rs/book/user_guide/custom_measurements.html
impl ValueFormatter for MeaningfullValueFormater {
    fn format_value(&self, ns: f64) -> String {
        MeaningfullValueFormater::get_formatter(ns)(ns)
    }

    fn format_throughput(&self, throughput: &criterion::Throughput, value: f64) -> String {
        unimplemented!()
    }

    fn scale_values(&self, ns: f64, values: &mut [f64]) -> &'static str {
        let scaler = MeaningfullValueFormater::get_scaler(ns);
        let _ = values
            .iter_mut()
            .map(|val| scaler(val))
            .collect::<Vec<f64>>();
        MeaningfullValueFormater::get_unit(ns)
    }

    fn scale_throughputs(
        &self,
        _typical: f64,
        throughput: &Throughput,
        values: &mut [f64],
    ) -> &'static str {
        println!("2");
        unimplemented!()
    }

    fn scale_for_machines(&self, values: &mut [f64]) -> &'static str {
        println!("1");
        unimplemented!()
    }
}

fn better_measurement() -> Criterion<Regular> {
    Criterion::default().with_measurement(Regular)
}
const LOREM_IPSUM : &str ="Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

fn get_from_file(path: &str) -> std::io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn compressing_hello_world(c: &mut Criterion<Regular>) {
    let text = "Hello world!";
    let mut group = c.benchmark_group("100 samples");
    group.sample_size(100);
    group.bench_function("Compressing Hello worlds", |b| {
        b.iter(|| Huffman::compress(black_box(text)))
    });
}

fn compressing_lorem_ipsum(c: &mut Criterion<Regular>) {
    let text = LOREM_IPSUM;
    let mut group = c.benchmark_group("100 samples");
    group.sample_size(100);
    group.bench_function("Compressing Lorem Ipsum", |b| {
        b.iter(|| Huffman::compress(black_box(text)))
    });
}

fn compressing_japanese_author(c: &mut Criterion<Regular>) {
    let text = get_from_file("ressources/text/Du côté de chez Swann by Marcel Proust")
        .expect("file should be found");
    let mut group = c.benchmark_group("100 samples");
    group.sample_size(100);
    group.bench_function("Compressing japanese author", |b| {
        b.iter(|| Huffman::compress(black_box(&text)))
    });
}

fn compressing_proust(c: &mut Criterion<Regular>) {
    let text = get_from_file("ressources/text/Du côté de chez Swann by Marcel Proust")
        .expect("file should be found");
    let mut group = c.benchmark_group("100 samples");
    group.sample_size(100);
    group.bench_function("Compressing Proust", |b| {
        b.iter(|| Huffman::compress(black_box(&text)))
    });
}

fn decompressing_hello_world(c: &mut Criterion<Regular>) {
    let text = "Hello world!";
    let compressed_text = Huffman::compress(text).expect("");
    let mut group = c.benchmark_group("100 samples");
    group.sample_size(100);
    group.bench_function("Decompressing Hello worlds", |b| {
        b.iter(|| black_box(compressed_text.decompress()))
    });
}

fn decompressing_lorem_ipsum(c: &mut Criterion<Regular>) {
    let text = LOREM_IPSUM;
    let compressed_text = Huffman::compress(text).expect("");
    let mut group = c.benchmark_group("100 samples");
    group.sample_size(100);
    group.bench_function("Decompressing Lorem Ipsum", |b| {
        b.iter(|| black_box(compressed_text.decompress()))
    });
}

fn decompressing_japanese_author(c: &mut Criterion<Regular>) {
    let text = get_from_file("ressources/text/Du côté de chez Swann by Marcel Proust")
        .expect("file should be found");
    let compressed_text = Huffman::compress(&text).expect("");
    let mut group = c.benchmark_group("100 samples");
    group.sample_size(100);
    group.bench_function("Decompressing japanse author", |b| {
        b.iter(|| black_box(compressed_text.decompress()))
    });
}

fn decompressing_proust(c: &mut Criterion<Regular>) {
    let text = get_from_file("ressources/text/Du côté de chez Swann by Marcel Proust")
        .expect("file should be found");
    let compressed_text = Huffman::compress(&text).expect("");
    let mut group = c.benchmark_group("100 samples");
    group.sample_size(100);
    group.bench_function("Decompressing Proust", |b| {
        b.iter(|| black_box(compressed_text.decompress()))
    });
}

criterion_group! {
    name=benches;
    config= better_measurement();
    targets = compressing_hello_world,
    compressing_lorem_ipsum,
    compressing_japanese_author,
    compressing_proust,
    decompressing_hello_world,
    decompressing_lorem_ipsum,
    decompressing_japanese_author,
    decompressing_proust
}

criterion_main!(benches);
