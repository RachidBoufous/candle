use crate::terminal::Terminal;
use termion::event::Key; // we import the Key enum


pub struct Editor {
    // a struct is a collection of variables, functions, which are grouped together to form an unity
    // pub: means that we can access this struct from outside the file
    should_quit: bool,
    terminal: Terminal,
}

impl Editor {
    pub fn run(&mut self) {

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
        Self {
            should_quit:false, // we are initializing the struct with the should_quit variable set to false
            terminal: Terminal::default().expect("Failed to initialize terminal"), // we are initializing the terminal
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        // print!("\x1b[2J"); // clear the screen
        // \x means that we are using hexadecimal numbers
        // 1b is the hexadecimal number for the escape key
        // [2J is the code to clear the screen ;; for reference (https://vt100.net/docs/vt100-ug/chapter3.html#ED)
        // OR we could do this
        Terminal::clear_screen(); // clear the screen
        Terminal::cursor_position(0,0); // move the cursor to the top left corner
        if self.should_quit {
            println!("{} 👾 Quitting Candle 🕯️, Goodbye.❤️\r \n", termion::color::Fg(termion::color::Cyan));
        }
        else {
            self.draw_rows();
            print!("{}", termion::cursor::Goto(1,1)); // move the cursor to the top left corner
        }
        Terminal::flush() // flush the screen
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error>{
        let key_pressed = Terminal::read_key()?;

        match key_pressed {
            Key::Ctrl('q') => self.should_quit = true,
            _ => (),
        }
        Ok(())
    }

    fn draw_rows(&self) {
        for _ in 0..self.terminal.size().height { // we are printing 24 tildes
            println!("👾\r");
        }
    }


}



/// Handles an error by panicking.
///
/// # Arguments
///
/// * `e` - The error to handle.
fn die(e: std::io::Error) {
    Terminal::clear_screen(); // clear the screen
    panic!("{}", e); // a macro that crashes the program and prints the error
}
