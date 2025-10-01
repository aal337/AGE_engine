use super::events::EventHandle;
use crate::aliases::{BoxedGameFn, Commands};
use crate::commands::CommandHandle;
use anymap::{AnyMap, Map};
use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;

#[derive(Default)]
pub struct Scheduler {
    once: VecDeque<BoxedGameFn>,
    update: VecDeque<BoxedGameFn>,
}

impl Debug for Scheduler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CommandHandle")
            .field("once", &self.once.len())
            .field("update", &self.update.len())
            .finish()
    }
}

impl Scheduler {
    pub fn new_from(once: VecDeque<BoxedGameFn>, update: VecDeque<BoxedGameFn>) -> Self {
        Self { once, update }
    }
    pub fn new_empty() -> Self {
        Default::default()
    }
    pub fn add_once(&mut self, once: BoxedGameFn) {
        self.once.push_back(once);
    }
    pub fn add_update(&mut self, update: BoxedGameFn) {
        self.update.push_back(update);
    }
    pub fn setup(
        &mut self,
        commands: &mut Commands,
        resources: &mut Map<dyn anymap::any::Any + Send>,
        event_handle: &mut EventHandle,
    ) {
        //println!("Once len: {}", self.once.len());
        for f in self.once.iter_mut() {
            f(commands, resources, event_handle);
        }
    }
    pub fn update(
        &mut self,
        commands: &mut Commands,
        resources: &mut Map<dyn anymap::any::Any + Send>,
        event_handle: &mut EventHandle,
    ) {
        for f in self.update.iter_mut() {
            f(commands, resources, event_handle);
        }
    }
}
