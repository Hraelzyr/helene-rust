use std::cmp::min;
use std::sync::{Arc, Mutex};
use cursive::{Printer, View};
use cursive::event::{Event, EventResult};
use cursive::event::Event::Key;
use signals2::Connect1;
use crate::actor::Movable;


use crate::controller::{Context, Controller};
use crate::controller::Message::{Movement, SaveKey, ExitKey, LoadKey, NewKey};
use crate::game_state::{GameState};

pub(crate) struct Terminal {
    game: Arc<Mutex<GameState>>,
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
        let cnt = self.ctrl.lock().unwrap();
        let cntx = cnt.active_context();
        match cntx {
            Context::MainMenu => {
                printer.print((0,0), "Welcome to Helene!\n \
                Press Ctrl-L to load an existing game,\
                Press Ctrl-S/Esc to exit, \
                or Ctrl-N to create a new game!");
            }
            Context::Pause => {printer.print((0,0), "\
                Press Esc to return, \
                or Ctrl-S to exit");}
            Context::Player | Context::Map => {
                let msg = self.game.lock().unwrap().send_message();
                let (map, wd, ht) = msg.map;
                let pl = msg.player;
                for i in 0..ht{
                    let start=wd as usize*i as usize;
                    let end = min(start+wd as usize, map.len());
                    printer.print((0, i), &map[start..end]);
                }
                printer.print(pl.get_loc(), "@");
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
            Event::CtrlChar('s') => {
                msg = SaveKey;
            }
            Event::CtrlChar('l') => {
                msg=LoadKey;
            }
            Event::CtrlChar('n') => {
                msg=NewKey;
            }
            Event::Char(ch) =>{
                match ch{
                    '1'..='9' =>{
                        msg = Movement(map_key_to_move(ch).0, map_key_to_move(ch).1);
                    }
                    _ =>{println!("Key unrecognised");}
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
        let a = Terminal{
            game: Arc::new(Mutex::new(GameState::new())),
            ctrl: Arc::new(Mutex::new(Controller::new())),
        };
        let game = Arc::clone(&a.game);
        let ctrl = Arc::clone(&a.ctrl);
        ctrl.lock().unwrap().bump_move.connect(move |(x, y)| game.lock().unwrap().player.mov(x, y));
        a
    }
}