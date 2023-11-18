use crate::actor::Actor;

pub(crate) enum Action{
    Move,
    Attack,
}

trait Scheduler {
    fn play(delay: u8) -> ();
    fn add(playlist: Vec<Actor>) ->();
}

