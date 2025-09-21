use crate::custom_events::builtin::ExitEvent;
use crate::events::EventHandle;
use crate::scheduler::Scheduler;
use anymap::AnyMap;
use graphics_core::state::State;
use std::convert::identity;
use std::path::Path;

pub struct Game {
    pub(crate) resources: AnyMap,
    pub(crate) scheduler: Scheduler,
    pub(crate) events: EventHandle,
    //only existent while setup
    pub(crate) graphics_state_config: Option<StateConfig>,
    pub(crate) graphics_state: Option<State>,
    //make it customisable later
    pub(crate) window_attributes: WindowAttributes,
    //TODO!: commands scheduler
    pub(crate) commands: Commands,
    pub(crate) world: World,
    //?
}

impl<'a> Default for Game {
    fn default() -> Self {
        Self {
            resources: AnyMap::new(),
            scheduler: Scheduler::new_empty(),
            events: EventHandle::new(),
            graphics_state: None,
            window_attributes: WindowAttributes::default(),
            commands: Commands::new(),
            graphics_state_config: Some(StateConfig::default()),
            world: World::new(),
        }
    }
}

impl Game {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_once(
        mut self,
        once: impl FnMut(&mut Commands, &mut AnyMap, &mut EventHandle) + 'static,
    ) -> Self {
        self.scheduler.add_once(Box::new(once));
        self
    }

    pub fn add_update(
        mut self,
        update: impl FnMut(&mut Commands, &mut AnyMap, &mut EventHandle) + 'static,
    ) -> Self {
        self.scheduler.add_update(Box::new(update));
        self
    }

    #[inline]
    pub fn prepare(&mut self) {}

    fn exit(&mut self, event_loop: ActiveEventLoop) {}

    #[inline]
    pub fn run(mut self) {
        env_logger::init();
        warn!("Starting event loop");
        //TODO!: nicer error handling -> (inline) function
        let event_loop = EventLoop::with_user_event().build();
        if let Err(e) = event_loop {
            let message = format!("Fatal error while building the event loop: {}", e);
            error!("{}", message);
            panic!("{}", message);
        }
        println!("Preparations finished");
        if let Err(e) = event_loop
            .expect("Would have panicked if there was an error with the event loop")
            .run_app(&mut self)
        {
            let message = format!("Fatal error while running the event loop: {}", e);
            error!("{}", message);
            panic!("{}", message);
        }
    }
}

//graphics state modifications
impl Game {
    ///Sets the clear color to the provided rgba color which is drawn to the whole screen before every render.
    /// Can be used as a provisoric one-color background
    pub fn with_color(mut self, r: f64, g: f64, b: f64, a: f64) -> Self {
        let config = self
            .graphics_state_config
            .as_mut()
            .expect("Should be Some(_) before running");
        config.color = wgpu::Color { r, g, b, a };
        self
    }

    pub fn with_model<P: Into<String>>(self, name: &'static str, path: P) -> Self {
        fn inner(mut game: Game, name: &'static str, path: String) -> Game {
            let config = game
                .graphics_state_config
                .as_mut()
                .expect("Should be Some(_) before running");
            config.models.insert(name, path);
            game
        }
        //as_ref() cosmetic here, but who cares...
        inner(self, name, path.into())
    }

    pub fn with_image<P: AsRef<Path>>(self, path: P, name: &'static str) -> Self {
        todo!();
        /*fn inner(mut game: Game, path: &Path, name: &'static str) -> Game {
            let config = game
                .graphics_state_config
                .as_mut()
                .expect("Should be Some(_) before running");
            config.assets.insert(name, Asset::Image(path.to_owned()));
            game
        }
        inner(self, path.as_ref(), name)*/
    }
}

//internal helper functions
impl Game {
    #[inline]
    fn propagate_event<T: 'static>(&mut self, event: T) {
        self.events.add(event);
    }
}

use log::{error, warn};
use std::sync::Arc;
//use crate::custom_events::input::{SimpleKeyEvent, SimpleMouseKeyEvent};
use crate::aliases::Commands;
use crate::world::{ModelState, World};
use graphics_core::config::{Asset, StateConfig};
use winit::window::WindowAttributes;
use winit::{
    application::ApplicationHandler,
    event::*,
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

impl ApplicationHandler<Game> for Game {
    /*fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: StartCause) {
        todo!()
    }*/

    //TODO!: make sure initialisation only happens once, resume() doesn't guarantee that!!
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(self.window_attributes.clone())
                .expect(
                    "Winit needs to be able to create a window, otherwise everything won't work",
                ),
        );

        self.graphics_state = Some(
            pollster::block_on(State::new(
                window,
                self.graphics_state_config
                    .take()
                    .expect("Set to Some(_) before running the game"),
            ))
            //TODO: proper exception handling
            .expect("State creation needs to work"),
        );
        //once logic

        self.scheduler
            .setup(&mut self.commands, &mut self.resources, &mut self.events);

        self.events.update();
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: Game) {
        *self = event;
    }
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        self.scheduler
            .update(&mut self.commands, &mut self.resources, &mut self.events);
        self.events.update();

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            //I should refactor that later - TODO
            WindowEvent::Resized(size) => {
                if let Some(graphics_state) = &mut self.graphics_state {
                    graphics_state.resize(size.width, size.height)
                };
            }
            WindowEvent::RedrawRequested => {
                //TODO: check if this is actually necessary later
                if self.graphics_state.is_none() {
                    return;
                }

                self.graphics_state.as_mut().expect("GENERIC").update();

                match self.graphics_state.as_mut().expect("fgfdsyxcfr").render(
                    self.world.to_be_rendered.iter().filter_map(|id| {
                        //TODO: .expect messages
                        //TODO: more efficient borrowing instead of only .as_ref / .as_mut
                        match self.world.entities.get(id).expect("").model {
                            ModelState::Visible(model_id) => Some(model_id),
                            _ => None,
                        }
                    }),
                ) {
                    Ok(_) => {}
                    // Reconfigure the surface if it's lost or outdated
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        let size = self.graphics_state.as_ref().expect("").window.inner_size();
                        self.graphics_state
                            .as_mut()
                            .expect("")
                            .resize(size.width, size.height);
                    }
                    Err(e) => {
                        log::error!("Unable to render {}", e);
                    }
                }
            }
            event => self.propagate_event(event),
        };
    }
    fn memory_warning(&mut self, event_loop: &ActiveEventLoop) {
        event_loop.exit();
        panic!("Received a memory warning, currently a fatal error");
        todo!();
    }
}
