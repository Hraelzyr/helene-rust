use signals2::{Emit0, Emit1, Signal};
use crate::controller::Context::{MainMenu, Pause, Player};

pub(crate) enum Context{
    MainMenu,
    Pause,
    Player,
    Map,
}

#[derive(Copy, Clone)]
pub(crate) enum Message {
    Movement(i8, i8),
    ExitKey,
    SaveKey,
    LoadKey,
    NewKey,
}

pub(crate) struct Controller {
    context: Vec<Context>,

    bump_move: Signal<((i8, i8),)>,

    map_move: Signal<((i8, i8),)>,

    load_signal: Signal<()>,
    new_signal: Signal<()>,
    save_signal: Signal<()>,

}

impl Controller{
    pub(crate) fn new() -> Self{
        Controller{
            context: vec![MainMenu],
            bump_move: Default::default(),
            map_move: Default::default(),
            load_signal: Default::default(),
            new_signal: Default::default(),
            save_signal: Default::default(),
        }
    }
    pub(crate) fn interpret(&mut self, msg: Message) -> bool{
        let mut exit=false;
        match self.context.last().expect("Empty context stack") {
            MainMenu =>{
                match msg {
                    Message::LoadKey => {
                        self.load_signal.emit();
                        self.context.push(Player);
                    }

                    Message::NewKey =>{
                        self.new_signal.emit();
                        self.context.push(Player);
                    }

                    Message::ExitKey | Message::SaveKey =>{
                        self.save_signal.emit();
                        exit=true;
                    }
                    _ => {}
                }
            }
            Pause => {
                match msg {
                    Message::ExitKey => {
                        self.context.pop();
                    }
                    Message::SaveKey => {
                        self.save_signal.emit();
                        exit=true;
                    }
                    _ => {}
                }
            }
            Player => {
                match msg {
                    Message::Movement(x, y) => {
                        self.bump_move.emit((x,y));
                    }
                    Message::ExitKey => {
                        self.context.push(Pause);
                    }
                    _ => {}
                }
            }
            Context::Map => {
                match msg {
                    Message::Movement(x, y) => {
                        self.map_move.emit((x,y));
                    }
                    Message::ExitKey => {
                        self.context.pop();
                    }
                    _ => {}
                }
            }
        }
        exit
    }
}