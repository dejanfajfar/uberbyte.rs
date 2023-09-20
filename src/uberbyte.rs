use std::ops::{Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign};

pub mod try_from;

pub const ZERO_BIT_MASK: u8 = 0b_0000_0001;
pub const FIRST_BIT_MASK: u8 = 0b_0000_0010;
pub const SECOND_BIT_MASK: u8 = 0b_0000_0100;
pub const THIRD_BIT_MASK: u8 = 0b_0000_1000;
pub const FOURTH_BIT_MASK: u8 = 0b_0001_0000;
pub const FIFTH_BIT_MASK: u8 = 0b_0010_0000;
pub const SIXTH_BIT_MASK: u8 = 0b_0100_0000;
pub const SEVENTH_BIT_MASK: u8 = 0b_1000_0000;
pub const NONE_BIT_MASK: u8 = 0b_0000_0000;
pub const ALL_BIT_MASK: u8 = 0b_1111_1111;

#[derive(Debug)]
pub enum UberByteError {
    ValueOverflow,
    ValueUnderflow
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct UberByte {
    value: u8,
}

impl UberByte {
    pub const MAX: UberByte = UberByte { value: ALL_BIT_MASK };

    pub const MIN: UberByte = UberByte { value: NONE_BIT_MASK };

    pub fn set(&self, bit_mask: u8) -> UberByte {
        let masked_value = (self.value ^ bit_mask) | self.value;

        return UberByte::from(masked_value);
    }

    pub fn are_set(&self, bit_mask: u8) -> bool {
        self.value & bit_mask != 0
    }

    pub fn is_bit_0_set(&self) -> bool {
        self.are_set(ZERO_BIT_MASK)
    }

    pub fn is_bit_1_set(&self) -> bool {
        self.are_set(FIRST_BIT_MASK)
    }

    pub fn is_bit_2_set(&self) -> bool {
        self.are_set(SECOND_BIT_MASK)
    }

    pub fn is_bit_3_set(&self) -> bool {
        self.are_set(THIRD_BIT_MASK)
    }

    pub fn is_bit_4_set(&self) -> bool {
        self.are_set(FOURTH_BIT_MASK)
    }

    pub fn is_bit_5_set(&self) -> bool {
        self.are_set(FIFTH_BIT_MASK)
    }

    pub fn is_bit_6_set(&self) -> bool {
        self.are_set(SIXTH_BIT_MASK)
    }

    pub fn is_bit_7_set(&self) -> bool {
        self.are_set(SEVENTH_BIT_MASK)
    }
}

impl From<u8> for UberByte {
    fn from(value: u8) -> Self {
        UberByte { value: value }
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
    fn set_per_mask() {
        let test_object = UberByte::from(0b_0001_1000);
        let test_result = test_object.set(0b_1001_0000);

        assert_eq!(test_result.value, 0b_1001_1000);
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
}
