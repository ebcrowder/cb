extern crate clipboard;

use clipboard::{ClipboardProvider, ClipboardContext};
use std::io::{self, Read, Write};

/// stdin manages the buffer received from stdin
/// and sent to stdout
struct StdIO {
    buffer: String,
}

impl StdIO {
    fn new() -> StdIO {
        StdIO {
            buffer: String::new()
        }
    }
    fn read_from_stdin(&mut self) {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                self.buffer = input;
            }
            Err(error) => println!("error: {}", error),
        }
    }
    fn write_to_stdout(self) -> Result<(), std::io::Error> {
        io::stdout().write_all(self.buffer.as_ref()).unwrap();
        Ok(())
    }
}

fn set_clipboard_contents(input: String) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(input).unwrap();
}

fn get_clipboard_contents() -> String {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let contents = ctx.get_contents().unwrap_or_default();
    contents
}

fn main() {
    let mut b = StdIO::new();
    b.read_from_stdin();

    set_clipboard_contents(b.buffer);
    let contents = get_clipboard_contents();
    println!("{:?}", contents);
}
