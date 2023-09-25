# uberbyte.rs

> Byte manipulation for dummies

[![Rust](https://github.com/dejanfajfar/uberbyte.rs/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/dejanfajfar/uberbyte.rs/actions/workflows/ci.yml) [![Crates.io](https://img.shields.io/crates/v/uberbyte?style=flat-square)](https://crates.io/crates/uberbyte) [![docs.rs](https://img.shields.io/docsrs/uberbyte?style=flat-square&label=Documentation)](https://docs.rs/uberbyte/0.5.0/uberbyte/)


## Backstory



## Installation

The library is easily added to any project with a simple cargo command

```shell
cargo add uberbyte 
```

## So what can you do with it?

### See the bit states of byte

When you somehow get a hold of byte and you would like to know if the 3rd bit is a _1_ or a _0_ you can either start looking online for help or try __uberbyte__

```rust
use uberbyte

pub main(){
    let byte: u8 = 0b_0101_1100;

    let is_byte_3_set: bool = UberByte::from(byte).is_bit_3_set();

    ...
}
```

### Check if multiple bits are set

Lets take for example that you have to combine the sate of multiple bits in order to determine one flag in your application.

I had the case where bit 0,2,5 hat to be set in order for me to set and application internal flag. 

This can be easily done with

```rust
use uberbyte

pub main(){
    let byte: u8 = 0b_0101_1100;

    let my_flag: bool = UberByte::from(byte)
        .are_set(0b_0010_0101);

    ...
}
```