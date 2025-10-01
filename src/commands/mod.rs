pub mod camera;
pub mod world;

use crate::game::Game;
use std::collections::VecDeque;
use std::fmt::Debug;

pub trait Command {
    //not efficient, will change that later - TODO
    fn execute(&self, game: &mut Game);
}

pub struct CommandHandle {
    commands: VecDeque<Box<dyn Command + Send + 'static>>,
    queue: VecDeque<Box<dyn Command + Send + 'static>>,
}

impl Default for CommandHandle {
    fn default() -> Self {
        Self::new()
    }
}

impl Debug for CommandHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CommandHandle")
            .field("commands", &self.commands.len())
            .field("queue", &self.queue.len())
            .finish()
    }
}

impl CommandHandle {
    pub fn new() -> Self {
        Self {
            commands: VecDeque::new(),
            queue: VecDeque::new(),
        }
    }
    #[inline]
    pub fn add<C: Command + 'static + Send>(&mut self, command: C) {
        self.queue.push_back(Box::new(command));
    }
    pub fn add_vec<C: Command + 'static + Send>(&mut self, commands: Vec<C>) {
        self.queue.append(
            &mut commands
                .into_iter()
                .map(|command| Box::new(command) as Box<dyn Command + Send + 'static>)
                .collect(),
        );
    }
    //maybe split that later, depending on what I need
    pub(crate) fn update(game: &mut Game) {
        let commands: Vec<_> = game.commands.commands.drain(..).collect();
        commands.iter().for_each(|c| c.execute(game));
        game.commands.commands = game.commands.queue.drain(..).collect();
    }
}
