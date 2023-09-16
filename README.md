# uberbyte.rs

> A humanly usable library to work with bytes

There is nothing in this library that cannot be done by normal bit and byte operations. 

My goal is to make it easier to interact with individual bits and bytes.

## Installation

TBD

`cargo add uberbyte`

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