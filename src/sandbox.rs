

#[allow(unused_imports)]
use amethyst::{
  	assets::{AssetStorage, Loader, Handle},
  	core::transform::Transform,
  	ecs::prelude::{Component, DenseVecStorage, Entity},
  	prelude::*,
  	renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
  	ui::{Anchor, TtfFormat, UiText, UiTransform},
};

use amethyst::core::timing::Time;

pub const ARENA_HEIGHT: f32 = 800.0;
pub const ARENA_WIDTH: f32 = 600.0;

pub const PADDLE_HEIGHT: f32 = 16.0;
pub const PADDLE_WIDTH: f32 = 4.0;

#[derive(Default)]
pub struct Sandbox {
  	#[allow(dead_code)]
  	ball_spawn_timer: Option<f32>,
  	#[allow(dead_code)]
  	sprite_sheet_handle: Option<Handle<SpriteSheet>>,
}

impl SimpleState for Sandbox {
  	// #![allow(dead_code)]
	fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
    	let world = _data.world;
		initialise_player(world);

		initialise_camera(world);
	}
  	// #![allow(dead_code)]
	fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
		let _time = _data.world.fetch::<Time>();

  		Trans::None
	}
}

fn initialise_camera(world: &mut World) {
	// Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left. 
	let mut transform = Transform::default();
	transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);
  
	world
		.create_entity()
		.with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
		.with(transform)
		.build();
}

#[derive(PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

//#[derive(Default)]
pub struct Player {
	pub side: Side,
    pub width: f32,
    pub height: f32,
}

impl Player {
    fn new(side: Side) -> Player {
        Player {
            side,
            width: PADDLE_WIDTH,
            height: PADDLE_HEIGHT,
        }
    }
}

impl Component for Player {
	type Storage = DenseVecStorage<Self>;
}

/// Initialises one player
//fn initialise_player(world: &mut World, sprite_sheet: Handle<SpriteSheet>) {
fn initialise_player(world: &mut World) {
	// Assign the sprites for the paddles
	//let sprite_render = SpriteRender {
	  //sprite_sheet: sprite_sheet.clone(),
	  //sprite_number: 0, // paddle is the first sprite in the sprite_sheet
	//};
  
	//let mut left_transform = Transform::default();
	//let mut right_transform = Transform::default();
	let mut player_transform = Transform::default();
  
	// Correctly position the paddles.
	let y = ARENA_HEIGHT / 2.0;
	//left_transform.set_translation_xyz(PADDLE_WIDTH * 0.5, y, 0.0);
	//right_transform.set_translation_xyz(ARENA_WIDTH - PADDLE_WIDTH * 0.5, y, 0.0);
	player_transform.set_translation_xyz(PADDLE_WIDTH * 0.5, y, 0.0);


	world
		.create_entity()
		//.with(sprite_render.clone())
		.with(Player::new(Side::Left))
		.with(player_transform)
		.build();
  
	// Create a left plank entity.
	//world
		//.create_entity()
		//.with(sprite_render.clone())
		//.with(Paddle::new(Side::Left))
		//.with(left_transform)
		//.build();
  
	// Create right plank entity.
	//world
		//.create_entity()
		//.with(sprite_render.clone())
		//.with(Paddle::new(Side::Right))
		//.with(right_transform)
		//.build();
  }