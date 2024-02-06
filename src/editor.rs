use std::default;
use std::rc::Weak;
use crate::document;
use crate::terminal;
use crate::Row;
use crate::Document;
use crate::terminal::Terminal;
use termion::event::Key; // we import the Key enum
use std::env;

const VERSION  : &str = env!("CARGO_PKG_VERSION"); // we are creating a constant called VERSION that is a string



#[derive(Default)] // this is a derive attribute that allows us to derive the default trait for the struct
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Editor {
    // a struct is a collection of variables, functions, which are grouped together to form an unity
    // pub: means that we can access this struct from outside the file
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    offset: Position,
    document: Document,
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
        let args: Vec<String> = env::args().collect();
        let document = if args.len() > 1 {
            let file_name = &args[1];
            Document::open(&file_name).unwrap_or_default();
        }
        else {
            Document::default()
        };
        Self {
            should_quit:false, // we are initializing the struct with the should_quit variable set to false
            terminal: Terminal::default().expect("Failed to initialize terminal"), // we are initializing the terminal
            cursor_position: Position {x: 0, y:0},
            document,
            offset: Position::default(),
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        // print!("\x1b[2J"); // clear the screen
        // \x means that we are using hexadecimal numbers
        // 1b is the hexadecimal number for the escape key
        // [2J is the code to clear the screen ;; for reference (https://vt100.net/docs/vt100-ug/chapter3.html#ED)
        // OR we could do this
        Terminal::cursor_hide();
        Terminal::clear_screen(); // clear the screen
        Terminal::cursor_position(&Position::default()); // move the cursor to the top left corner
        if self.should_quit {
            Terminal::clear_screen();
            println!("{} 👾 Quitting Candle 🕯️, Goodbye.❤️\r \n", termion::color::Fg(termion::color::Cyan));
        }
        else {
            self.draw_rows();
            Terminal::cursor_position(&self.cursor_position); // move the cursor to the top left corner
        }
        Terminal::cursor_show();
        Terminal::flush() // flush the screen
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error>{
        let key_pressed = Terminal::read_key()?;

        match key_pressed {
            Key::Ctrl('q') => self.should_quit = true,
            Key::Up
            | Key::Down
            | Key::Left
            | Key::Right
            | Key::PageUp
            | Key::PageDown
            | Key::Home
            => self.move_cursor(key_pressed),
            _ => (),
        }
        self.scroll();
        Ok(())
    }

    fn scroll(&mut self) {
        let Position {mut y, mut x} = self.cursor_position;
        let width =  self.terminal.size().width as usize;
        let height = self.terminal.size().height as usize;
        let mut offset = &mut self.offset;

        if y < offset.y {
            offset.y = y;
        }
        else if y >= offset.y.saturating_add(height){
            offset.y = y.saturating_sub(height).saturating_add(1);
        }

        if x < offset.x {
            offset.x = x;
        }
        else if x >= offset.x.saturating_add(width) {}
    }

    fn move_cursor(&mut self, key: Key){
        let Position {mut x, mut y} = self.cursor_position;
        let size = self.terminal.size();
        let height = size.height.saturating_sub(1) as usize; // we are creating a variable called height that is the height of the terminal
        let width = size.width.saturating_sub(1) as usize; // we are creating a variable called width that is the width of the terminal

        match key {
            Key::Up => y = y.saturating_sub(1), // we are subtracting 1 from y
            Key::Down => {
                if y < height {
                    y = y.saturating_add(1); // we are adding 1 to y
                }
            } // we are adding 1 to y
            Key::Left => x = x.saturating_sub(1), // we are subtracting 1 from x
            Key::Right => {
                if x < width {
                    x = x.saturating_add(1); // we are adding 1 to x
                }
            },// we are adding 1 to x
            Key::PageUp => y = 0,
            Key::PageDown => y = height,
            Key::Home => x = 0,
            Key::End => x = width,

            _ => (),
        }
        self.cursor_position = Position {x, y};
    }

    fn draw_welcome_message(&self) {
        let mut welcome_message = format!("👾 Welcome to Candle Editor!🕯️ {}\r", VERSION); // string that will be used as a message
        let width = self.terminal.size().width as usize; // we are creating a variable called width that is the width of the terminal
        let len = welcome_message.len(); // we are creating a variable called len that is the length of the welcome message
        let padding = width.saturating_sub(len) / 2; // we are calculating the padding
        let spaces = " ".repeat(padding.saturating_sub(1)); // we are creating a string of spaces
        welcome_message = format!("~{}{}", spaces, welcome_message); // we are adding the spaces to the welcome message
        welcome_message.truncate(width); // we are truncating the welcome message to the width of the terminal
        println!("{}\r", welcome_message);
    }

    fn draw_row(&self, row: &Row) {
        let width = self.terminal.size().width as usize; // we are creating a variable called width that is the width of the terminall
        let start = self.offset.x;
        let end = self.offset.x + width;
        let row = row.render(start, end);
        println!("{}\r", row);
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height; // we are creating a variable called height that is the height of the terminal
        for terminal_row in 0..height - 1 { // we are printing 24 tildes
            Terminal::clear_current_line(); // clear the current line
            if let Some(row) = self.document.row(terminal_row as usize + self.offset.y as usize) {
                self.draw_row(row);
            }
            else if self.document.is_empty() && terminal_row == height / 3 {
                self.draw_welcome_message();
            }
            else {
                println!("👾 \r");
            }
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
