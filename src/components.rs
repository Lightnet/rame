/*
    Project Name: Rame
    Created by: Lightnet
    License: MIT
    Information: work in progress test.

*/

#[allow(unused_imports)]
use amethyst::{
  	assets::{AssetStorage, Loader, Handle},
  	core::transform::Transform,
  	ecs::prelude::{Component, DenseVecStorage, Entity},
  	prelude::*,
  	renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    ui::{Anchor, TtfFormat, UiText, UiTransform},
    input::{is_close_requested, is_key_down, is_mouse_button_down},
    ui::UiCreator,
    winit::{MouseButton, VirtualKeyCode},
};

#[allow(dead_code)]
pub const ARENA_HEIGHT: f32 = 800.0;
#[allow(dead_code)]
pub const ARENA_WIDTH: f32 = 600.0;
#[allow(dead_code)]
pub const PADDLE_HEIGHT: f32 = 16.0;
#[allow(dead_code)]
pub const PADDLE_WIDTH: f32 = 4.0;


#[derive(PartialEq, Eq)]
pub enum Side {
	#[allow(dead_code)]
    Left,
    #[allow(dead_code)]
    Right,
}

//#[derive(Default)]
pub struct Player {
    pub id: usize,
	pub side: Side,
    pub width: f32,
    pub height: f32,
}

impl Player {
	#[allow(dead_code)]
    fn new(side: Side) -> Player {
        Player {
            id: 0,
            side,
            width: PADDLE_WIDTH,
            height: PADDLE_HEIGHT,
        }
    }
}

impl Player {
    pub fn shoot(&self) {
        println!("PEW! {}", self.id);
    }
}

impl Component for Player {
	type Storage = DenseVecStorage<Self>;
}