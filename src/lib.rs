//! Bit manipulation for dummies
//!
//! A easy to use utility for bit manipulation
//!
//! ```rust
//! use uberbyte::UberByte;
//!
//! fn main() {
//!     let my_byte: UberByte = UberByte::from(42);
//!
//!     println!("{:b}", my_byte);
//!     for index in 0..7 {
//!         if my_byte.is_bit_set(index) {
//!             println!("Bit on position {} is set", index);
//!         } else {
//!             println!("Bit on position {} is not set", index);
//!         }
//!     }
//! }
//! ```
//!
//! # Additional resources
//!
//! Additional resources are available at (GitHub project page)[https://github.com/dejanfajfar/uberbyte.rs]

pub mod uberbyte;
pub mod byte_array;

pub use uberbyte::*;

/// Defines the 0 bit bit mask
/// bin: 0000 0001
/// dec: 1
/// oct: 1
/// hex: 1
pub const ZERO_BIT_MASK: u8 = 0b_0000_0001;
/// Defines the 1 bit bit mask
/// bin: 0000 0010
/// dec: 2
/// oct: 2
/// hex: 2
pub const FIRST_BIT_MASK: u8 = 0b_0000_0010;
/// Defines the 2 bit bit mask
/// bin: 0000 0100
/// dec: 4
/// oct: 4
/// hex: 4
pub const SECOND_BIT_MASK: u8 = 0b_0000_0100;
/// Defines the 3 bit bit mask
/// bin: 0000 1000
/// dec: 8
/// oct: 10
/// hex: 8
pub const THIRD_BIT_MASK: u8 = 0b_0000_1000;
/// Defines the 4 bit bit mask
/// bin: 0001 0000
/// dec: 16
/// oct: 20
/// hex: 10
pub const FOURTH_BIT_MASK: u8 = 0b_0001_0000;
/// Defines the 5 bit bit mask
/// bin: 0010 0000
/// dec: 32
/// oct: 40
/// hex: 20
pub const FIFTH_BIT_MASK: u8 = 0b_0010_0000;
/// Defines the 6 bit bit mask
/// bin: 0100 0000
/// dec: 64
/// oct: 100
/// hex: 40
pub const SIXTH_BIT_MASK: u8 = 0b_0100_0000;
/// Defines the 0 bit bit mask
/// bin: 1000 0000
/// dec: 128
/// oct: 200
/// hex: 80
pub const SEVENTH_BIT_MASK: u8 = 0b_1000_0000;
/// Defines the 0 bit bit mask
/// bin: 0000 0000
/// dec: 0
/// oct: 0
/// hex: 0
pub const NONE_BIT_MASK: u8 = 0b_0000_0000;
/// Defines the 0 bit bit mask
/// bin: 1111 1111
/// dec: 255
/// oct: 377
/// hex: ff
pub const ALL_BIT_MASK: u8 = 0b_1111_1111;

/// Defines the possible errors that can happen inside the _UberByte_ crate
#[derive(Debug)]
pub enum UberByteError {
    /// The provided data overflows a single byte
    ValueOverflow,
    /// The provided data underflows a single byte
    ValueUnderflow,
    /// The desired index is exceeding the length of the array 
    IndexOutOfRange,
}
