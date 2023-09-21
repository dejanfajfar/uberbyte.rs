use uberbyte::{UberByte, FIFTH_BIT_MASK, SEVENTH_BIT_MASK};


fn main(){
    let my_byte = UberByte::from(u8::MAX);

    // Check if the fifth and the seventh bit are 1
    let all_required_set = my_byte.are_set(FIFTH_BIT_MASK | SEVENTH_BIT_MASK);

    println!("The fifth and seventh bit are set {}", all_required_set);
}