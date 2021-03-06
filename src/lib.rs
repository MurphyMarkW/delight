// Operations like + on u32 values is intended to never overflow. This is
// problematic for certain operations that depend on overflow and underflow,
// such as the following bitwise arithmetic. We avoid the issue of overflow
// and underflow protection by using the Wrapping type, which compiles to
// efficient machine code, but maintains a safe interface.
use std::num::Wrapping;


const ZERO: Wrapping<usize> = Wrapping(0 as usize);
const ONE: Wrapping<usize> = Wrapping(1 as usize);

/// Converts the right-most 1 bit to a 0.
/// Returns 0 if all bits are 0.
///
/// ```
/// # use delight::binary_turn_off_rightmost_one;
/// let x = usize::from_str_radix("11001110", 2).unwrap();
/// let y = binary_turn_off_rightmost_one(x);
///
/// assert_eq!(format!("{:08b}", y), "11001100");
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
/// let x = usize::from_str_radix("11001111", 2).unwrap();
/// let y = binary_turn_on_rightmost_zero(x);
///
/// assert_eq!(format!("{:08b}", y), "11011111");
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
/// let x = usize::from_str_radix("11011011", 2).unwrap();
/// let y = binary_turn_off_trailing_ones(x);
///
/// assert_eq!(format!("{:08b}", y), "11011000");
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
/// let x = usize::from_str_radix("11011000", 2).unwrap();
/// let y = binary_turn_on_trailing_zeros(x);
///
/// assert_eq!(format!("{:08b}", y), "11011111");
/// ```
pub fn binary_turn_on_trailing_zeros(x: usize) -> usize {
    let w = Wrapping(x);

    x | (w - ONE).0
}

/// Generates the bitmask identifying the rightmost 0 bit.
/// Returns 0 if there is no rightmost 0 bit.
///
/// ```
/// # use delight::binary_rightmost_zero_bitmask;
/// let x = usize::from_str_radix("11011011", 2).unwrap();
/// let y = binary_rightmost_zero_bitmask(x);
/// 
/// assert_eq!(format!("{:08b}", y), "00000100");
/// ```
pub fn binary_rightmost_zero_bitmask(x: usize) -> usize {
    let w = Wrapping(x);

    !x & (w + ONE).0
}

/// Generates the bitmask identifying the rightmost 1 bit.
/// Returns 0 if there is no rightmost 1 bit.
///
/// ```
/// # use delight::binary_rightmost_one_bitmask;
/// let x = usize::from_str_radix("11011000", 2).unwrap();
/// let y = binary_rightmost_one_bitmask(x);
/// 
/// assert_eq!(format!("{:08b}", y), "00001000");
/// ```
pub fn binary_rightmost_one_bitmask(x: usize) -> usize {
    let w = Wrapping(x);

    x & (ZERO - w).0
}

/// Generates the bitmask identifying any trailing 0 bits.
/// Returns 0 if there are no trailing 0 bits.
///
/// ```
/// # use delight::binary_trailing_zeros_bitmask;
/// let x = usize::from_str_radix("11011000", 2).unwrap();
/// let y = binary_trailing_zeros_bitmask(x);
///
/// assert_eq!(format!("{:08b}", y), "00000111");
/// ```
pub fn binary_trailing_zeros_bitmask(x: usize) -> usize {
    let w = Wrapping(x);

    !x & (w - ONE).0
}

/// Generates the bitmask identifying any trailing 1 bits.
/// Returns 0 if there are no trailing 1 bits.
///
/// ```
/// # use delight::binary_trailing_ones_bitmask;
/// let x = usize::from_str_radix("11011011", 2).unwrap();
/// let y = binary_trailing_ones_bitmask(x);
///
/// assert_eq!(format!("{:08b}", y), "00000011");
/// ```
pub fn binary_trailing_ones_bitmask(x: usize) -> usize {
    let w = Wrapping(x);

    x & !(w + ONE).0
}

/// Generates the bitmask with the leftmost contiguous run
/// of 1 bits disabled.
/// Returns 0 if there are no trailing 1 bits.
///
/// ```
/// # use delight::binary_trailing_ones_bitmask;
/// let x = usize::from_str_radix("11011110", 2).unwrap();
/// let y = binary_leading_ones_bitmask(x);
///
/// assert_eq!(format!("{:08b}", y), "11000000");
/// ```
pub fn binary_leading_ones_bitmask(x: usize) -> usize {
    let w = Wrapping(x);

    let rightmost = Wrapping(binary_rightmost_one_bitmask(x));

    ((rightmost + w) & w).0
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_binary_turn_off_rightmost_one() {
        let x = usize::from_str_radix("11001110", 2).unwrap();
        let y = binary_turn_off_rightmost_one(x);

        assert_eq!(format!("{:08b}", y), "11001100");
    }

    #[test]
    fn test_binary_turn_on_rightmost_zero() {
        let x = usize::from_str_radix("11001111", 2).unwrap();
        let y = binary_turn_on_rightmost_zero(x);

        assert_eq!(format!("{:08b}", y), "11011111");
    }

    #[test]
    fn test_binary_turn_off_trailing_ones() {
        let x = usize::from_str_radix("11011011", 2).unwrap();
        let y = binary_turn_off_trailing_ones(x);

        assert_eq!(format!("{:08b}", y), "11011000");
    }

    #[test]
    fn test_binary_turn_on_trailing_zeros() {
        let x = usize::from_str_radix("11011000", 2).unwrap();
        let y = binary_turn_on_trailing_zeros(x);

        assert_eq!(format!("{:08b}", y), "11011111");
    }

    #[test]
    fn test_binary_rightmost_zero_bitmask() {
        let x = usize::from_str_radix("11011011", 2).unwrap();
        let y = binary_rightmost_zero_bitmask(x);

        assert_eq!(format!("{:08b}", y), "00000100");
    }

    #[test]
    fn test_binary_rightmost_one_bitmask() {
        let x = usize::from_str_radix("11011000", 2).unwrap();
        let y = binary_rightmost_one_bitmask(x);

        assert_eq!(format!("{:08b}", y), "00001000");
    }

    #[test]
    fn test_binary_trailing_zeros_bitmask() {
        let x = usize::from_str_radix("11011000", 2).unwrap();
        let y = binary_trailing_zeros_bitmask(x);

        assert_eq!(format!("{:08b}", y), "00000111");
    }

    #[test]
    fn test_binary_trailing_ones_bitmask() {
        let x = usize::from_str_radix("11011011", 2).unwrap();
        let y = binary_trailing_ones_bitmask(x);

        assert_eq!(format!("{:08b}", y), "00000011");
    }

    #[test]
    fn test_binary_leading_ones_bitmask() {
        let x = usize::from_str_radix("11011110", 2).unwrap();
        let y = binary_leading_ones_bitmask(x);

        assert_eq!(format!("{:08b}", y), "11000000");
    }
}
