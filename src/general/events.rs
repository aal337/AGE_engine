use std::collections::{HashMap,VecDeque};
use std::fmt::Debug;


#[derive(Debug, Clone)]
pub struct EventHandle {
    events: VecDeque<Event>,
    queue: VecDeque<Event>,
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
        if self.mode==EventQueueMode::StoreOnce {
            self.events.drain(0..self.events.len());
        }
        while let Some(event) = self.queue.pop_front() {
            self.events.push_back(event);
        }
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

#[derive(Debug, Clone)]
pub struct Event {
    id: &'static str,
}

impl Event {
    pub fn new(id: &'static str) -> Self {
        Self { id }
    }
}
