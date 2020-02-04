use amethyst::core::{math::clamp, timing::Time, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

// You'll have to mark PADDLE_HEIGHT as public in pong.rs
use crate::pong::{Paddle, Ai, Side, ARENA_HEIGHT, PADDLE_SIZE_COLLISION};

#[derive(SystemDesc)]
pub struct PaddleSystem;

const BOTTOM_OF_SCREEN: f32 = ARENA_HEIGHT - PADDLE_SIZE_COLLISION[1] * 0.5;
const TOP_OF_SCREEN: f32 = PADDLE_SIZE_COLLISION[1] * 0.5;

impl<'s> System<'s> for PaddleSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Paddle>,
        ReadStorage<'s, Ai>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, mut paddles, ais, input, time): Self::SystemData) {
        for (paddle, _, transform) in (&mut paddles, !&ais, &mut transforms).join() {
            let acceleration = match paddle.side {
                Side::Left => input.axis_value("left_paddle"),
                Side::Right => input.axis_value("right_paddle"),
            };
            if let Some(acc) = acceleration {
                let scaled_acc = 2000.0 * acc;
                let new_speed = paddle.velocity + scaled_acc * time.delta_seconds();
                paddle.velocity = clamp(new_speed, -150.0, 150.0);
                paddle.velocity *= 1.0 - 5.0 * time.delta_seconds();
            }
            let mut paddle_y = transform.translation().y;
            paddle_y += paddle.velocity * time.delta_seconds();
            if paddle_y < TOP_OF_SCREEN {
                paddle_y = TOP_OF_SCREEN;
                paddle.velocity *= -0.3;
            } else if paddle_y > BOTTOM_OF_SCREEN {
                paddle_y = BOTTOM_OF_SCREEN;
                paddle.velocity *= -0.3;
            }
            transform.set_translation_y(paddle_y);
        }
    }
}
