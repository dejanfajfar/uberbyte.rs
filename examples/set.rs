use uberbyte::UberByte;

fn main(){
    let my_byte = UberByte::MIN;

    let my_set_byte = my_byte.set(0b_0001_0001);

    println!("original {:b}, after set {:b}", my_byte, my_set_byte);
}