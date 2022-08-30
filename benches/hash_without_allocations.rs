use criterion::{criterion_group, criterion_main, Criterion};
use experiment::{gen_keys, LEN, KEY_LEN, prepare_hash_map, ShortKey, Key};

fn criterion_benchmark(c: &mut Criterion) {
    let keys = gen_keys(LEN, KEY_LEN);
    let map = prepare_hash_map(&keys);

    c.bench_function("hash_without_allocations", |b| b.iter(
        || {
            let mut sum = 0;
            for key in &keys {
                sum += map[&ShortKey(&key) as &dyn Key];
            }
            assert_eq!(keys.len(), sum);
        }
    ));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);