use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;

pub struct EventHandle {
    events: VecDeque<Box<dyn Event>>,
    queue: VecDeque<Box<dyn Event>>,
    mode: EventQueueMode,
}

impl EventHandle {
    pub(crate) fn new(mode: EventQueueMode) -> Self {
        Self {
            events: VecDeque::new(),
            queue: VecDeque::new(),
            mode,
        }
    }

    //could have them as one, maybe going to do that, idk
    pub(crate) fn setup(&mut self) {
        while let Some(event) = self.queue.pop_front() {
            self.events.push_back(event);
        }
    }

    pub(crate) fn update(&mut self) {
        if self.mode == EventQueueMode::StoreOnce {
            self.events.drain(0..self.events.len());
        }
        while let Some(event) = self.queue.pop_front() {
            self.events.push_back(event);
        }
    }
    //it's guranteed to return atleast 1 Event if it returns Some(_)
    pub fn read(&mut self) -> Option<Vec<Box<dyn Event>>> {
        if self.events.is_empty() {
            return None;
        }
        Some(self.events.drain(..).collect::<Vec<Box<dyn Event>>>())
    }

    //maybe I'll obey the linter, we'll see
    pub fn contains(&mut self, target_event: &Box<dyn Event>) -> bool {
        let mut result = false;
        let mut indexes = Vec::new();
        for (index, event) in self.events.iter_mut().enumerate() {
            if event.id() == target_event.id() {
                indexes.push(index);
                result = true;
            }
        }
        indexes.iter().for_each(|index| {
            self.events.remove(*index);
        });
        result
    }
}

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub enum EventQueueMode {
    //if this mode is activated, the developer is responsible for consuming the events
    //so they don't accumulate until an overflow happens
    StoreUntilConsume,
    //default later for quickstart
    #[default]
    StoreOnce,
}

//proc macro for deriving the event trait later maybe

pub trait Event: Send + Sync {
    //just an unique id, maybe the typename later??
    fn id(&self) -> &'static str;
}

pub mod special_events {
    use super::Event;
    struct ExitEvent;
    impl Event for ExitEvent {
        fn id(&self) -> &'static str {
            "__exit"
        }
    }
    //no constant sadly
    pub fn exit_event() -> Box<dyn Event> {
        Box::new(ExitEvent)
    }
}
