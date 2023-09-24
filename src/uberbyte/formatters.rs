use std::fmt::{Binary, LowerHex, UpperHex, Octal};

use crate::UberByte;

impl Binary for UberByte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:b}", self.value))
    }
}

impl LowerHex for UberByte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:x}", self.value))
    }
}

impl UpperHex for UberByte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:X}", self.value))
    }
}

impl Octal for UberByte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:o}", self.value))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bit_formatter(){
        let my_bit = UberByte::from(0b_1001_1001);

        assert_eq!(String::from("10011001"), format!("{:b}", my_bit));
    }

    #[test]
    fn lowerhex_formatter(){
        let my_bit = UberByte::from(0b_1101_1001);

        assert_eq!(String::from("d9"), format!("{:x}", my_bit));
    }

    #[test]
    fn upperhex_formatter(){
        let my_bit = UberByte::from(0b_1101_1001);

        assert_eq!(String::from("D9"), format!("{:X}", my_bit));
    }
}