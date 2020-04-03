
extern crate amethyst;

use amethyst::{
  input::is_key_down, prelude::*,
  utils::application_root_dir,
  window::WindowBundle,
  core::transform::TransformBundle,
  prelude::*,
  renderer::{
      plugins::{RenderFlat2D, RenderToWindow},
      types::DefaultBackend,
      RenderingBundle,
  },
  winit::VirtualKeyCode,
};

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

fn main() -> amethyst::Result<()> {
  amethyst::start_logger(Default::default());

  let app_root = application_root_dir()?;
  let display_config_path = app_root.join("config/display.ron");

  let assets_dir = app_root.join("assets/");

  let game_data = GameDataBuilder::default()
      //.with_bundle(WindowBundle::from_config_path(&display_config_path)?)?
      .with_bundle(TransformBundle::new())?
      .with_bundle(
        RenderingBundle::<DefaultBackend>::new()
            .with_plugin(
                RenderToWindow::from_config_path(&display_config_path)?
                    .with_clear([0.34, 0.36, 0.52, 1.0]),
            )
            .with_plugin(RenderFlat2D::default()),
        )?;

  let mut game = Application::new(assets_dir, ExampleState, game_data)?;
  game.run();

  Ok(())
}

/*
use amethyst::{
  core::transform::TransformBundle,
  prelude::*,
  renderer::{
      plugins::{RenderFlat2D, RenderToWindow},
      types::DefaultBackend,
      RenderingBundle,
  },
  utils::application_root_dir,
};

mod state;

fn main() -> amethyst::Result<()> {
  amethyst::start_logger(Default::default());

  let app_root = application_root_dir()?;

  let resources = app_root.join("resources");
  let display_config = resources.join("display_config.ron");

  let game_data = GameDataBuilder::default()
      .with_bundle(TransformBundle::new())?
      .with_bundle(
          RenderingBundle::<DefaultBackend>::new()
              .with_plugin(
                  RenderToWindow::from_config_path(display_config)
                      .with_clear([0.34, 0.36, 0.52, 1.0]),
              )
              .with_plugin(RenderFlat2D::default()),
      )?;

  let mut game = Application::new(resources, state::MyState, game_data)?;
  game.run();

  Ok(())
}
*/