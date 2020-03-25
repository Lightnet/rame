// #[allow(clippy::type_complexity, dead_code)]
#[allow(dead_code)]
/*

*/

extern crate amethyst;

#[allow(unused_imports)]
use amethyst::{
  input::is_key_down, prelude::*,
  utils::application_root_dir,
  //window::WindowBundle,
  core::transform::TransformBundle,
  prelude::*,
  renderer::{
      plugins::{RenderFlat2D, RenderToWindow},
      types::DefaultBackend,
      RenderingBundle,
  },
  winit::VirtualKeyCode,
};
use amethyst::input::{InputBundle, StringBindings};
use amethyst::ui::{RenderUi, UiBundle};
/*
struct ExampleState;
impl SimpleState for ExampleState {

    fn handle_event(&mut self,_: StateData<'_, GameData<'_, '_>>,event: StateEvent,) -> SimpleTrans {
        if let StateEvent::Window(event) = event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                Trans::Quit
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
    }
}
*/

mod sandbox;
use crate::sandbox::Sandbox;

mod systems; // Import the module

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config/display.ron");
    let binding_path = app_root.join("config").join("bindings.ron");
    let input_bundle = InputBundle::<StringBindings>::new()
    .with_bindings_from_file(binding_path)?;

    let assets_dir = app_root.join("assets/");

    let game_data = GameDataBuilder::default()
        //.with_bundle(WindowBundle::from_config_path(&display_config_path)?)?
        // Add the transform bundle which handles tracking entity positions
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(systems::ControllerSystem, "controller_system", &["input_system"])
        .with_bundle(UiBundle::<StringBindings>::new())?
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

    //let mut game = Application::new(assets_dir, ExampleState, game_data)?;
    let mut game = Application::new(assets_dir, Sandbox::default(), game_data)?;
    game.run();

    Ok(())
}