use clap::Parser;

use std::fs::{File, OpenOptions};
use std::io::Write;

mod cli;
mod zero_width;

fn main() {
    let args = cli::App::parse();

    match args.action {
        cli::Program::Hide{cover, message} => {
            let path = std::path::absolute(".").unwrap();
            let path = path.join(args.path);

            println!("Covering '{message}' in '{cover}'");
            println!("Saving contents at {:#?} to preserve the hidden bytes", path.as_os_str());
            let hidden = zero_width::hide(cover, message);

            let mut file: File = OpenOptions::new().write(true).create(true).open(path).unwrap();

            file.write_all(hidden.as_bytes()).unwrap();
        },
        cli::Program::Reveal { payload } => {
            let result = zero_width::reveal(payload);

            if result.len() > 0 {
                println!("Revealed message: {}", result);
            } else {
                println!("No hidden message");
            }
        }
    }
}
