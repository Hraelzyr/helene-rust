mod main_menu;

extern crate ferris_says;

use ferris_says::say;
use std::io::{ stdout, BufWriter };

fn main() {
    let out = "Welcome to Helene!\nI am the Crab of Life and Death";
    let width = 14;

    let mut writer = BufWriter::new(stdout());
    say(out, width, &mut writer).unwrap();
}
