#![feature(test)]

extern crate test;


// Operations like + on u32 values is intended to never overflow. This is
// problematic for certain operations that depend on overflow and underflow,
// such as the following bitwise arithmetic. We avoid the issue of overflow
// and underflow protection by using the Wrapping type, which compiles to
// efficient machine code, but maintains a safe interface.
use std::num::Wrapping;


const ONE: Wrapping<usize> = Wrapping(1 as usize);

/// Converts the right-most 1 bit to a 0.
/// Returns 0 if all bits are 0.
///
/// ```
/// # use delight::binary_turn_off_rightmost_one;
/// let x = usize::from_str_radix("1001110", 2).unwrap();
/// let y = binary_turn_off_rightmost_one(x);
///
/// assert_eq!(format!("{:b}", y), "1001100");
/// ```
pub fn binary_turn_off_rightmost_one(x: usize) -> usize {
    let w = Wrapping(x);
    x & (w - ONE).0
}

/// Converts the right-most 0 bit to a 1.
/// Returns the value `std::usize::MAX` if all bits are 1.
///
/// ```
/// # use delight::binary_turn_on_rightmost_zero;
/// let x = usize::from_str_radix("1001111", 2).unwrap();
/// let y = binary_turn_on_rightmost_zero(x);
///
/// assert_eq!(format!("{:b}", y), "1011111");
/// ```
pub fn binary_turn_on_rightmost_zero(x: usize) -> usize {
    let w = Wrapping(x);
    x | (w + ONE).0
}

#[cfg(test)]
mod tests {

    use super::*;
    use test::Bencher;

    const N: usize = 1024;

    #[test]
    fn test_binary_turn_off_rightmost_one() {
        let x = usize::from_str_radix("1001110", 2).unwrap();
        let y = binary_turn_off_rightmost_one(x);

        assert_eq!(format!("{:b}", y), "1001100");
    }

    #[bench]
    fn bench_binary_turn_off_rightmost_one(b: &mut Bencher) {
        b.bytes = (std::mem::size_of::<usize>() * N) as u64;

        b.iter(|| {
            let n = test::black_box(N);

            (0..n).fold(0, |_, x| test::black_box(binary_turn_off_rightmost_one(x)))
        })
    }

    #[test]
    fn test_binary_turn_on_rightmost_zero() {
        let x = usize::from_str_radix("1001111", 2).unwrap();
        let y = binary_turn_on_rightmost_zero(x);

        assert_eq!(format!("{:b}", y), "1011111");
    }

    #[bench]
    fn bench_binary_turn_on_rightmost_zero(b: &mut Bencher) {
        b.bytes = (std::mem::size_of::<usize>() * N) as u64;

        b.iter(|| {
            let n = test::black_box(N);

            (0..n).fold(0, |_, x| test::black_box(binary_turn_on_rightmost_zero(x)))
        })
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
