#[allow(unused_imports)]
use amethyst::{
    assets::{
        Completion, Handle, Prefab, PrefabLoader, PrefabLoaderSystemDesc, ProgressCounter,
        RonFormat,
    },
    input::{is_close_requested, is_key_down},
    prelude::*,
    utils::application_root_dir,
    core::transform::TransformBundle,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle, UiCreator, UiLoader, UiPrefab},
    ecs::{System, SystemData, World, Dispatcher, DispatcherBuilder, WorldExt},
    ecs::prelude::Entity,
    error::Error,
    winit::VirtualKeyCode,
    DataDispose, DataInit,
};

//mod customgamedata;

use crate::customgamedata::*;
//use crate::pausestate::*;
//use crate::pausestate::Paused;

#[derive(Default)]
pub struct MainMenuState {
    #[allow(dead_code)]
    progress: ProgressCounter,
    #[allow(dead_code)]
    paused_ui: Option<Handle<UiPrefab>>,
    #[allow(dead_code)]
    ui_root: Option<Entity>,
}

impl<'a, 'b> State<CustomGameData<'a, 'b>, StateEvent> for MainMenuState {

    #[allow(dead_code)]
    fn on_start(&mut self,
        //#[allow(unused_variables)]
        data: StateData<CustomGameData>) 
    {
        let world = data.world;

        self.ui_root =
            Some(world.exec(|mut creator: UiCreator<'_>| creator.create("ui/mainmenu.ron", ())));
        
            
        println!("init main...");
        //initialise(data.world);
    }

    fn handle_event(
        &mut self,
        _: StateData<CustomGameData>,
        event: StateEvent,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                println!("quit");
                Trans::Quit
            } else if is_key_down(&event, VirtualKeyCode::Space) {
                println!("Paused");
                //Trans::Push(Box::new(Paused))
                Trans::None
            } else {
                //println!("...");
                Trans::None
            }
        } else {
            Trans::None
        }
        
        //Trans::None
    }

    fn update(&mut self, data: StateData<CustomGameData>) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        data.data.update(&data.world, true); // true to say we should dispatch running
        Trans::None
    }

    fn on_stop(&mut self, 
        //#[allow(dead_code)]
        #[allow(unused_variables)]
        data: StateData<CustomGameData>) {

            
    }

}