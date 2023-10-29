use uberbyte::{ByteArray, UberByte, SECOND_BIT_MASK, FIFTH_BIT_MASK};

/*
Getting a byte array and having to make sense of it is something
that should be easily

The structure of the byte array is as follows

byte 01 -> command identifier
byte 02 -> device id
byte 03 -> device id
byte 04 -> device state
byte 05 -> device state
byte 05 -> device state
*/
fn main() {
    let foo = ByteArray::from(vec![12, 13, 14]);

    let slice = to_u16(foo[1..2].to_vec());
    let device_id = u16::from(slice);

    let device_state = foo[4..].to_vec();

    let is_set = device_state[0].are_set(SECOND_BIT_MASK | FIFTH_BIT_MASK);

    if is_set {
        println!("Devices {} special button is pressed", device_id);
    }
    else {
        println!("Devices {} special button is not pressed", device_id)
    }
}

fn to_u16(bytes: Vec<UberByte>) -> u16 {
    let u8_array: Vec<u8> = bytes.into_iter().map(|f| f.into_u8()).collect();
    let foo = u8_array
        .chunks_exact(2)
        .map(|chunk| <[u8; 2]>::try_from(chunk).unwrap())
        .next()
        .unwrap();

    return u16::from_be_bytes(foo);
}