use std::io::{self, stdout};
use termion::event::Key; // we import the Key enum
use termion::input::TermRead; // we import the TermRead trait
use termion::raw::IntoRawMode; //

pub struct Editor {
    // a struct is a collection of variables, functions, which are grouped together to form an unity
    // pub: means that we can access this struct from outside the file
}



impl Editor {
    pub fn run(&self) {
        let _stdout = stdout().into_raw_mode().unwrap(); // read data from standard input (the keyboard)

        for key in io::stdin().keys() {
            // now we are reading the keys pressed by the user
            match key {
                // we match the key pressed by the user
                Ok(key) => match key {
                    // we match the key pressed by the user
                    Key::Char(c) => {
                        // if the user presses a character key, we print the character
                        if c.is_control() {
                            // if the character is a control character, we print its code
                            println!("{:?}\r", c as u8); // the u8 cast converts the character to its byte value
                        } else {
                            println!("{:?} ({})\r", c as u8, c); // otherwise, we print the character and its code
                        }
                    }

                    Key::Ctrl('q') => break, // if the user presses ctrl + q , we exit the program
                    _ => println!("{:?}\r", key), // otherwise, we do nothing
                },

                Err(err) => die(err),
            }
        }
    }

    pub fn default() -> Self {
        Self {}
    }
}


/// Handles an error by panicking.
///
/// # Arguments
///
/// * `e` - The error to handle.
fn die(e: std::io::Error) {
    panic!("{}", e); // a macro that crashes the program and prints the error
}

