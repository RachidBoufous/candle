use std::io::{self, stdout};
use termion::event::Key; // we import the Key enum
use termion::input::TermRead; // we import the TermRead trait
use termion::raw::IntoRawMode; //

pub struct Editor {
    // a struct is a collection of variables, functions, which are grouped together to form an unity
    // pub: means that we can access this struct from outside the file
    should_quit: bool,
}

impl Editor {
    pub fn run(&mut self) {
        let _stdout = stdout().into_raw_mode().unwrap(); // read data from standard input (the keyboard)

        loop {
            if let Err(error) = self.process_keypress() {
                die(error);
            }

            if self.should_quit {
                break;
            }
        }
    }

    pub fn default() -> Self {
        Self {should_quit:false}
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error>{
        let key_pressed = read_key()?;

        match key_pressed {
            Key::Ctrl('q') => self.should_quit = true,
            _ => (),
        }
        Ok(())
    }


}

fn read_key() -> Result<Key, std::io::Error> {
    loop {
        if let Some(key_pressed) = io::stdin().lock().keys().next() {
            return key_pressed;
        }
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
