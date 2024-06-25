use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ltc::encoder::{Encoder, EncoderType};

// fn big_encode_benchmark(c: &mut Criterion) {
//     let file =
//         std::fs::File::open("../../test_big_file.txt").expect("Failed to open the input file.");
//     let mut output_file =
//         std::fs::File::create("../../test_big_file.hm").expect("Failed to create the output file.");
//     let mut base = Encoder::new(file, output_file, EncoderType::Lossy);
//     c.bench_function("big encoding", |b| b.iter(|| base.encode().unwrap()));
// }

fn medium_encode_benchmark(c: &mut Criterion) {
    let file = std::fs::File::open("encode.txt").expect("Failed to open the input file.");
    let mut output_file =
        std::fs::File::create("./decode.hm").expect("Failed to create the output file.");
    let mut base = Encoder::new(file, output_file, EncoderType::Lossy);
    c.bench_function("medium encoding", |b| b.iter(|| base.encode().unwrap()));
}

fn small_encode_benchmark(c: &mut Criterion) {
    let file = std::fs::File::open("basetest_encode.txt").expect("Failed to open the input file.");
    let mut output_file =
        std::fs::File::create("./basetest_decode.hm").expect("Failed to create the output file.");
    let mut base = Encoder::new(file, output_file, EncoderType::Lossy);
    c.bench_function("small encoding", |b| b.iter(|| base.encode().unwrap()));
}

criterion_group!(
    benches,
    medium_encode_benchmark,
    small_encode_benchmark,
    big_encode_benchmark
);
criterion_main!(benches);
