mod data;
mod scheduler;
mod events;

use wgpu::naga::FastHashMap;
use crate::general::data::GameData;
use crate::general::scheduler::Scheduler;

pub struct Game {
    //FastHashMap here since DoS ins't a problem here
    resources: FastHashMap<&'static str, GameData>,
    scheduler: Scheduler,
    //?
}
