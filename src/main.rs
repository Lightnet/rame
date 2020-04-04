/*
    Proejct Name: Rame
    Rust
    Amethyst
    Module
    Engine

    Created by: Lightnet

    License: MIT

    Information:
        Work in progress prototype build.

        working on the module setup.
        
*/
//#[allow(unused_imports)]
// #[allow(clippy::type_complexity, dead_code)]
//#[allow(dead_code)]
extern crate amethyst;

#[allow(unused_imports)]
use amethyst::{
    assets::HotReloadBundle,
    input::{is_close_requested, is_key_down},
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
    winit::VirtualKeyCode,
    DataDispose, DataInit,
};
//use amethyst::DataDispose;
//use amethyst::core::SystemBundle;
//use amethyst::core::ArcThreadPool;
use amethyst::input::{InputBundle, StringBindings};
#[allow(unused_imports)]
use amethyst::ui::{RenderUi, UiBundle};

mod sandbox;
mod customgamedata;

#[allow(unused_imports)]
use crate::sandbox::Sandbox;
#[allow(unused_imports)]
use crate::customgamedata::*;

mod pausestate;
//use crate::pausestate::*;

mod mainstate;
use crate::mainstate::*;

mod systems; // Import the module
mod menu;
mod util;

/*
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
        println!("pause");
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
        #[allow(unused_variables)]
        event: StateEvent,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        /*
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
        */
        Trans::None
    }

    fn update(&mut self, data: StateData<CustomGameData>) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        data.data.update(&data.world, true); // true to say we should dispatch running
        Trans::None
    }
}
*/

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    // this will be the directory the 'Cargo.toml' is defined in.
    let app_root = application_root_dir()?;
    // our display config is in our configs folder.
    let display_config_path = app_root.join("config/display.ron");

    let binding_path = app_root.join("config").join("bindings.ron");
    #[allow(unused_variables)]
    let input_bundle = InputBundle::<StringBindings>::new()
    .with_bindings_from_file(binding_path)?;
    // other assets ('*.ron' files, '*.png' textures, '*.ogg' audio files, ui prefab files, ...) are here
    let assets_dir = app_root.join("assets/");


    //let mut world = World::new();
    //let game_data = GameDataBuilder::default();
    //let mut game = Application::new(assets_dir, Sandbox::default(), game_data)?;
    //game.run();
    
    //let game_data = GameDataBuilder::default()
    let game_data = CustomGameDataBuilder::default()
        //.with_bundle(WindowBundle::from_config_path(&display_config_path)?)?
        // Add the transform bundle which handles tracking entity positions
        // a lot of other bundles/systems depend on this (without it being explicitly clear), so it
        // makes sense to add it early on
        .with_base_bundle(TransformBundle::new())
        .with_base_bundle(input_bundle)
        .with_base(systems::ControllerSystem, "controller_system", &["input_system"])
        // this bundle allows us to 'find' the Buttons and other UI elements later on
        .with_base_bundle(UiBundle::<StringBindings>::new())
        // this allows us to reload '*.ron' files during execution
        .with_base_bundle(HotReloadBundle::default())
        // without this Bundle, our Program will silently (!) fail when trying to start the 'Game'.
        .with_base_bundle(
            RenderingBundle::<DefaultBackend>::new()
            // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
            .with_plugin(
                RenderToWindow::from_config_path(&display_config_path)?
                    .with_clear([0.34, 0.36, 0.52, 1.0]),
            )
            // RenderFlat2D plugin is used to render entities with a `SpriteRender` component.
            .with_plugin(RenderFlat2D::default())
            .with_plugin(RenderUi::default()),
        );

    // creating the Application with the assets_dir, the first Screen, and the game_data with it's
    // systems.
    //let mut game = Application::new(assets_dir, ExampleState, game_data)?;
    //let mut game = Application::new(assets_dir, Sandbox::default(), game_data)?;
    let mut game = Application::new(assets_dir, MainState::default(), game_data)?;
    //log::info!("Starting game!");
    game.run();
    

    Ok(())
}