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
*/


use std::marker::PhantomData;
#[allow(unused_imports)]
use amethyst::{
    input::{is_close_requested, is_key_down},
    prelude::*,
    assets::{
        Completion, Handle, Prefab, PrefabLoader, PrefabLoaderSystemDesc, ProgressCounter,
        RonFormat,
    },
    ecs::prelude::Entity,
    //utils::application_root_dir,
    //core::transform::TransformBundle,
    //renderer::{
        //plugins::{RenderFlat2D, RenderToWindow},
        //types::DefaultBackend,
        //RenderingBundle,
    //},
    ui::{RenderUi, UiBundle, UiCreator, UiLoader, UiPrefab},
    ecs::{System, World, Dispatcher, DispatcherBuilder, WorldExt},
    error::Error,
    winit::VirtualKeyCode,
};
//use amethyst::input::{InputBundle, StringBindings};
//use amethyst::ui::{RenderUi, UiBundle};
use amethyst::DataDispose;
use amethyst::core::SystemBundle;
use amethyst::core::ArcThreadPool;

pub struct CustomGameData<'a, 'b> {
    pub base: Option<Dispatcher<'a, 'b>>,
    pub running: Option<Dispatcher<'a, 'b>>,
}

impl<'a, 'b> CustomGameData<'a, 'b> {
    /// Update game data
    #[allow(dead_code)]
    pub fn update(&mut self, world: &World, running: bool) {
        if running {
            if let Some(running) = &mut self.running {
                running.dispatch(&world);
            }
        }
        if let Some(base) = &mut self.base {
            base.dispatch(&world);
        }
    }

    /// Dispose game data, dropping the dispatcher
    pub fn dispose(&mut self, world: &mut World) {
        if let Some(base) = self.base.take() {
            base.dispose(world);
        }
        if let Some(running) = self.running.take() {
            running.dispose(world);
        }
    }

    pub fn test(&mut self){
        println!("test CustomGameData");
    }
    
}

impl DataDispose for CustomGameData<'_, '_> {
    fn dispose(&mut self, world: &mut World) {
        self.dispose(world);
    }
}

pub struct CustomGameDataBuilder<'a, 'b> {
    base_dispatcher_operations: Vec<Box<dyn DispatcherOperation<'a, 'b>>>,
    running_dispatcher_operations: Vec<Box<dyn DispatcherOperation<'a, 'b>>>,
}

impl<'a, 'b> Default for CustomGameDataBuilder<'a, 'b> {
    fn default() -> Self {
        CustomGameDataBuilder::new()
    }
}

impl<'a, 'b> CustomGameDataBuilder<'a, 'b> {
    pub fn new() -> Self {
        CustomGameDataBuilder {
            base_dispatcher_operations: vec![],
            running_dispatcher_operations: vec![],
        }
    }

    pub fn test(&mut self){
        println!("test CustomGameDataBuilder");
    }

    pub fn with_base<SD, S>(
        mut self,
        system_desc: SD,
        name: &'static str,
        dependencies: &'static [&'static str],
    ) -> Self
    where
        SD: SystemDesc<'a, 'b, S> + 'static,
        S: for<'c> System<'c> + 'static + Send,
    {
        let dispatcher_operation = Box::new(AddSystem {
            system_desc,
            name,
            dependencies,
            marker: PhantomData::<S>,
        }) as Box<dyn DispatcherOperation<'a, 'b> + 'static>;
        self.base_dispatcher_operations.push(dispatcher_operation);
        self
    }

    pub fn with_base_bundle<B>(mut self, bundle: B) -> Self
    where
        B: SystemBundle<'a, 'b> + 'static,
    {
        self.base_dispatcher_operations
            .push(Box::new(AddBundle { bundle }));
        self
    }
    #[allow(dead_code)]
    pub fn with_running<SD, S>(
        mut self,
        system_desc: SD,
        name: &'static str,
        dependencies: &'static [&'static str],
    ) -> Self
    where
        SD: SystemDesc<'a, 'b, S> + 'static,
        S: for<'c> System<'c> + 'static + Send,
    {
        let dispatcher_operation = Box::new(AddSystem {
            system_desc,
            name,
            dependencies,
            marker: PhantomData::<S>,
        }) as Box<dyn DispatcherOperation<'a, 'b> + 'static>;
        self.running_dispatcher_operations
            .push(dispatcher_operation);
        self
    }
}

impl<'a, 'b> DataInit<CustomGameData<'a, 'b>> for CustomGameDataBuilder<'a, 'b> {
    fn build(self, world: &mut World) -> CustomGameData<'a, 'b> {
        let base = build_dispatcher(world, self.base_dispatcher_operations);
        let running = build_dispatcher(world, self.running_dispatcher_operations);

        CustomGameData {
            base: Some(base),
            running: Some(running),
        }
    }
}

fn build_dispatcher<'a, 'b>(
    world: &mut World,
    dispatcher_operations: Vec<Box<dyn DispatcherOperation<'a, 'b>>>,
) -> Dispatcher<'a, 'b> {
    let mut dispatcher_builder = DispatcherBuilder::new();

    #[cfg(not(no_threading))]
    {
        let pool = world.read_resource::<ArcThreadPool>().clone();
        dispatcher_builder = dispatcher_builder.with_pool((*pool).clone());
    }

    dispatcher_operations
        .into_iter()
        .try_for_each(|dispatcher_operation| {
            dispatcher_operation.exec(world, &mut dispatcher_builder)
        })
        .unwrap_or_else(|e| panic!("Failed to set up dispatcher: {}", e));

    let mut dispatcher = dispatcher_builder.build();
    dispatcher.setup(world);
    dispatcher
}

/// Trait to capture deferred dispatcher builder operations.
trait DispatcherOperation<'a, 'b> {
    /// Executes the dispatcher builder instruction.
    fn exec(
        self: Box<Self>,
        world: &mut World,
        dispatcher_builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error>;
}

struct AddSystem<SD, S> {
    system_desc: SD,
    name: &'static str,
    dependencies: &'static [&'static str],
    marker: PhantomData<S>,
}

impl<'a, 'b, SD, S> DispatcherOperation<'a, 'b> for AddSystem<SD, S>
where
    SD: SystemDesc<'a, 'b, S>,
    S: for<'s> System<'s> + Send + 'a,
{
    fn exec(
        self: Box<Self>,
        world: &mut World,
        dispatcher_builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        let system = self.system_desc.build(world);
        dispatcher_builder.add(system, self.name, self.dependencies);
        Ok(())
    }
}

struct AddBundle<B> {
    bundle: B,
}

impl<'a, 'b, B> DispatcherOperation<'a, 'b> for AddBundle<B>
where
    B: SystemBundle<'a, 'b>,
{
    fn exec(
        self: Box<Self>,
        world: &mut World,
        dispatcher_builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        self.bundle.build(world, dispatcher_builder)?;
        Ok(())
    }
}

/*
#[derive(Default)]
pub struct Paused;

impl<'a, 'b> State<CustomGameData<'a, 'b>, StateEvent> for Paused {
    #[allow(dead_code)]
    fn on_start(&mut self, 
        #[allow(unused_variables)]
        data: StateData<CustomGameData>) {
        println!("pause menu");
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
}
*/

//https://github.com/amethyst/evoli/blob/master/src/states/menu.rs
//const MENU_ID: &str = "menu";

/*
#[derive(Default)]
pub struct MainState {
    progress: ProgressCounter,
    paused_ui: Option<Handle<UiPrefab>>,
    ui_root: Option<Entity>,
}
/*
impl Default for MainState {
    fn default() -> Self {
        MainState {
            paused_ui: None,
            ui_root: None,
        }
    }
}
*/
impl<'a, 'b> State<CustomGameData<'a, 'b>, StateEvent> for MainState {

    #[allow(dead_code)]
    fn on_start(&mut self,
        //#[allow(unused_variables)]
        data: StateData<CustomGameData>) 
    {
        let world = data.world;

        self.ui_root =
            Some(world.exec(|mut creator: UiCreator<'_>| creator.create("ui/menu.ron", ())));
        
            
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
                Trans::Push(Box::new(Paused))
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
}
*/


/*
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

    let mut world = World::new();
    let game_data = CustomGameDataBuilder::default()
        //.with_running(ExampleSystem, "example_system", &[])
        .with_base_bundle( TransformBundle::new())
        .with_base_bundle( input_bundle)
        .with_base_bundle( UiBundle::<StringBindings>::new())
        .with_base_bundle(
            RenderingBundle::<DefaultBackend>::new()
                // The RenderToWindow plugin provides all the scaffolding for opening a window and
                // drawing on it
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        );

    let mut game = Application::new(assets_dir, Main, game_data)?;
    game.run();

    Ok(())
}
*/