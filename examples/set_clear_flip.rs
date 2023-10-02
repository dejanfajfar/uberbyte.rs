use uberbyte::{UberByte, FIFTH_BIT_MASK};

fn main() {
    // set with with copy
    println!("original {:b}, after set {:b}", UberByte::MIN, UberByte::MIN.set(FIFTH_BIT_MASK));

    // set mut
    let mut set_byte = UberByte::MIN;
    set_byte.set_mut(FIFTH_BIT_MASK);
    println!("Byte after set {:b}", set_byte);

    // clear with copy
    println!("Original {:b}, after clear {:b}", UberByte::MAX, UberByte::MAX.clear(FIFTH_BIT_MASK));

    // clear mut 
    let mut clear_byte = UberByte::MAX;
    clear_byte.clear_mut(FIFTH_BIT_MASK);
    println!("Byte after clear {:b}", clear_byte);
    
    // flip with copy
    println!("Original {:b}, after flip {:b}", UberByte::from(42), UberByte::from(42).flip());

    // flip mut
    let mut flip_byte = UberByte::from(42);
    flip_byte.flip_mut();
    println!("Byte after flip {:b}", flip_byte);
}
