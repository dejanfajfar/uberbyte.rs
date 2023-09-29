//! Byte manipulation for dummies
//!
//! Contains simple utilities to help you make sense ob bits and bytes
//! without having to resort to bit shifting and other bit shenanigans.
//!
//! # Quick start
//!
//! Add the uberbyte crate to your project with
//!
//! ```shell
//! cargo install uberbyte
//! ```
//!
//! after that you can simply use it in your application
//!
//! ```rust
//! use uberbyte::UberByte;
//!
//! fn main(){
//!     let input:u8 = 8;
//!
//!     let my_byte = UberByte::from(input);
//!
//!     println!("Is bit 00 set: {}", my_byte.is_bit_0_set());
//!     println!("Is bit 01 set: {}", my_byte.is_bit_1_set());
//!     println!("Is bit 02 set: {}", my_byte.is_bit_2_set());
//!     println!("Is bit 03 set: {}", my_byte.is_bit_3_set());
//!     println!("Is bit 04 set: {}", my_byte.is_bit_4_set());
//!     println!("Is bit 05 set: {}", my_byte.is_bit_5_set());
//!     println!("Is bit 06 set: {}", my_byte.is_bit_6_set());
//!     println!("Is bit 07 set: {}", my_byte.is_bit_7_set());
//! }
//! ```

pub mod uberbyte;

pub use uberbyte::*;