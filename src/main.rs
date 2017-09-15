#[macro_use]
extern crate clap;

use std::io::Write;

fn main() {
    app_from_crate!().get_matches();

    loop {
        match writeln!(std::io::stdout(), "n") {
            Err(_) => break,
            Ok(()) => (),
        }
    }
}
