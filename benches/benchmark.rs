#![feature(test)]
extern crate test;

#[cfg(test)]
mod tests {

    use rust_benchmarking::isr::inverse_square_root::InverseSquareRoot;
    use test::black_box;
    use test::Bencher;

    #[bench]
    fn bench_isr(b: &mut Bencher) {
        let val: f32 = 0.123_456_f32;

        b.iter(|| {
            for _i in 1..100 {
                black_box(val.isr());
            }
        });
    }

    #[bench]
    fn bench_isr_optimized(b: &mut Bencher) {
        let val: f32 = 0.123_456_f32;

        b.iter(|| {
            for _i in 1..100 {
                black_box(val.isr_optimized());
            }
        });
    }

    #[bench]
    fn bench_isr_optimized_unsafe(b: &mut Bencher) {
        let val: f32 = 0.123_456_f32;

        b.iter(|| {
            for _i in 1..100 {
                black_box(val.isr_optimized_unsafe());
            }
        });
    }
}
