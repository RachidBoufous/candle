
pub struct Size { // we are creating a struct called size
    pub width: u16, // we are creating a variable called width that is a 16 bit unsigned integer
    pub height: u16, // we are creating a variable called height that is a 16 bit unsigned integer
}

pub struct Terminal {
    size: Size, // we are creating a variable called size that is a Size struct
}

impl Terminal {
    pub fn default() -> Result<Self, std::io::Error> {
        let size = termion::terminal_size()?;

        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1,
            }
        })
    }

    pub fn size(&self) -> &Size { // we are creating a method called size that returns a reference to a Size struct
        &self.size
    }
}