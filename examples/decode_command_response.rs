use std::str::FromStr;

use uberbyte::{ByteArray, UberByte};

/*
Getting a byte array and having to make sense of it is something
that should be easily

The structure of the byte array is as follows

byte 01 -> command identifier
byte 02 -> device id
byte 03 -> device id
byte 04 ->
*/
fn main() {
    let foo = ByteArray::from(vec![12, 13, 14]);

    let slice = to_u16(foo[1..2].to_vec());
    let deviceId = u16::from(slice);
}

fn to_u16(bytes: Vec<UberByte>) -> u16 {
    let u8_array: Vec<u8> = bytes.into_iter().map(|f| f.into()).collect();
    let foo = u8_array
        .chunks_exact(2)
        .map(|chunk| <[u8; 2]>::try_from(chunk).unwrap())
        .next()
        .unwrap();

    return u16::from_be_bytes(foo);
}
