use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode;

/// Converts a character to its corresponding control byte.
///
/// # Arguments
///
/// * `c` - The character to convert.
///
/// # Returns
///
/// The control byte of the character.
fn to_ctrl_byte(c: char) -> u8 {
    let byte = c as u8;
    byte & 0b0001_1111
}

/// Handles an error by panicking.
///
/// # Arguments
///
/// * `e` - The error to handle.
fn die(e: std::io::Error) {
    panic!("{}", e); // a macro that crashes the program and prints the error
}

fn main() {
    let _stdout = stdout().into_raw_mode().unwrap();

    for b in io::stdin().bytes() {
        match b {  // match is like a switch statement
            Ok(b) => { // if the byte is valid (not an error)
                let c = b as char; // convert the byte to a character

                if c.is_control() { // we check if the character is a control character
                    println!("{:?} \r", b);
                } else {
                    println!("{:?} ({})\r", b, c);
                }


                if b == to_ctrl_byte('q') {
                        break;
                }

            }
            Err(e) => die(e), // if the byte is invalid, crash the program
        };
    }
}
