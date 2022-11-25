use lab3::matrix::Matrix;
use criterion::{criterion_group, criterion_main, Criterion};

macro_rules! benchmark {
    ($benchmark:ident; $($method:ident),+) => {
        fn $benchmark(c: &mut Criterion) {
            let mut group = c.benchmark_group(stringify!($benchmark));
            let m1 = Matrix::random(1000, 900);
            let m2 = Matrix::random(800, 700);

            $(
                group.bench_function(stringify!($method), |b| {
                    let bench = lab3::$benchmark::$method::Benchmark::new();
                    b.iter(|| bench.run(&m1, &m2));
                });
            )+
        }
    };
}

benchmark!(
    row_major;
    sequential,
    threads,
    tasks
);

benchmark!(
    col_major;
    sequential,
    threads,
    tasks
);

benchmark!(
    sparse;
    threads,
    tasks
);

criterion_group!(
    benchmarks,
    row_major,
    col_major,
    sparse,
);

criterion_main!(benchmarks);