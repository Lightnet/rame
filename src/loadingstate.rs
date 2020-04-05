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
    ui::{RenderUi, UiBundle, UiCreator, UiLoader, UiPrefab},
    ecs::{System, SystemData, World, Dispatcher, DispatcherBuilder, WorldExt},
    ecs::prelude::Entity,
    error::Error,
    winit::VirtualKeyCode,
    DataDispose, DataInit,
};

use crate::customgamedata::*;

#[derive(Default)]
pub struct Loading{
    #[allow(dead_code)]
    progress: ProgressCounter,
    #[allow(dead_code)]
    ui_root: Option<Entity>,
}

impl<'a, 'b> State<CustomGameData<'a, 'b>, StateEvent> for Loading {
    #[allow(dead_code)]
    fn on_start(&mut self, 
        #[allow(unused_variables)]
        data: StateData<CustomGameData>) {

        println!("Loading menu");
        let world = data.world;

        let menuname: &str = "ui/loading.ron";

        self.ui_root =
            Some(world.exec(|mut creator: UiCreator<'_>| creator.create(menuname, ())));

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
                println!("return");
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

    #[allow(dead_code)]
    fn on_stop(&mut self, 
        #[allow(dead_code)]
        data: StateData<CustomGameData>) {
        // after destroying the current UI, invalidate references as well (makes things cleaner)
        if let Some(root_entity) = self.ui_root {
            data.world
                .delete_entity(root_entity)
                .expect("Failed to remove Menu UI");
        }

        self.ui_root = None;
        //self.progress = None;
        //self.button_start = None;
    }



}