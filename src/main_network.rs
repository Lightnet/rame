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
use std::time::Duration;

#[allow(unused_imports)]
use amethyst::{
    assets::HotReloadBundle,
    input::{is_close_requested, is_key_down},
    prelude::*,
    utils::application_root_dir,
    core::transform::TransformBundle,
    core::{bundle::SystemBundle, frame_limiter::FrameRateLimitStrategy, SystemDesc, Time},
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ecs::{System, SystemData, World, Dispatcher, DispatcherBuilder, WorldExt, Read, Write},
    error::Error,
    winit::VirtualKeyCode,
    DataDispose, DataInit,
    network::simulation::{tcp::TcpNetworkBundle, NetworkSimulationEvent, NetworkSimulationTime, TransportResource},
    shrev::{EventChannel, ReaderId},
    Result,
};
use log::{error, info};
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


use std::net::TcpListener;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

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
    
    if let Some(o) = matches.value_of("server") {

        println!("[server] Value for output: {}", o);
        //isnetwork = true;
        //isserver = true;

        let listener = TcpListener::bind("0.0.0.0:3457")?;
        listener.set_nonblocking(true)?;

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
            )
            .with_base_bundle(TcpNetworkBundle::new(Some(listener), 2048))
            .with_base_bundle(ServerReceiveBundle);

        let mut game = Application::new(assets_dir, Networking::default(), game_data)?;
        //log::info!("Starting game!");
        println!("server network...");
        game.run();
             
    }else if let Some(o) = matches.value_of("client") {

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
            )
            .with_base_bundle(TcpNetworkBundle::new(None, 2048))
            .with_base_bundle(ClientBundle);
            let mut game = Application::new(assets_dir, Networking::default(), game_data)?;
            //log::info!("Starting game!");
            println!("client network...");
            game.run();
        
    }else{
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

        println!("no network...");
        let mut game = Application::new(assets_dir, Networking::default(), game_data)?;
        //log::info!("Starting game!");
        game.run();
    }
 
    //let mut game = Application::new(assets_dir, Networking::default(), game_data)?;
    //log::info!("Starting game!");
    //game.run();
    
    Ok(())
}

//===============================================
// server start
//===============================================
#[derive(Debug)]
struct ServerReceiveBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for ServerReceiveBundle {
    fn build(self, world: &mut World, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        println!("server init...");
        builder.add(
            ServerReceiveSystemDesc::default().build(world),
            "receiving_system",
            &[],
        );
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ServerReceiveSystemDesc;

impl<'a, 'b> SystemDesc<'a, 'b, ServerReceiveSystem> for ServerReceiveSystemDesc {
    fn build(self, world: &mut World) -> ServerReceiveSystem {
        // Creates the EventChannel<NetworkEvent> managed by the ECS.
        <ServerReceiveSystem as System<'_>>::SystemData::setup(world);
        // Fetch the change we just created and call `register_reader` to get a
        // ReaderId<NetworkEvent>. This reader id is used to fetch new events from the network event
        // channel.
        let reader = world
            .fetch_mut::<EventChannel<NetworkSimulationEvent>>()
            .register_reader();
            ServerReceiveSystem::new(reader)
    }
}

/// A simple system that receives a ton of network events.
struct ServerReceiveSystem {
    reader: ReaderId<NetworkSimulationEvent>,
}

impl ServerReceiveSystem {
    pub fn new(reader: ReaderId<NetworkSimulationEvent>) -> Self {
        Self { reader }
    }
}

impl<'a> System<'a> for ServerReceiveSystem {
    type SystemData = (
        Write<'a, TransportResource>,
        Read<'a, EventChannel<NetworkSimulationEvent>>,
    );

    fn run(&mut self, (mut net, channel): Self::SystemData) {
        for event in channel.read(&mut self.reader) {
            match event {
                NetworkSimulationEvent::Message(addr, payload) => {
                    info!("{}: {:?}", addr, payload);
                    // In a typical client/server simulation, both the client and the server will
                    // be exchanging messages at a constant rate. Laminar makes use of this by
                    // packaging message acks with the next sent message. Therefore, in order for
                    // reliability to work properly, we'll send a generic "ok" response.
                    net.send(*addr, b"ok");
                }
                NetworkSimulationEvent::Connect(addr) => info!("New client connection: {}", addr),
                NetworkSimulationEvent::Disconnect(addr) => {
                    info!("Client Disconnected: {}", addr);
                }
                NetworkSimulationEvent::RecvError(e) => {
                    error!("Recv Error: {:?}", e);
                }
                _ => {}
            }
        }
    }
}
//===============================================
// server end
//===============================================



#[derive(Debug)]
struct ClientBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for ClientBundle {
    fn build(self, world: &mut World, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        builder.add(ClientSystemDesc::default().build(world), "spam_system", &[]);
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ClientSystemDesc;

impl<'a, 'b> SystemDesc<'a, 'b, ClientSystem> for ClientSystemDesc {
    fn build(self, world: &mut World) -> ClientSystem {
        // Creates the EventChannel<NetworkEvent> managed by the ECS.
        <ClientSystem as System<'_>>::SystemData::setup(world);
        // Fetch the change we just created and call `register_reader` to get a
        // ReaderId<NetworkEvent>. This reader id is used to fetch new events from the network event
        // channel.
        let reader = world
            .fetch_mut::<EventChannel<NetworkSimulationEvent>>()
            .register_reader();

            ClientSystem::new(reader)
    }
}

/// A simple system that receives a ton of network events.
struct ClientSystem {
    reader: ReaderId<NetworkSimulationEvent>,
}

impl ClientSystem {
    pub fn new(reader: ReaderId<NetworkSimulationEvent>) -> Self {
        Self { reader }
    }
}

impl<'a> System<'a> for ClientSystem {
    type SystemData = (
        Read<'a, NetworkSimulationTime>,
        Read<'a, Time>,
        Write<'a, TransportResource>,
        Read<'a, EventChannel<NetworkSimulationEvent>>,
    );
    fn run(&mut self, (sim_time, time, mut net, event /*, tx*/): Self::SystemData) {
        let server_addr = "127.0.0.1:3457".parse().unwrap();
        for frame in sim_time.sim_frames_to_run() {
            info!("Sending message for sim frame {}.", frame);
            let payload = format!(
                "CL: sim_frame:{},abs_time:{}",
                frame,
                time.absolute_time_seconds()
            );
            net.send(server_addr, payload.as_bytes());
        }

        for event in event.read(&mut self.reader) {
            match event {
                NetworkSimulationEvent::Message(_addr, payload) => info!("Payload: {:?}", payload),
                NetworkSimulationEvent::Connect(addr) => info!("New client connection: {}", addr),
                NetworkSimulationEvent::Disconnect(addr) => info!("Server Disconnected: {}", addr),
                NetworkSimulationEvent::RecvError(e) => {
                    error!("Recv Error: {:?}", e);
                }
                NetworkSimulationEvent::SendError(e, msg) => {
                    error!("Send Error: {:?}, {:?}", e, msg);
                }
                _ => {}
            }
        }
    }
}















