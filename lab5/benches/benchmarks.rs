use criterion::*;

macro_rules! benchmark {
    ($benchmark:ident; $($algorithm:ident),*) => {
        fn $benchmark(c: &mut Criterion) {
            let mut group = c.benchmark_group(stringify!($benchmark));

            $(
                group.bench_function(stringify!($algorithm), |b| {
                    let (p1, p2) = lab5::utils::generate_polynomials(1024);

                    b.iter(move || {
                        black_box(lab5::$benchmark::$algorithm::multiply(&p1, &p2));
                    });
                });
            )*
        }
    };
}

benchmark!(multiplication; simple, simple_par, karatsuba, karatsuba_par);

criterion_group!(benchmarks, multiplication);
criterion_main!(benchmarks);
