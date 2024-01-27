use std::io::{self, stdout, Write}; // we are importing the io module, the stdout function, and the Write trait
use termion::event::Key; // we import the Key enum
use termion::input::TermRead; // we import the TermRead trait
use termion::raw::{IntoRawMode, RawTerminal}; // we import the IntoRawMode trait and the RawTerminal struct

pub struct Size { // we are creating a struct called size
    pub width: u16, // we are creating a variable called width that is a 16 bit unsigned integer
    pub height: u16, // we are creating a variable called height that is a 16 bit unsigned integer
}

pub struct Terminal {
    size: Size, // we are creating a variable called size that is a Size struct
    _stdout: RawTerminal<std::io::Stdout>, // we are creating a variable called _stdout that is a RawTerminal struct
}

impl Terminal {
    pub fn default() -> Result<Self, std::io::Error> {
        let size = termion::terminal_size()?;

        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1,
            },
            _stdout: stdout().into_raw_mode()?,
        })
    }

    pub fn size(&self) -> &Size { // we are creating a method called size that returns a reference to a Size struct
        &self.size
    }

    pub fn clear_screen() {
        print!("{}", termion::clear::All); // clear the screen
    }

    pub fn cursor_position(x: u16, y: u16) {
        let x = x.saturating_add(1); // it checks if it possible to add 1 to x, if it is not possible it returns the maximum value of u16 
        let y = y.saturating_add(1); // this is to avoid overflow 111 + 1 = 000
        print!("{}", termion::cursor::Goto(x,y)); // move the cursor to the top left corner
    }

    pub fn flush() -> Result<(), std::io::Error> {
        io::stdout().flush() // flush the screen
    }

    pub fn read_key() -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() { // we are creating a variable called key that is a Key enum
                return key;
            }
        }
    }

    pub fn cursor_hide() {
        print!("{}", termion::cursor::Hide); // hide the cursor
    }

    pub fn cursor_show() {
        print!("{}", termion::cursor::Show); // show the cursor
    }

    pub fn clear_current_line() {
        print!("{}", termion::clear::CurrentLine); // clear the current line
    }

}