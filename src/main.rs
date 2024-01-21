#![warn(clippy::all, clippy::pedantic)]

mod editor; // here we are importing the editor module

use editor::Editor; // here we are importing the Editor struct from the editor module

fn main() {
    let editor = Editor::default(); // we create an instance of the Editor struct
    editor.run(); // we call the run method of the Editor struct
}
