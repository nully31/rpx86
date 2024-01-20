//
// io
//
use std::io::{stdin, stdout, Read, Write};

pub fn in8(address: u16) -> u8 {
    match address {
        0x03f8 => getchar(),
        _ => 0,
    }
}

pub fn out8(address: u16, value: u8) {
    match address {
        0x03f8 => putchar(value),
        _ => (),
    }
}

fn getchar() -> u8 {
    let mut input = [0u8; 1];
    stdin().read_exact(&mut input).unwrap();
    input[0]
}

fn putchar(value: u8) {
    stdout().flush().unwrap();
    print!("{}", value as char);
    stdout().flush().unwrap();
}