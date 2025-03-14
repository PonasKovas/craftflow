use craftflow_nbt::{Nbt, NbtString, NbtValue};
use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use rand::{Rng, SeedableRng, rngs::StdRng};
use shared::gen_random_string;

mod perf;
#[path = "../shared.rs"]
mod shared;

fn roundtrip_dynamic(c: &mut Criterion) {
	let mut group = c.benchmark_group("dynamic");

	for size in [10, 100, 1000] {
		let nbt = shared::gen_random_dyn_nbt(size);
		let mut buffer = Vec::new();
		nbt.nbt_write(&mut buffer);
		let n_bytes = buffer.len();

		group.throughput(Throughput::BytesDecimal(n_bytes as u64));

		group.bench_function(BenchmarkId::new("serialize", n_bytes), |b| {
			let mut buf = Vec::with_capacity(n_bytes);
			b.iter(|| {
				buf.clear();
				black_box(&nbt).nbt_write(&mut buf);
				black_box(&buf);
			})
		});

		group.bench_function(BenchmarkId::new("deserialize", n_bytes), |b| {
			b.iter_with_large_drop(|| {
				let mut s = black_box(&buffer[..]);
				let r = NbtValue::nbt_read(&mut s).unwrap();
				assert!(s.is_empty());
				r
			})
		});
	}

	group.finish();
}

fn roundtrip_structured(c: &mut Criterion) {
	#[derive(Nbt)]
	struct A {
		a: NbtString,
		b: Vec<B>,
	}
	#[derive(Nbt)]
	struct B {
		a: f64,
		b: f64,
		c: f64,
		d: f64,
		e: f64,
		f: f64,
		g: f64,
		h: f64,
		i: C,
	}
	#[derive(Nbt)]
	struct C {
		a: f64,
		b: f64,
		c: f64,
		d: f64,
		e: f64,
		f: f64,
		g: f64,
		h: f64,
	}
	let mut group = c.benchmark_group("structured");
	let mut rng = StdRng::seed_from_u64(0);
	for size in [1000, 100000, 1000000] {
		let nbt = A {
			a: gen_random_string(&mut rng, 512),
			b: (0..size)
				.into_iter()
				.map(|_| B {
					a: rng.random(),
					b: rng.random(),
					c: rng.random(),
					d: rng.random(),
					e: rng.random(),
					f: rng.random(),
					g: rng.random(),
					h: rng.random(),
					i: C {
						a: rng.random(),
						b: rng.random(),
						c: rng.random(),
						d: rng.random(),
						e: rng.random(),
						f: rng.random(),
						g: rng.random(),
						h: rng.random(),
					},
				})
				.collect(),
		};
		let mut buffer = Vec::new();
		nbt.nbt_write(&mut buffer);
		let n_bytes = buffer.len();

		group.throughput(Throughput::BytesDecimal(n_bytes as u64));

		group.bench_function(BenchmarkId::new("serialize", n_bytes), |b| {
			let mut buf = Vec::with_capacity(n_bytes);
			b.iter(|| {
				buf.clear();
				black_box(&nbt).nbt_write(&mut buf);
				black_box(&buf);
			})
		});

		group.bench_function(BenchmarkId::new("deserialize", n_bytes), |b| {
			b.iter_with_large_drop(|| {
				let mut s = black_box(&buffer[..]);
				let r = A::nbt_read(&mut s).unwrap();
				assert!(s.is_empty());
				r
			})
		});
	}

	group.finish();
}

fn complex_player(c: &mut Criterion) {
	let input = include_bytes!("../complex_player.nbt");

	let mut group = c.benchmark_group("complex_player");
	group.throughput(Throughput::Bytes(input.len() as u64));

	group.bench_function("deserialize", |b| {
		b.iter_with_large_drop(|| {
			let mut s = black_box(&input[..]);
			let r = NbtValue::nbt_read_named(&mut s).unwrap();
			assert!(s.is_empty());
			r
		})
	});
}

criterion_group! {
	name = benches;
	config = Criterion::default().with_profiler(perf::FlamegraphProfiler::new(100));
	targets = roundtrip_dynamic, roundtrip_structured, complex_player
}
criterion_main!(benches);
