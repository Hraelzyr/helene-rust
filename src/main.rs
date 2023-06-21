use cursive::view::Resizable;
use crate::terminal_vc::terminal_test::TestKR;
use ron::{ser, de};
use serde::{Deserialize, Serialize};
use crate::terminal_vc::terminal_actual::Terminal;

mod controller;
mod game_state;
mod actor;
mod damage_types;
mod terminal_vc;
mod factions;

#[derive(Serialize, Deserialize)]
struct KilllaKill {
    wow: i32,
}

fn main() {
    let mut siv = cursive::default();
    siv.add_layer(Terminal::new().full_screen());
    siv.run();

    // let wow = KilllaKill {
    //     wow: 0,
    // };
    //
    // let out=ser::to_string(&wow).unwrap();
    // println!("RON {}", out);
}
