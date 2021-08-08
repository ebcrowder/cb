extern crate clap;
extern crate clipboard;

use clap::Clap;
use clipboard::{ClipboardContext, ClipboardProvider};
use std::io::{self};

/// clipboard manager
#[derive(Clap)]
#[clap(version = "0.1.0", author = "Eric Crowder <eric@ebcrowder.dev>")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    /// get a value from the system clipboard
    Get,
    /// set a value to the system clipboard
    Set,
    /// clear the system clipboard
    Clear,
}

fn read_from_stdin() -> Result<String, std::io::Error> {
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(_) => Ok(buffer),
        Err(error) => Err(error),
    }
}

fn set_clipboard_contents(mut ctx: ClipboardContext, input: String) {
    ctx.set_contents(input).unwrap();
    // read contents after setting so that they persist
    ctx.get_contents().unwrap_or_default();
}

fn get_clipboard_contents(mut ctx: ClipboardContext) -> String {
    ctx.get_contents().unwrap_or_default()
}

fn clear_clipboard_contents(mut ctx: ClipboardContext) {
    ctx.set_contents("".to_string()).unwrap();
    // read contents after setting so that they persist
    ctx.get_contents().unwrap_or_default();
}

fn main() {
    // parse arguments
    let opts: Opts = Opts::parse();

    // instantiate ClipboardProvider
    let ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    match opts.subcmd {
        SubCommand::Get => {
            let contents = get_clipboard_contents(ctx);
            println!("clipboard contents: {:?}", contents);
        }
        SubCommand::Set => {
            let b = read_from_stdin().unwrap();
            set_clipboard_contents(ctx, b);
            println!("value set to clipboard!");
        }
        SubCommand::Clear => {
            clear_clipboard_contents(ctx);
            println!("clipboard cleared!");
        }
    }
}
