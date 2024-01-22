use std::io::{self, stdout, Write};
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
            if let Err(error) = self.refresh_screen() {
                die(error);
            }

            if self.should_quit {
                break;
            }

            if let Err(error) = self.process_keypress() {
                die(error);
            }
        }
    }

    pub fn default() -> Self {
        Self {should_quit:false}
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        // print!("\x1b[2J"); // clear the screen
        // \x means that we are using hexadecimal numbers
        // 1b is the hexadecimal number for the escape key
        // [2J is the code to clear the screen ;; for reference (https://vt100.net/docs/vt100-ug/chapter3.html#ED)
        // OR we could do this
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1,1)); // clear the screen
        if self.should_quit {
            println!("{}👾 Quitting Candle 🕯️, Goodbye.❤️\r \n", termion::color::Fg(termion::color::Cyan));
        }
        else {
            self.draw_rows();
            print!("{}", termion::cursor::Goto(1,1)); // move the cursor to the top left corner
        }
        io::stdout().flush() // flush the screen
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error>{
        let key_pressed = read_key()?;

        match key_pressed {
            Key::Ctrl('q') => self.should_quit = true,
            _ => (),
        }
        Ok(())
    }

    fn draw_rows(&self) {
        for _ in 0..24 { // we are printing 24 tildes
            println!("👾\r");
        }
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
    print!("{}", termion::clear::All); // clear the screen
    panic!("{}", e); // a macro that crashes the program and prints the error
}
