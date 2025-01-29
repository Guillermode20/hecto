use crossterm::event::{read, Event::Key, KeyCode::Char};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

pub struct Editor {}

impl Editor {
    pub fn default() -> Self {
        Editor {}
    }
    pub fn run(&self) {
        enable_raw_mode().unwrap();
        loop {
            match read() {
                Ok(Key(event)) => {
                    println!("{:?} \r", event);
                    match event.code {
                        // if event.code looks like a box with a character
                        Char(c) => {
                            // call the character c and execute the block
                            if c == 'q' {
                                break;
                            }
                        }
                        _ => (),
                        // for everything that didn't match, denoted by: _
                        // do nothing, denoted by: ()
                    }
                }
                Err(err) => println!("Error: {err}"),
                _ => (),
            }
        }
        disable_raw_mode().unwrap();
    }
}
