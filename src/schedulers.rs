use crate::actor::Actor;

//TODO: schedule actions from all actors round by round on a person action
//TODO: monster AI
//TODO: check sanity of human action
trait Scheduler {
    fn play(&self, delay: u8) -> ();
    fn add_actor(&mut self, playlist: Actor) ->();
    fn add_actors(&mut self, playlist: Vec<Actor>) ->();
}

pub(crate) struct Sequencer{
    actors: Vec<Actor>
}

impl Scheduler for Sequencer{
    fn play(&self, delay: u8) -> () {
        todo!()
    }

    fn add_actor(&mut self, playlist: Actor) -> () {
        self.actors.push(playlist);
    }

    fn add_actors(&mut self, playlist: Vec<Actor>) -> () {
        self.actors.extend(playlist);
    }

}