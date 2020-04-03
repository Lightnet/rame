//use amethyst::core::{Transform, SystemDesc};
use amethyst::core::{Transform};
use amethyst::derive::SystemDesc;
//use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

// You'll have to mark PADDLE_HEIGHT as public in sandbox.rs
//use crate::sandbox::{Player, ARENA_HEIGHT, PADDLE_HEIGHT};
use crate::sandbox::{Player};

//https://book.amethyst.rs/stable/input/handling_input.html
#[derive(SystemDesc)]
pub struct ControllerSystem;

impl<'s> System<'s> for ControllerSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, players, input): Self::SystemData) {
        for (_player, _transform) in (&players, &mut transforms).join() {
            //let v = input.axis_value("y_axis");
            //let h = input.axis_value("x_axis");
            
            //println!("v: {:?} h: {:?}", v,h);

            let shoot = input.action_is_down("shoot").unwrap_or(false);

            if shoot {
                _player.shoot();
            }

            /*
            let movement = match paddle.side {
                Side::Left => input.axis_value("left_paddle"),
                Side::Right => input.axis_value("right_paddle"),
            };
            if let Some(mv_amount) = movement {

            let scaled_amount = 1.2 * mv_amount as f32;
            let paddle_y = transform.translation().y;
            transform.set_translation_y(
                (paddle_y + scaled_amount)
                    .min(ARENA_HEIGHT - PADDLE_HEIGHT * 0.5)
                    .max(PADDLE_HEIGHT * 0.5),
            );
          }
          */
        }
    }
}