use criterion::{Criterion, criterion_group, criterion_main};
use huffman::encode_decode_from_string;
use std::hint::black_box;

fn criterion_benchmark(c: &mut Criterion) {
    let input = "The quick brown fox jumps over the lazy dog. ".repeat(1000);

    c.bench_function("encode_decode_from_string", |b| {
        b.iter(|| {
            let s = black_box(input.clone());
            let _res = encode_decode_from_string(s);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
