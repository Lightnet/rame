/*
    Proejct Name: RAME
    Rust
    Amethyst
    Module
    Engine

    Information:
        Work in progress prototype build.

        working on the module setup.
        
*/

// #[allow(clippy::type_complexity, dead_code)]
//#[allow(dead_code)]
extern crate amethyst;
//#[allow(unused_imports)]
use amethyst::{
    assets::HotReloadBundle,
    //input::is_key_down,
    prelude::*,
    utils::application_root_dir,
    core::transform::TransformBundle,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ecs::{System, SystemData, World, Dispatcher, DispatcherBuilder, WorldExt},
    error::Error,
    //winit::VirtualKeyCode,
};
use amethyst::input::{InputBundle, StringBindings};
use amethyst::ui::{RenderUi, UiBundle};
use amethyst::core::ArcThreadPool;
//use amethyst::shred::DispatcherBuilder;
//use amethyst::ecs::prelude::Dispatcher;
//use amethyst::Error;
use amethyst::DataDispose;
use amethyst::core::SystemBundle;


mod sandbox;
use crate::sandbox::Sandbox;

mod systems; // Import the module
mod menu;
mod util;

pub struct CustomGameData<'a, 'b> {
    core_dispatcher: Option<Dispatcher<'a, 'b>>,
    running_dispatcher: Option<Dispatcher<'a, 'b>>,
}

impl<'a, 'b> CustomGameData<'a, 'b> {
    /// Update game data
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

    pub fn with_base_bundle<B>(mut self, world: &mut World, bundle: B) -> Result<Self, Error>
    where
        B: SystemBundle<'a, 'b>,
    {
        bundle.build(world, &mut self.core)?;
        Ok(self)
    }

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


fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    // this will be the directory the 'Cargo.toml' is defined in.
    let app_root = application_root_dir()?;
    // our display config is in our configs folder.
    let display_config_path = app_root.join("config/display.ron");

    let binding_path = app_root.join("config").join("bindings.ron");
    let input_bundle = InputBundle::<StringBindings>::new()
    .with_bindings_from_file(binding_path)?;
    // other assets ('*.ron' files, '*.png' textures, '*.ogg' audio files, ui prefab files, ...) are here
    let assets_dir = app_root.join("assets/");

    let mut world = World::new();
    let game_data = GameDataBuilder::default()
    //let game_data = CustomGameDataBuilder::default()
        //.with_bundle(WindowBundle::from_config_path(&display_config_path)?)?
        // Add the transform bundle which handles tracking entity positions
        // a lot of other bundles/systems depend on this (without it being explicitly clear), so it
        // makes sense to add it early on
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(systems::ControllerSystem, "controller_system", &["input_system"])
        // this bundle allows us to 'find' the Buttons and other UI elements later on
        .with_bundle(UiBundle::<StringBindings>::new())?
        // this allows us to reload '*.ron' files during execution
        .with_bundle(HotReloadBundle::default())?
        // without this Bundle, our Program will silently (!) fail when trying to start the 'Game'.
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
            // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
            .with_plugin(
                RenderToWindow::from_config_path(&display_config_path)?
                    .with_clear([0.34, 0.36, 0.52, 1.0]),
            )
            // RenderFlat2D plugin is used to render entities with a `SpriteRender` component.
            .with_plugin(RenderFlat2D::default())
            .with_plugin(RenderUi::default()),
        )?;

    // creating the Application with the assets_dir, the first Screen, and the game_data with it's
    // systems.
    //let mut game = Application::new(assets_dir, ExampleState, game_data)?;
    let mut game = Application::new(assets_dir, Sandbox::default(), game_data)?;
    //log::info!("Starting game!");
    game.run();

    Ok(())
}