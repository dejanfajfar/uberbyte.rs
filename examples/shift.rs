use uberbyte::UberByte;

fn main() {
    let my_byte = UberByte::from(0x11);

    let left = my_byte << UberByte::from(1);
    let right = my_byte >> UberByte::from(1);

    println!("Left {:b}, right {:b}", left, right);
}
