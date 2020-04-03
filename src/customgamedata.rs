#[allow(unused_imports)]
use amethyst::{
    //assets::HotReloadBundle,
    input::{is_close_requested, is_key_down},
    prelude::*,
    //utils::application_root_dir,
    //core::transform::TransformBundle,
    //renderer::{
        //plugins::{RenderFlat2D, RenderToWindow},
        //types::DefaultBackend,
        //RenderingBundle,
    //},
    ecs::{System, SystemData, World, Dispatcher, DispatcherBuilder, WorldExt},
    error::Error,
    winit::VirtualKeyCode,
};
use amethyst::DataDispose;
use amethyst::core::SystemBundle;
use amethyst::core::ArcThreadPool;

pub struct CustomGameData<'a, 'b> {
    core_dispatcher: Option<Dispatcher<'a, 'b>>,
    running_dispatcher: Option<Dispatcher<'a, 'b>>,
}

impl<'a, 'b> CustomGameData<'a, 'b> {
    /// Update game data
    #[allow(dead_code)]
    pub fn update(&mut self, world: &World, running: bool) {
        if running {
            if let Some(dispatcher) = self.running_dispatcher.as_mut() {
                dispatcher.dispatch(&world);
            }
        }
        if let Some(dispatcher) = self.core_dispatcher.as_mut() {
            dispatcher.dispatch(&world);
        }
    }
}


pub struct CustomGameDataBuilder<'a, 'b> {
    pub core: DispatcherBuilder<'a, 'b>,
    pub running: DispatcherBuilder<'a, 'b>,
}

impl<'a, 'b> Default for CustomGameDataBuilder<'a, 'b> {
    fn default() -> Self {
        CustomGameDataBuilder::new()
    }
}

impl<'a, 'b> CustomGameDataBuilder<'a, 'b> {
    pub fn new() -> Self {
        CustomGameDataBuilder {
            core: DispatcherBuilder::new(),
            running: DispatcherBuilder::new(),
        }
    }
    #[allow(dead_code)]
    pub fn with_base_bundle<B>(mut self, world: &mut World, bundle: B) -> Result<Self, Error>
    where
        B: SystemBundle<'a, 'b>,
    {
        bundle.build(world, &mut self.core)?;
        Ok(self)
    }

    #[allow(dead_code)]
    pub fn with_running<S>(mut self, system: S, name: &str, dependencies: &[&str]) -> Self
    where
        for<'c> S: System<'c> + Send + 'a,
    {
        self.running.add(system, name, dependencies);
        self
    }
}

impl<'a, 'b> DataInit<CustomGameData<'a, 'b>> for CustomGameDataBuilder<'a, 'b> {
    fn build(self, world: &mut World) -> CustomGameData<'a, 'b> {
        // Get a handle to the `ThreadPool`.
        let pool = (*world.read_resource::<ArcThreadPool>()).clone();

        let mut core_dispatcher = self.core.with_pool(pool.clone()).build();
        let mut running_dispatcher = self.running.with_pool(pool.clone()).build();
        core_dispatcher.setup(world);
        running_dispatcher.setup(world);

        let core_dispatcher = Some(core_dispatcher);
        let running_dispatcher = Some(running_dispatcher);

        CustomGameData { core_dispatcher, running_dispatcher }
    }
}

impl<'a,'b> DataDispose for CustomGameData<'a,'b> {
    // We dispose each dispatcher owned by the `CustomGameData` structure.
    fn dispose(&mut self, world: &mut World) {
        if let Some(dispatcher) = self.core_dispatcher.take() {
            dispatcher.dispose(world);
        }
        if let Some(dispatcher) = self.running_dispatcher.take() {
            dispatcher.dispose(world);
        }
    }
}

pub struct Main;
pub struct Paused;

impl<'a, 'b> State<CustomGameData<'a, 'b>, StateEvent> for Paused {
    #[allow(dead_code)]
    fn on_start(&mut self, 
        #[allow(unused_variables)]
        data: StateData<CustomGameData>) {
        //create_paused_ui(data.world);
    }

    fn handle_event(
        &mut self,
        #[allow(unused_variables)]
        data: StateData<CustomGameData>,
        event: StateEvent,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                Trans::Quit
            } else if is_key_down(&event, VirtualKeyCode::Space) {
                //delete_paused_ui(data.world);
                Trans::Pop
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
    }

    fn update(&mut self, data: StateData<CustomGameData>) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        data.data.update(&data.world, false); // false to say we should not dispatch running
        Trans::None
    }
}

impl<'a, 'b> State<CustomGameData<'a, 'b>, StateEvent> for Main {

    #[allow(dead_code)]
    fn on_start(&mut self,
        #[allow(unused_variables)]
        data: StateData<CustomGameData>) {
        //initialise(data.world);
    }

    fn handle_event(
        &mut self,
        _: StateData<CustomGameData>,
        event: StateEvent,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                Trans::Quit
            } else if is_key_down(&event, VirtualKeyCode::Space) {
                Trans::Push(Box::new(Paused))
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
    }

    fn update(&mut self, data: StateData<CustomGameData>) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        data.data.update(&data.world, true); // true to say we should dispatch running
        Trans::None
    }
}