use uberbyte::{ByteArray, UberByte, FIFTH_BIT_MASK};

/*
In this example the capabilities of the ByteArray is show when creating custom binary commands

This example will compose a command in the following format

byte 00 -> command category
byte 01 -> command identifier
byte 02 -> MAC address
byte 03 -> MAC address
byte 04 -> MAC address
byte 05 -> MAC address
byte 06 -> MAC address
byte 07 -> MAC address
byte 08 -> MAC address
byte 09 -> Command parameter
*/
fn main() {
    // Create a new instance of the ByteArray
    let mut command = ByteArray::default();
    // A sample MAC address
    let mac_address = ByteArray::from(vec![0xBC, 0xD0, 0x74, 0x22, 0x66, 0x37]);
    // 2 byte command category and command identifier
    let command_identifier = ByteArray::from(vec![12, 03]);

    // Add command identifier
    command += command_identifier;

    // Add target MAC address
    command += mac_address;

    // Add command parameters
    let command_parameter = UberByte::MIN;
    command_parameter.set(FIFTH_BIT_MASK);
    command.add_mut(command_parameter);
}
