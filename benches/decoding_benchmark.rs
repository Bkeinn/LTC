use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ltc::decoder::{Decoder, EncoderType};

// fn big_benchmark(c: &mut Criterion) {
//     let file = std::fs::File::open("../../test_big_file.hm").expect("could not open file : ( ");
//     let mut output_file =
//         std::fs::File::create("../../test_big_file_normal.txt").expect("Could not create file");
//     let mut group = c.benchmark_group("big");

//     group
//         .sample_size(10)
//         .warm_up_time(std::time::Duration::from_secs(1)) // Set your warm-up time here
//         .measurement_time(std::time::Duration::from_secs(5)); // Set your measurement time here
//                                                               // .throughput(criterion::Throughput::Bytes(1000)); // Adjust the throughput value as needed

//     let mut base = Decoder::new(file, output_file, EncoderType::Lossy);
//     group.bench_function("big decoding", |b| b.iter(|| base.decode().unwrap()));
//     group.finish();
// }

fn decode_benchmark(c: &mut Criterion) {
    let file = std::fs::File::open("./test_decoder.hm").expect("could not open file : ( ");
    let mut output_file =
        std::fs::File::create("./test_decoder.txt").expect("Could not create file");
    let mut base = Decoder::new(file, output_file, EncoderType::Lossy);
    c.bench_function("medium decoding", |b| b.iter(|| base.decode().unwrap()));
}

criterion_group!(benches, decode_benchmark, big_benchmark);
criterion_main!(benches);
