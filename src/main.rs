#![warn(clippy::all, clippy::pedantic)]

mod editor; // here we are importing the editor module

use editor::Editor; // here we are importing the Editor struct from the editor module

fn main() {
    
    Editor::default().run(); // we call the run method of the Editor struct
}
