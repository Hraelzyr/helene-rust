use cursive::event::Event::{AltChar, Char, Key};
use cursive::event::Key::{Esc};
use cursive::{Printer, View};
use cursive::event::{Callback, Event, EventResult};

pub struct TestKR{
    evt: Event,
}

impl TestKR{
    pub fn new() -> Self{
        TestKR{
            evt: Event::WindowResize,
        }
    }
}

impl View for TestKR{
    fn draw(&self, printer: &Printer) {
        match self.evt {
            AltChar('s') | Key(Esc) | Event::Exit =>{
                printer.print((0,0),"Should have quit...");
            }
            Char(c) | Event::CtrlChar(c) | AltChar(c)=> {
                let mut s=String::from("You entered ");
                s.push(c);
                printer.print((0,0), s.as_str());
            }
            _ => {
                printer.print((0,0),"Unrecognised input");
            }
        }
    }

    fn on_event(&mut self, mrn: Event) -> EventResult {
        match mrn{
            Event::Exit | Key(Esc) | AltChar('s')=>{
                EventResult::Consumed(Some(Callback::from_fn(|s| s.quit())))
            }
            _ => {
                self.evt=mrn;
                EventResult::Consumed(None)
            }
        }
    }

}