use cursive::view::Resizable;

use crate::terminal_vc::terminal_actual::Terminal;
use crate::terminal_vc::terminal_test::TestKR;

mod controller;
mod game_state;
mod actor;
mod damage_types;
mod terminal_vc;
mod factions;
mod schedulers;

fn main() {
    let mut siv = cursive::default();
    siv.add_layer(Terminal::new().full_screen());
    //siv.add_layer(TestKR::new().full_screen());
    siv.run();
}
