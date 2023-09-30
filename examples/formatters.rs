use uberbyte::UberByte;

/*
In the case that you want to print a UberByte you have multiple formatters available
*/

fn main(){
    let byte = UberByte::from(128);

    println!("Binary:   {:b}", byte);
    println!("LowHex:   {:x}", byte);
    println!("HighHex:  {:X}", byte);
    println!("Octa:     {:o}", byte);
    println!("Default:  {}", byte);
    println!("Debug:    {:?}", byte);

}