use criterion::*;

macro_rules! benchmark {
    ($benchmark:ident; $($algorithm:ident),*) => {
        fn $benchmark(c: &mut Criterion) {
            let mut group = c.benchmark_group(stringify!($benchmark));

            $(
                group.bench_function(stringify!($algorithm), |b| {
                    let mut bench = lab5::$benchmark::$algorithm::Benchmark::new();
                    b.iter(move || bench.run());
                });
            )*
        }
    };
}

benchmark!(multiplication; simple, simple_par);

criterion_group!(benchmarks, multiplication);
criterion_main!(benchmarks);
