mod data;
mod scheduler;
pub mod events;
mod info;

use std::collections::VecDeque;
use std::collections::HashMap;
use crate::general::aliases::{Resources};
pub use crate::general::data::GameData;
use crate::general::events::{Event, EventHandle, EventQueueMode};
use crate::general::scheduler::Scheduler;

pub mod aliases {
    use std::collections::HashMap;
    use crate::general::events::EventHandle;
    use crate::general::GameData;

    pub type SetupFn = Box<dyn FnMut(&mut Resources, &mut EventHandle) + 'static>;
    pub type UpdateFn = Box<dyn FnMut(&mut Resources, &mut EventHandle) + 'static>;
    pub type Resources=HashMap<&'static str, GameData>;
}

pub struct Game {
    //HashMap here since DoS ins't a problem here
    resources: HashMap<&'static str, GameData>,
    scheduler: Scheduler,
    events: EventHandle,
    //?
}

impl Default for Game {
    fn default() -> Self {
        Self {
            resources: HashMap::default(),
            scheduler: Scheduler::new_empty(),
            //make the mode customisable!!
            events: EventHandle::new(EventQueueMode::StoreOnce)
        }
    }
}

impl Game {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_once(mut self, once: impl FnMut(&mut Resources, &mut EventHandle) + 'static) -> Self{
        self.scheduler.add_once(once);
        self
    }

    pub fn add_update(mut self, once: impl FnMut(&mut Resources, &mut EventHandle) + 'static) -> Self {
        self.scheduler.add_update(once);
        self
    }

    pub fn run(mut self) {
        self.scheduler.setup(&mut self.resources, &mut self.events);
        self.events.setup();

        //at the moment just a loop, so std::process::exit is OK
        loop {
            self.scheduler.update(&mut self.resources, &mut self.events);
            self.events.update();
        }
    }
}

//macro later?
