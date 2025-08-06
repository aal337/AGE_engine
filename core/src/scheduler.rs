use super::game_fn::GameFn;
use super::events::EventHandle;
use super::info::GameInfo;
use std::collections::{HashMap, VecDeque};
use anymap::AnyMap;

#[derive(Default)]
pub(crate) struct Scheduler {
    once: VecDeque<Box<dyn GameFn>>,
    update: VecDeque<Box<dyn GameFn>>,
}

impl Scheduler {
    pub(crate) fn new_from(once: VecDeque<Box<dyn GameFn>>, update: VecDeque<Box<dyn GameFn>>) -> Self {
        Self { once, update }
    }
    pub(crate) fn new_empty() -> Self {
        Default::default()
    }
    pub(crate) fn add_once(
        &mut self,
        once: impl GameFn,
    ) {
        self.once.push_back(Box::new(once));
    }
    pub(crate) fn add_update(
        &mut self,
        once: impl GameFn,
    ) {
        self.update.push_back(Box::new(once));
    }
    pub(crate) fn setup(
        &mut self,
        resources: &mut AnyMap,
        event_handle: &mut EventHandle,
        info: &GameInfo,
    ) {
        for f in self.once.iter_mut() {
            f.exec(resources, event_handle, info);
        }
    }
    pub(crate) fn update(
        &mut self,
        resources: &mut AnyMap,
        event_handle: &mut EventHandle,
        info: &GameInfo,
    ) {
        for f in self.update.iter_mut() {
            f.exec(resources, event_handle, info);
        }
    }
}
