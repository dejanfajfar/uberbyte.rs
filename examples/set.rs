use uberbyte::{UberByte, FIFTH_BIT_MASK};

fn main() {
    // set with with copy
    let my_byte = UberByte::MIN;
    let my_set_byte = my_byte.set(0b_0001_0001);

    println!("original {:b}, after set {:b}", my_byte, my_set_byte);

    // set mut
    let mut my_byte2 = UberByte::MIN;
    my_byte2.set_mut(FIFTH_BIT_MASK);

    println!("Byte after set {:b}", my_byte2);
}
