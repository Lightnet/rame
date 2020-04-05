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
extern crate clap;
extern crate amethyst;

//use clap::{Arg, App, SubCommand};
use clap::{Arg, App};
//use clap::App;

//use std::env;

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
//#[allow(unused_imports)]
use amethyst::ui::{RenderUi, UiBundle};
use amethyst::input::{InputBundle, StringBindings};
//use amethyst::DataDispose;
//use amethyst::core::SystemBundle;
//use amethyst::core::ArcThreadPool;

mod components; // struct components
mod systems; // Import the module
//mod menu;
mod util; // menu remove root
//mod sandbox;
mod customgamedata; // custom game data
//menu and other
//mod mainstate; //work in progress
//mod mainmenustate; //mainmenu
//mod pausestate; // pause menu
//mod settingsstate; //setting menu
//mod loadingstate; // loading assets work in progress
mod networkstate; // setup network work in progress
//mod creditsstate; // credits menu
//mod optionsstate; // options menu

#[allow(unused_imports)]
use crate::components::*;
//#[allow(unused_imports)]
//use crate::sandbox::Sandbox;
#[allow(unused_imports)]
use crate::customgamedata::*;
//#[allow(unused_imports)]
//use crate::pausestate::*;
//#[allow(unused_imports)]
//use crate::loadingstate::*;
#[allow(unused_imports)]
use crate::networkstate::*;

//#[allow(unused_imports)]
//use crate::mainmenustate::*;
//#[allow(unused_imports)]
//use crate::mainstate::*;
//#[allow(unused_imports)]
//use crate::settingsstate::*;

//#[allow(unused_imports)]
//use crate::optionsstate::*;
//#[allow(unused_imports)]
//use crate::creditsstate::*;


fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    // this will be the directory the 'Cargo.toml' is defined in.
    let app_root = application_root_dir()?;
    // our display config is in our configs folder.
    let display_config_path = app_root.join("config\\display.ron");

    let binding_path = app_root.join("config").join("bindings.ron");
    //#[allow(unused_variables)]
    // user input
    let input_bundle = InputBundle::<StringBindings>::new()
    .with_bindings_from_file(binding_path)?;
    // other assets ('*.ron' files, '*.png' textures, '*.ogg' audio files, ui prefab files, ...) are here
    let assets_dir = app_root.join("assets");
    
    //let game_data = GameDataBuilder::default()
    let mut game_data = CustomGameDataBuilder::default()
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
    game_data.test();
    //let args: Vec<String> = env::args().collect();

    
    let matches = App::new("RAME Program")
        .version("1.0")
        .author("Lightnet <>")
        .about("Does awesome things")
        .arg(
            Arg::with_name("server")
                .long("server")
                .help("Sets an optional output file")
                .takes_value(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("client")
                .long("client")
                .help("Sets an optional output file")
                .takes_value(true)
                .multiple(true),
        )
        .get_matches();
        
        
    let mut isnetwork: bool = false;
    let mut isserver: bool = false;
    // You can check the value provided by positional arguments, or option arguments
    if let Some(o) = matches.value_of("server") {
        println!("[server] Value for output: {}", o);
        isnetwork = true;
        isserver = true;
    }

    if let Some(o) = matches.value_of("client") {
        println!("[client] Value for output: {}", o);
        isnetwork = true;
    }

    if isnetwork {
        println!("server network test...");
    }

    // You can check the value provided by positional arguments, or option arguments
    //if let Some(o) = matches.value_of("server") {
        //println!("Value for server: {}", o);
    //}

    //if true{
        //println!("server");
    //}
    
    let mut game = Application::new(assets_dir, Networking::default(), game_data)?;
    //log::info!("Starting game!");
    game.run();
    
    Ok(())
}