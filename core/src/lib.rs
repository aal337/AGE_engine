mod data;
pub mod events;
mod game_fn;
mod info;
mod scheduler;

pub use crate::data::GameData;
use crate::events::{EventHandle, EventQueueMode};
use crate::info::GameInfo;
use crate::scheduler::Scheduler;
//btw, I didn't choose the AnyMap because of Bevy but because it is just extremely convenient :-)
use anymap::AnyMap;
use crate::events::special_events::exit_event;
use graphics_core::state::State;
use crate::game_fn::GameFn;

pub mod aliases {
    use super::events::EventHandle;
    use super::info::GameInfo;
    use anymap::AnyMap;

    pub type _MaxParamGameFn = Box<dyn FnMut(&mut AnyMap, &mut EventHandle, &GameInfo) + 'static>;
}

pub struct Game {
    resources: AnyMap,
    scheduler: Scheduler,
    events: EventHandle,
    info: GameInfo,
    graphics_state: Option<State>,
    //?
}

impl Default for Game {
    fn default() -> Self {
        Self {
            resources: AnyMap::new(),
            scheduler: Scheduler::new_empty(),
            //make the mode customisable later!!
            events: EventHandle::new(EventQueueMode::StoreOnce),
            info: GameInfo::new(),
            graphics_state: None,
        }
    }
}

impl Game {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_once(
        mut self,
        once: impl GameFn,
    ) -> Self {
        self.scheduler.add_once(once);
        self
    }

    pub fn add_update(
        mut self,
        once: impl GameFn,
    ) -> Self {
        self.scheduler.add_update(once);
        self
    }

    pub fn run(mut self) {
        self.scheduler
            .setup(&mut self.resources, &mut self.events, &self.info);
        self.events.setup();

        while !self.events.contains(&exit_event()) {
            self.scheduler
                .update(&mut self.resources, &mut self.events, &self.info);
            self.events.update();
        }
    }
}
