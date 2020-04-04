/*
    Project Name: Rame
    Created by: Lightnet
    License: MIT
    Information: work in progress test.

*/

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
    ui::{RenderUi, UiBundle, UiCreator, UiLoader, UiPrefab, UiFinder, UiEvent, UiEventType},
    ecs::{System, SystemData, World, Dispatcher, DispatcherBuilder, WorldExt},
    ecs::prelude::Entity,
    error::Error,
    winit::VirtualKeyCode,
    DataDispose, DataInit,
};


use crate::customgamedata::*;

#[derive(Default)]
pub struct Networking{
    #[allow(dead_code)]
    ui_root: Option<Entity>,
    button_start: Option<Entity>,
}

const BUTTON_START: &str = "start";

impl<'a, 'b> State<CustomGameData<'a, 'b>, StateEvent> for Networking {
    #[allow(dead_code)]
    fn on_start(&mut self, 
        #[allow(unused_variables)]
        data: StateData<CustomGameData>) {
        let world = data.world;
        println!("Networking menu");
        //create_paused_ui(data.world);
        self.ui_root =
            //Some(world.exec(|mut creator: UiCreator<'_>| creator.create("ui/menu.ron", ())));
            Some(world.exec(|mut creator: UiCreator<'_>| creator.create("ui/menunetwork.ron", ())));
    }

    fn handle_event(
        &mut self,
        #[allow(unused_variables)]
        data: StateData<CustomGameData>,
        event: StateEvent,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        
        match event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) {
                    //log::info!("[Trans::Quit] Quitting Application!");
                    Trans::Quit
                } else if is_key_down(&event, VirtualKeyCode::Escape) {
                    //log::info!("[Trans::Switch] Switching back to WelcomeScreen!");
                    //Trans::Switch(Box::new(WelcomeScreen::default()))
                    Trans::None
                } else {
                    Trans::None
                }
            }
            StateEvent::Ui(UiEvent {
                event_type: UiEventType::Click,
                target,
            }) => {
                //if Some(target) == self.button_credits {
                    //log::info!("[Trans::Switch] Switching to CreditsScreen!");
                    //return Trans::Switch(Box::new(CreditsScreen::default()));
                //}
                if Some(target) == self.button_start {
                    //log::info!("[Trans::Switch] Switching to Game!");
                    //return Trans::Switch(Box::new(Game::default()));
                    println!("click start button");
                    return Trans::None;
                }
                //if Some(target) == self.button_load || Some(target) == self.button_options {
                    //log::info!("This Buttons functionality is not yet implemented!");
                //}

                Trans::None
            }
            _ => Trans::None,
        }
        
        //Trans::None
        /*
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                Trans::Quit
            } else if is_key_down(&event, VirtualKeyCode::Space) {
                //delete_paused_ui(data.world);
                println!("return");
                Trans::Pop
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
        */
    }

    #[allow(dead_code)]
    fn update(&mut self, data: StateData<CustomGameData>) -> Trans<CustomGameData<'a, 'b>, StateEvent> {

        let StateData { world, .. } = data;
        
        if self.button_start.is_none() {
            world.exec(|ui_finder: UiFinder<'_>| {
                self.button_start = ui_finder.find(BUTTON_START);
            });
        }
        

        //data.data.update(&data.world, false); // false to say we should not dispatch running
        //data.data.update(&data.world, true);
        data.data.update(&world, true);
        Trans::None
    }

    #[allow(dead_code)]
    fn on_stop(&mut self, data: StateData<CustomGameData>) {
        // after destroying the current UI, invalidate references as well (makes things cleaner)
        if let Some(root_entity) = self.ui_root {
            data.world
                .delete_entity(root_entity)
                .expect("Failed to remove MainMenu");
        }

        self.ui_root = None;
        self.button_start = None;
    }
}