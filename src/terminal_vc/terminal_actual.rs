use std::cmp::min;
use std::sync::{Arc, Mutex, RwLock};
use cursive::{Printer, View};
use cursive::event::{Event, EventResult};
use cursive::event::Event::{AltChar, Key};


use crate::controller::{Controller, Message};
use crate::controller::Message::{Movement, SaveKey, ExitKey, LoadKey};
use crate::game_state::{GameState};

pub(crate) struct Terminal {
    game: Arc<RwLock<GameState>>,
    ctrl: Arc<Mutex<Controller>>,
}


fn map_key_to_move(ch: char) -> (i8, i8){
    match ch {
        '1' => { (-1, 1) }
        '2' => { (0, 1) }
        '3' => { (1, 1) }
        '4' => { (-1, 0) }
        // '5' => { (0, 0) }
        '6' => { (1, 0) }
        '7' => { (-1, -1) }
        '8' => { (0, -1) }
        '9' => { (1, -1)}
        _ => {
            (0,0)
        }
    }
}

impl View for Terminal{
    fn draw(&self, printer: &Printer) {
        // println!("Reached the draw");
        let msg = self.game.read().unwrap().send_message();
        let (map, wd, ht) = msg.map;
        let pl = msg.player;
        for i in 0..ht{
            let start=wd as usize*i as usize;
            let end = min(start+wd as usize, map.len());
            printer.print((0, i), &map[start..end]);
        }

        match pl{
            None => {}
            Some(ex) => {
                printer.print(ex.get_loc(), "@");
            }
        }
    }

    fn on_event(&mut self, mrn: Event) -> EventResult {
        let mut msg = Movement(0,0);
        let mut evt = EventResult::Consumed(None);
        match mrn{
            Event::Exit | Key(cursive::event::Key::Esc)=>{
                msg = ExitKey;
            }
            AltChar('s') => {
                msg = SaveKey;
            }
            Event::Char(ch) =>{
                match ch{
                    '1'..='9' =>{
                        msg = Movement(map_key_to_move(ch).0, map_key_to_move(ch).1);
                    }
                    _ =>{}
                }
            }
            _ => {
                evt = EventResult::Ignored;
            }
        }
        let exit = self.ctrl.lock().unwrap().interpret(msg);
        if exit{
            evt = EventResult::with_cb(|s| s.quit());
        }
        // println!("Handled an event");
        evt
    }
}

impl Terminal{
    pub fn new() -> Self{
        Terminal{
            game: Arc::new(RwLock::new(GameState::new())),
            ctrl: Arc::new(Mutex::new(Controller::new())),
        }
    }
}