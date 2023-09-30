use uberbyte::{UberByte, FIFTH_BIT_MASK, SEVENTH_BIT_MASK};


/*
One of basic tasks that I was always most annoyed was to find out if a specific bit or bits are set.

This can be done by other bit manipulation methods but here we have a simple method to do this
 */

fn main() {
    // This will create a Uber Byte well all bits are set to 1
    let my_byte = UberByte::MAX;

    // Check if the fifth and the seventh bit are 1
    let are_set = my_byte.are_set(FIFTH_BIT_MASK | SEVENTH_BIT_MASK);

    println!("The fifth and seventh bit are set {}", are_set);

    // The same could be achieved by using the is_bit_X_set methods

    let are_set2 = my_byte.is_bit_5_set() && my_byte.is_bit_7_set();

    println!("The fifth and seventh bit are set {}", are_set2);
}
