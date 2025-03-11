use craftflow_nbt::{Nbt, NbtValue};
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};

mod perf;
#[path = "../shared.rs"]
mod shared;

fn roundtrip(c: &mut Criterion) {
    let sizes = [10, 100, 1000];

    let mut group = c.benchmark_group("NBT dynamic");

    for &size in &sizes {
        let nbt = shared::gen_random_dyn_nbt(size);
        let mut buffer = Vec::new();
        nbt.nbt_write(&mut buffer);
        let n_bytes = buffer.len();

        group.bench_with_input(
            BenchmarkId::new("serialize", n_bytes),
            &nbt,
            |b, nbt| {
                let mut buf = Vec::with_capacity(n_bytes);
                b.iter_with_large_drop(|| {
                    buf.clear();
                    black_box(nbt).nbt_write(&mut buf);
                    black_box(&buf);
                })

            },
        );

        group.bench_with_input(
            BenchmarkId::new("deserialize", n_bytes),
            &buffer,
            |b, buffer| {
                b.iter_with_large_drop(|| {
                    NbtValue::nbt_read(&mut black_box(buffer)).unwrap()
                })
            },
        );
    }

    group.finish();
}

criterion_group!{
    name = benches;
    config = Criterion::default().with_profiler(perf::FlamegraphProfiler::new(100));
    targets = roundtrip
}
criterion_main!(benches);
