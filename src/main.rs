fn main() {
    let value = 0b1111_0101u8;
    println!("value is {}", value);
    println!("value is {:08b}", value);
    println!("not value is {:08b}", !value);
    println!("value but reset bit in position 2 is {:08b}", value & 0b1111_1011);
    println!("value if bit 6 is {}", value & 0b0100_0000);
    println!("set position 4 to 1 is {:08b}", value | 0b0000_1000);
    println!("set position 4 to 1 is {:08b}", value ^ 0b0000_1000);
    println!("set position 4 to 1 is {:08b}", value << 2);
    println!("set position 4 to 1 is {:08b}", value >> 3);
}
