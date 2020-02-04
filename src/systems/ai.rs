use amethyst::{
    core::{math::clamp, timing::Time, Transform},
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

use crate::pong::{Ai, Ball, Paddle, BOTTOM_OF_SCREEN, TOP_OF_SCREEN};

#[derive(SystemDesc)]
pub struct AiSystem;

impl<'s> System<'s> for AiSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Paddle>,
        ReadStorage<'s, Ball>,
        ReadStorage<'s, Ai>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, mut paddles, balls, ai, time): Self::SystemData) {
        let (ball_transform, _) = match (&transforms, &balls).join().next() {
            Some(tuple) => tuple,
            None => {
                eprintln!("No ball in system. Ai useless..");
                return;
            }
        };
        let ball_transform = ball_transform.clone();
        for (paddle, transform, _) in (&mut paddles, &mut transforms, &ai).join() {
            // AI Part
            let ball_y = ball_transform.translation().y;
            let paddle_y = transform.translation().y;
            let scaled_acc = clamp(100.0 * (ball_y - paddle_y), -300.0, 300.0);

            // Physics
            let new_speed = paddle.velocity + scaled_acc * time.delta_seconds();
            paddle.velocity = clamp(new_speed, -150.0, 150.0);
            paddle.velocity *= 1.0 - 5.0 * time.delta_seconds();
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
