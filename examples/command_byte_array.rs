use uberbyte::{ByteArray, UberByte, FIFTH_BIT_MASK};

fn main() {
    let mut command = ByteArray::default();
    let mac_address = ByteArray::from(vec![0xBC, 0xD0, 0x74, 0x22, 0x66, 0x37]);
    let command_identifier = ByteArray::from(vec![12, 03]);

    // Add command start
    command.add_mut(UberByte::from(0xFF));

    // Add command identifier
    command += command_identifier;

    // Add target MAC address
    command += mac_address;

    // Add command parameters
    let command_parameter = UberByte::MIN;
    command_parameter.set(FIFTH_BIT_MASK);
    command.add_mut(command_parameter);
}
