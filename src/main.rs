// #![warn(clippy::all, clippy::pedantic)]

mod editor; // here we are importing the editor module
mod terminal; // here we are importing the terminal module
mod document;
mod row;
pub use document::Document;
pub use row::Row;
use editor::Editor; // here we are importing the Editor struct from the editor module
pub use terminal::Terminal;
pub use editor::Position;


fn main() {
    Editor::default().run(); // we call the run method of the Editor struct
}
