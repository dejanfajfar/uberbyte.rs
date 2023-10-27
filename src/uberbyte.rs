use std::{
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Shl,
        ShlAssign, Shr, ShrAssign,
    },
    usize,
};

use crate::{
    ALL_BIT_MASK, FIFTH_BIT_MASK, FIRST_BIT_MASK, FOURTH_BIT_MASK, NONE_BIT_MASK, SECOND_BIT_MASK,
    SEVENTH_BIT_MASK, SIXTH_BIT_MASK, THIRD_BIT_MASK, ZERO_BIT_MASK,
};

pub mod formatters;
pub mod try_from;

/// Implements a simple wrapper over a __u8__ that allows you simple bit manipulation
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Copy)]
pub struct UberByte {
    value: u8,
}

/// A simple byte manipolation utility
impl UberByte {
    /// Represents the maximal possible value that the _UberByte_ can handle.
    /// All bits are set to 1.
    ///
    /// # Example
    ///
    /// ```rust
    /// use uberbyte::UberByte;
    ///
    /// let max = UberByte::MAX;
    /// ```
    pub const MAX: UberByte = UberByte {
        value: ALL_BIT_MASK,
    };

    /// Represents the minimal possible value that a _UberByte_ can handle.
    ///
    /// # Example
    ///
    /// ```rust
    /// use uberbyte::UberByte;
    ///
    /// let min = UberByte::MIN;
    /// ```
    pub const MIN: UberByte = UberByte {
        value: NONE_BIT_MASK,
    };

    /// Returns a new instance of a UberByte with the bits set
    ///  to 1 according to the bit max given
    ///
    /// # Example
    ///
    /// ```rust
    /// use uberbyte::UberByte;
    ///
    /// let my_byte = UberByte::MIN;
    ///
    /// let new_byte = my_byte.set(0b_1000_1000);
    /// ```
    pub fn set(&self, bit_mask: u8) -> UberByte {
        let masked_value = (self.value ^ bit_mask) | self.value;

        return UberByte::from(masked_value);
    }

    /// Sets the bits to 1 according to the bit mask
    ///
    /// # Example
    ///
    /// ```rust
    /// use uberbyte::UberByte;
    ///
    /// let mut my_byte = UberByte::MIN;
    ///
    /// my_byte.set_mut(0b_1000_1000)
    /// ```
    pub fn set_mut(&mut self, bit_mask: u8) {
        self.value = (self.value ^ bit_mask) | self.value
    }

    /// Returns a new instance of a UberByte with the bits cleared
    /// according to the given bit mask
    ///
    /// # Example
    ///
    /// ```rust
    /// use uberbyte::UberByte;
    ///
    /// let my_byte = UberByte::MAX;
    ///
    /// let cleared_byte = my_byte.clear(0b_1000_1000);
    /// ```
    pub fn clear(&self, bit_mask: u8) -> UberByte {
        let masked_value = (self.value ^ bit_mask) & self.value;

        return UberByte::from(masked_value);
    }

    /// Clears the bits to 0 according to the given bit mask
    ///
    /// # Example
    ///
    /// ```rust
    /// use uberbyte::UberByte;
    ///
    /// let mut my_byte = UberByte::MIN;
    ///
    /// my_byte.clear_mut(0b_1000_1000)
    /// ```
    pub fn clear_mut(&mut self, bit_mask: u8) {
        self.value = (self.value ^ bit_mask) & self.value
    }

    /// Returns a new instance of a UberByte with all bits flipped
    ///
    /// # Explanation
    ///
    /// Flipping a bit will transform a bit to 0 if 1 and 1 if 0.
    ///
    /// 0 => 1
    /// 1 => 0
    ///
    /// # Example
    ///
    /// ```rust
    /// use uberbyte::UberByte;
    ///
    /// let my_byte = UberByte::MAX;
    ///
    /// let flipped_byte = my_byte.flip();
    /// ```
    pub fn flip(&self) -> UberByte {
        return UberByte::from(!self.value);
    }

    /// Flips all bits in the UberByte
    ///
    /// # Explanation
    ///
    /// Flipping a bit will transform a bit to 0 if 1 and 1 if 0.
    ///
    /// 0 => 1
    /// 1 => 0
    ///
    /// # Example
    ///
    /// ```rust
    /// use uberbyte::UberByte;
    ///
    /// let mut my_byte = UberByte::MAX;
    ///
    /// my_byte.flip_mut();
    /// ```
    pub fn flip_mut(&mut self) {
        self.value = !self.value;
    }

    /// Determines if all bits in the bit mask are also set
    ///
    /// # Returns
    ///
    /// True if all set bits in the bit mask are also set in the UberByte
    /// False if not
    ///
    /// # Example
    ///
    /// ```rust
    /// use uberbyte::UberByte;
    ///
    /// let my_byte = UberByte::from(0b_0101_1010);
    ///
    /// let are_set = my_byte.are_set(0b_0100_1000);
    ///
    /// if (are_set) {
    ///     println!("All bits set");
    /// }
    /// else {
    ///     println!("Not all bits set");
    /// }
    /// ```
    pub fn are_set(&self, bit_mask: u8) -> bool {
        self.value & bit_mask != 0
    }

    /// Determines if the given bit index is set
    ///
    /// # Returns
    ///
    /// TRUE if the index is set
    /// FALSE if the index is not set
    ///
    /// ## Remarks
    ///
    /// If the index given exceeds 7 then the answer is automatically FALSE
    ///
    /// # Example
    ///
    /// ```rust
    /// use uberbyte::UberByte;
    ///
    /// let my_byte = UberByte::from(42);
    ///
    /// for index in 0..7 {
    ///     println!("Bit on index {} is {}", index, my_byte.is_bit_set(index))
    /// }
    /// ```
    pub fn is_bit_set(&self, bit_index: usize) -> bool {
        match bit_index {
            0 => self.is_bit_0_set(),
            1 => self.is_bit_1_set(),
            2 => self.is_bit_2_set(),
            3 => self.is_bit_3_set(),
            4 => self.is_bit_4_set(),
            5 => self.is_bit_5_set(),
            6 => self.is_bit_6_set(),
            7 => self.is_bit_7_set(),
            _ => false,
        }
    }

    /// Determines if the bit at index 0 is set
    ///
    /// # Returns
    ///
    /// given 0000 0001 -> true
    /// given 0000 0000 -> false
    ///
    /// # Remarks
    ///
    /// Ignores the state of all other bits
    pub fn is_bit_0_set(&self) -> bool {
        self.are_set(ZERO_BIT_MASK)
    }

    /// Determines if the bit at index 1 is set
    ///
    /// # Returns
    ///
    /// given 0000 0010 -> true
    /// given 0000 0000 -> false
    ///
    /// # Remarks
    ///
    /// Ignores the state of all other bits
    pub fn is_bit_1_set(&self) -> bool {
        self.are_set(FIRST_BIT_MASK)
    }

    /// Determines if the bit at index 2 is set
    ///
    /// # Returns
    ///
    /// given 0000 0100 -> true
    /// given 0000 0000 -> false
    ///
    /// # Remarks
    ///
    /// Ignores the state of all other bits
    pub fn is_bit_2_set(&self) -> bool {
        self.are_set(SECOND_BIT_MASK)
    }

    /// Determines if the bit at index 3 is set
    ///
    /// # Returns
    ///
    /// given 0000 1000 -> true
    /// given 0000 0000 -> false
    ///
    /// # Remarks
    ///
    /// Ignores the state of all other bits
    pub fn is_bit_3_set(&self) -> bool {
        self.are_set(THIRD_BIT_MASK)
    }

    /// Determines if the bit at index 4 is set
    ///
    /// # Returns
    ///
    /// given 0001 0000 -> true
    /// given 0000 0000 -> false
    ///
    /// # Remarks
    ///
    /// Ignores the state of all other bits
    pub fn is_bit_4_set(&self) -> bool {
        self.are_set(FOURTH_BIT_MASK)
    }

    /// Determines if the bit at index 5 is set
    ///
    /// # Returns
    ///
    /// given 0010 0000 -> true
    /// given 0000 0000 -> false
    ///
    /// # Remarks
    ///
    /// Ignores the state of all other bits
    pub fn is_bit_5_set(&self) -> bool {
        self.are_set(FIFTH_BIT_MASK)
    }

    /// Determines if the bit at index 6 is set
    ///
    /// # Returns
    ///
    /// given 0100 0000 -> true
    /// given 0000 0000 -> false
    ///
    /// # Remarks
    ///
    /// Ignores the state of all other bits
    pub fn is_bit_6_set(&self) -> bool {
        self.are_set(SIXTH_BIT_MASK)
    }

    /// Determines if the bit at index 7 is set
    ///
    /// # Returns
    ///
    /// given 1000 0000 -> true
    /// given 0000 0000 -> false
    ///
    /// # Remarks
    ///
    /// Ignores the state of all other bits
    pub fn is_bit_7_set(&self) -> bool {
        self.are_set(SEVENTH_BIT_MASK)
    }

    /// Determines the number of set bits
    ///
    /// # Returns
    ///
    /// The number of set bits in the UberByte as a u8
    ///
    /// # Remarks
    ///
    /// If no bits are set the then 0 is returned
    ///
    /// If all bits are set then 8 is returned
    pub fn count_set_bits(&self) -> u8 {
        let mut count = 0;
        let mut n = self.value;

        while n > 0 {
            if n & 1 == 1 {
                count += 1;
            }
            n >>= 1;
        }

        count
    }
}

impl Default for UberByte {
    fn default() -> Self {
        Self {
            value: Default::default(),
        }
    }
}

impl AddAssign for UberByte {
    fn add_assign(&mut self, rhs: Self) {
        let sum = self.clone() + rhs;
        self.value = sum.value;
    }
}

impl Add for UberByte {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let sum: Option<u8> = self.value.checked_add(rhs.value);
        match sum {
            Some(s) => UberByte::from(s),
            None => UberByte::from(ALL_BIT_MASK),
        }
    }
}

impl BitOr for UberByte {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        UberByte::from(self.value | rhs.value)
    }
}

impl BitOrAssign for UberByte {
    fn bitor_assign(&mut self, rhs: Self) {
        self.value = self.value | rhs.value;
    }
}

impl BitAnd for UberByte {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        UberByte::from(self.value & rhs.value)
    }
}

impl BitAndAssign for UberByte {
    fn bitand_assign(&mut self, rhs: Self) {
        self.value = self.value & rhs.value;
    }
}

impl BitXor for UberByte {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        UberByte::from(self.value ^ rhs.value)
    }
}

impl BitXorAssign for UberByte {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.value = self.value ^ rhs.value;
    }
}

impl Shl for UberByte {
    type Output = Self;

    fn shl(self, rhs: Self) -> Self::Output {
        let shift_left = self.value << rhs.value;
        UberByte::from(shift_left)
    }
}

impl ShlAssign for UberByte {
    fn shl_assign(&mut self, rhs: Self) {
        self.value = self.value << rhs.value;
    }
}

impl Shr for UberByte {
    type Output = Self;

    fn shr(self, rhs: Self) -> Self::Output {
        let shift_right = self.value >> rhs.value;
        UberByte::from(shift_right)
    }
}

impl ShrAssign for UberByte {
    fn shr_assign(&mut self, rhs: Self) {
        self.value = self.value >> rhs.value;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn individual_bits_set() {
        let test_object = UberByte::from(FIFTH_BIT_MASK);

        assert_eq!(test_object.is_bit_0_set(), false);
        assert_eq!(test_object.is_bit_1_set(), false);
        assert_eq!(test_object.is_bit_2_set(), false);
        assert_eq!(test_object.is_bit_3_set(), false);
        assert_eq!(test_object.is_bit_4_set(), false);
        assert_eq!(test_object.is_bit_5_set(), true);
    }

    #[test]
    fn is_bit_set() {
        assert!(UberByte::MAX.is_bit_set(0));
        assert!(UberByte::MAX.is_bit_set(1));
        assert!(UberByte::MAX.is_bit_set(2));
        assert!(UberByte::MAX.is_bit_set(3));
        assert!(UberByte::MAX.is_bit_set(4));
        assert!(UberByte::MAX.is_bit_set(5));
        assert!(UberByte::MAX.is_bit_set(6));
        assert!(UberByte::MAX.is_bit_set(7));
        assert_eq!(false, UberByte::MAX.is_bit_set(8));

        assert_eq!(false, UberByte::MIN.is_bit_set(0));
        assert_eq!(false, UberByte::MIN.is_bit_set(1));
        assert_eq!(false, UberByte::MIN.is_bit_set(2));
        assert_eq!(false, UberByte::MIN.is_bit_set(3));
        assert_eq!(false, UberByte::MIN.is_bit_set(4));
        assert_eq!(false, UberByte::MIN.is_bit_set(5));
        assert_eq!(false, UberByte::MIN.is_bit_set(6));
        assert_eq!(false, UberByte::MIN.is_bit_set(7));

        assert_eq!(false, UberByte::from(42).is_bit_set(0));
        assert_eq!(true, UberByte::from(42).is_bit_set(1));
        assert_eq!(false, UberByte::from(42).is_bit_set(2));
        assert_eq!(true, UberByte::from(42).is_bit_set(3));
        assert_eq!(false, UberByte::from(42).is_bit_set(4));
        assert_eq!(true, UberByte::from(42).is_bit_set(5));
        assert_eq!(false, UberByte::from(42).is_bit_set(6));
        assert_eq!(false, UberByte::from(42).is_bit_set(7));
    }

    #[test]
    fn set_per_mask() {
        let test_object = UberByte::MIN;
        let test_result = test_object.set(0b_1001_0000);

        assert_eq!(test_result.value, 0b_1001_0000);
    }

    #[test]
    fn set_per_mask_collision() {
        let test_object = UberByte::from(0b_1000_1000);
        let test_result = test_object.set(0b_1001_0000);

        assert_eq!(test_result.value, 0b_1001_1000);
    }

    #[test]
    fn clear_per_mask() {
        let test_object = UberByte::MAX;
        let test_result = test_object.clear(0b_1001_0000);

        assert_eq!(test_result.value, 0b_0110_1111);
    }

    #[test]
    fn flip() {
        assert_eq!(UberByte::MAX.flip(), UberByte::MIN);
        assert_eq!(UberByte::MIN.flip(), UberByte::MAX);
        assert_eq!(
            UberByte::from(0b_0101_0101).flip(),
            UberByte::from(0b_1010_1010)
        );
    }

    #[test]
    fn clear_per_mask_collision() {
        let test_object = UberByte::from(0b_1010_0010);
        let test_result = test_object.clear(0b_1001_0000);

        assert_eq!(test_result.value, 0b_0010_0010);
    }

    #[test]
    fn are_set() {
        let test_object = UberByte::from(0b_0001_1000);

        assert!(test_object.are_set(THIRD_BIT_MASK));
        assert!(test_object.are_set(FOURTH_BIT_MASK));
        assert!(test_object.are_set(THIRD_BIT_MASK | FOURTH_BIT_MASK));
    }

    #[test]
    fn add_overflow() {
        let augend = UberByte::MAX;
        let added = UberByte::MAX;

        assert_eq!(UberByte::MAX, augend + added)
    }

    #[test]
    fn add() {
        let augend = UberByte::from(5);
        let added = UberByte::from(6);

        assert_eq!(UberByte::from(11), augend + added);
    }

    #[test]
    fn or() {
        let first = UberByte::from(0b_0001_0001);
        let second = UberByte::from(0b_0000_0001);

        let test_result = first | second;

        assert_eq!(UberByte::from(0b_0001_0001), test_result)
    }

    #[test]
    fn or_assign() {
        let mut test_result = UberByte::from(0b_0001_0001);
        let second = UberByte::from(0b_0000_0001);

        test_result |= second;

        assert_eq!(UberByte::from(0b_0001_0001), test_result)
    }

    #[test]
    fn xor() {
        let first = UberByte::from(0b_0001_0001);
        let second = UberByte::from(0b_0000_0001);

        let test_result = first ^ second;

        assert_eq!(UberByte::from(0b_0001_0000), test_result)
    }

    #[test]
    fn xor_assign() {
        let mut test_result = UberByte::from(0b_0001_0001);
        let second = UberByte::from(0b_0000_0001);

        test_result ^= second;

        assert_eq!(UberByte::from(0b_0001_0000), test_result)
    }

    #[test]
    fn and() {
        let first = UberByte::from(0b_0001_0001);
        let second = UberByte::from(0b_0000_0001);

        let test_result = first & second;

        assert_eq!(UberByte::from(0b_0000_0001), test_result)
    }

    #[test]
    fn and_assign() {
        let mut test_result = UberByte::from(0b_0001_0001);
        let second = UberByte::from(0b_0000_0001);

        test_result &= second;

        assert_eq!(UberByte::from(0b_0000_0001), test_result)
    }

    #[test]
    fn shl() {
        let test_object = UberByte::from(0b_0000_0001);
        let shift = UberByte::from(1);

        let test_result = test_object << shift;

        assert_eq!(UberByte::from(0b_0000_0010), test_result);
    }

    #[test]
    fn shl_assign() {
        let mut test_object = UberByte::from(0b_0000_0001);
        let shift = UberByte::from(1);

        test_object <<= shift;

        assert_eq!(UberByte::from(0b_0000_0010), test_object);
    }

    #[test]
    fn shr() {
        let test_object = UberByte::from(0b_0000_1000);
        let shift = UberByte::from(1);

        let test_result = test_object >> shift;

        assert_eq!(UberByte::from(0b_0000_0100), test_result);
    }

    #[test]
    fn shr_assign() {
        let mut test_object = UberByte::from(0b_0000_1000);
        let shift = UberByte::from(1);

        test_object >>= shift;

        assert_eq!(UberByte::from(0b_0000_0100), test_object);
    }

    #[test]
    fn count_set_bits() {
        assert_eq!(0, UberByte::MIN.count_set_bits());
        assert_eq!(8, UberByte::MAX.count_set_bits());
        assert_eq!(3, UberByte::from(0b_0001_0110).count_set_bits());
    }
}
