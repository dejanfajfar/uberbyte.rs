use uberbyte::UberByte;

/*
Before you can use the UberByte you have to create it.

And there are many ways to create one
*/

fn main(){
    // Create a uber byte from an unsigned u8
    println!("From u8:          {:b}", UberByte::from(12u8));
    println!("MAX value:        {:b}", UberByte::MAX);
    println!("MIN value:        {:b}", UberByte::MIN);
    println!("Default value:    {:b}", UberByte::default());

    // You can try and create it from any numerical type
    let byte = UberByte::try_from(300u32);
    println!("From 300u32:      {:?}", byte);

    // There is also the possibility to provide a negative number
    let byte2 = UberByte::try_from(-300);
    println!("From 300u32:      {:?}", byte2);
}