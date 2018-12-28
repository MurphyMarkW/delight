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

/// Converts any trailing 1 bits to 0.
/// Returns the input if it has no trailing 1 bits.
///
/// ```
/// # use delight::binary_turn_off_trailing_ones;
/// let x = usize::from_str_radix("1011011", 2).unwrap();
/// let y = binary_turn_off_trailing_ones(x);
///
/// assert_eq!(format!("{:b}", y), "1011000");
/// ```
pub fn binary_turn_off_trailing_ones(x: usize) -> usize {
    let w = Wrapping(x);
    x & (w + ONE).0
}

/// Converts any trailing 0 bits to 1.
/// Returns the input if it has no trailing 0 bits.
///
/// ```
/// # use delight::binary_turn_on_trailing_zeros;
/// let x = usize::from_str_radix("1011000", 2).unwrap();
/// let y = binary_turn_on_trailing_zeros(x);
///
/// assert_eq!(format!("{:b}", y), "1011111");
/// ```
pub fn binary_turn_on_trailing_zeros(x: usize) -> usize {
    let w = Wrapping(x);
    x | (w - ONE).0
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_binary_turn_off_rightmost_one() {
        let x = usize::from_str_radix("1001110", 2).unwrap();
        let y = binary_turn_off_rightmost_one(x);

        assert_eq!(format!("{:b}", y), "1001100");
    }

    #[test]
    fn test_binary_turn_on_rightmost_zero() {
        let x = usize::from_str_radix("1001111", 2).unwrap();
        let y = binary_turn_on_rightmost_zero(x);

        assert_eq!(format!("{:b}", y), "1011111");
    }

    #[test]
    fn test_binary_turn_off_trailing_ones() {
        let x = usize::from_str_radix("1011011", 2).unwrap();
        let y = binary_turn_off_trailing_ones(x);

        assert_eq!(format!("{:b}", y), "1011000");
    }

    #[test]
    fn test_binary_turn_on_trailing_zeros() {
        let x = usize::from_str_radix("1011000", 2).unwrap();
        let y = binary_turn_on_trailing_zeros(x);

        assert_eq!(format!("{:b}", y), "1011111");
    }
}
