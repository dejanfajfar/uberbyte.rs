use uberbyte::UberByte;

/// A simple example showing how to check if a given bit in the byte is set

fn main() {
    let my_byte: UberByte = UberByte::from(42);

    println!("{:b}", my_byte);
    for index in 0..7 {
        if my_byte.is_bit_set(index) {
            println!("Bit on position {} is set", index);
        } else {
            println!("Bit on position {} is not set", index);
        }
    }
}
