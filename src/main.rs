#[macro_use]
extern crate clap;

fn main() {
    app_from_crate!().get_matches();

    loop {
        println!("n");
    }
}
