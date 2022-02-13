
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use polygon::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../examples/test.ply");
    let mut lexer = lexer::Lexer::new(input);
    c.bench_function("lexer", |b| b.iter(||{
        lexer.next_token();
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);