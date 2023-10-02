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

pub use uberbyte::*;
