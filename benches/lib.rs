#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {

    use delight::*;
    use test::Bencher;

    const N: usize = 1024;

    #[bench]
    fn bench_binary_turn_off_rightmost_one(b: &mut Bencher) {
        b.bytes = (std::mem::size_of::<usize>() * N) as u64;

        b.iter(|| {
            let n = test::black_box(N);

            (0..n).fold(0, |_, x| test::black_box(binary_turn_off_rightmost_one(x)))
        })
    }

    #[bench]
    fn bench_binary_turn_on_rightmost_zero(b: &mut Bencher) {
        b.bytes = (std::mem::size_of::<usize>() * N) as u64;

        b.iter(|| {
            let n = test::black_box(N);

            (0..n).fold(0, |_, x| test::black_box(binary_turn_on_rightmost_zero(x)))
        })
    }

    #[bench]
    fn bench_binary_turn_off_trailing_ones(b: &mut Bencher) {
        b.bytes = (std::mem::size_of::<usize>() * N) as u64;

        b.iter(|| {
            let n = test::black_box(N);

            (0..n).fold(0, |_, x| test::black_box(binary_turn_off_trailing_ones(x)))
        })
    }

    #[bench]
    fn bench_binary_turn_on_trailing_zeros(b: &mut Bencher) {
        b.bytes = (std::mem::size_of::<usize>() * N) as u64;

        b.iter(|| {
            let n = test::black_box(N);

            (0..n).fold(0, |_, x| test::black_box(binary_turn_on_trailing_zeros(x)))
        })
    }

    #[bench]
    fn bench_binary_rightmost_zero_bitmask(b: &mut Bencher) {
        b.bytes = (std::mem::size_of::<usize>() * N) as u64;

        b.iter(|| {
            let n = test::black_box(N);

            (0..n).fold(0, |_, x| test::black_box(binary_rightmost_zero_bitmask(x)))
        })
    }

    #[bench]
    fn bench_binary_rightmost_one_bitmask(b: &mut Bencher) {
        b.bytes = (std::mem::size_of::<usize>() * N) as u64;

        b.iter(|| {
            let n = test::black_box(N);

            (0..n).fold(0, |_, x| test::black_box(binary_rightmost_one_bitmask(x)))
        })
    }

    #[bench]
    fn bench_binary_trailing_zeros_bitmask(b: &mut Bencher) {
        b.bytes = (std::mem::size_of::<usize>() * N) as u64;

        b.iter(|| {
            let n = test::black_box(N);

            (0..n).fold(0, |_, x| test::black_box(binary_trailing_zeros_bitmask(x)))
        })
    }

    #[bench]
    fn bench_binary_trailing_ones_bitmask(b: &mut Bencher) {
        b.bytes = (std::mem::size_of::<usize>() * N) as u64;

        b.iter(|| {
            let n = test::black_box(N);

            (0..n).fold(0, |_, x| test::black_box(binary_trailing_ones_bitmask(x)))
        })
    }
}
