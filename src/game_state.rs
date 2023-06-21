use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

use serde::{Deserialize, Serialize};

use crate::actor::Actor;

const RANGE: u8 = 8;

#[derive(Serialize, Deserialize)]
pub(crate) struct Level {
    map: String,
    width: u8,
    height: u8,
    contamination: u32,
    playbill: Vec<Actor>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct GameState {
    levels: HashMap<u8, Level>,
    player: Actor,
    level: u8,
}

pub(crate) struct View {
    pub map: (String, u8, u8),
    pub player: Option<Actor>,
}

impl GameState {
    pub fn save(&self, path: &str) {
        let out = serde_json::ser::to_string(self).expect("Unable to serialise");
        let mut file = File::create(path).expect("Unable to create a file at the given path!");
        file.write_all(out.as_bytes()).expect("Unable to write");
    }

    pub fn gen_levels(&mut self, n: u8) {
        for i in 0..n {
            self.levels.insert(i, Level::new((30, 30)));
        }
        self.level = 0;
    }

    pub fn new() -> Self {
        let mut gs = GameState {
            levels: Default::default(),
            player: Actor::new_player(),
            level: 0,
        };
        gs.gen_levels(1);
        gs
    }

    pub fn send_message(&self) -> View {
        View {
            map: self.levels.get(&self.level).unwrap().gen_msg(),
            player: Some(self.player.clone()),
        }
    }
}

impl Level {
    pub fn in_los_empty(src: (u8, u8), dest: (u8, u8)) -> bool {
        dest.0.abs_diff(src.0) <= RANGE && dest.1.abs_diff(src.1) <= RANGE
    }

    pub fn new(size: (u8, u8)) -> Self {
        Level {
            map: Level::generate_empty(size),
            width: size.0,
            height: size.1,
            contamination: 0,
            playbill: vec![],
        }
    }

    fn generate_empty(size: (u8, u8)) -> String {
        // println!("{}", size.1 as usize * size.0 as usize);
        String::from(".".repeat(size.1 as usize * size.0 as usize))
    }

    fn gen_msg(&self) -> (String, u8, u8) {
        (self.map.clone(), self.width, self.height)
    }
}
