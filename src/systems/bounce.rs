use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source},
    core::{
        math::{Rotation, Vector3},
        Transform,
    },
    ecs::prelude::{Join, Read, ReadExpect, ReadStorage, System, WriteStorage},
};

use std::ops::Deref;

use crate::{
    audio::{play_bounce_paddle_sound, play_bounce_wall_sound, Sounds},
    pong::{Ball, Paddle, Side, ARENA_HEIGHT, GRIP_WALL, MAX_ROTATION_ON_COLLISION, ROT_FACTOR},
};

pub struct BounceSystem;

impl<'s> System<'s> for BounceSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        ReadStorage<'s, Paddle>,
        ReadStorage<'s, Transform>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
    );

    fn run(
        &mut self,
        (mut balls, paddles, transforms, storage, sounds, audio_output): Self::SystemData,
    ) {
        // Check whether a ball collided, and bounce off accordingly.
        // We also check for the velocity of the ball every time,
        // to prevent multiple collisions from occurring.
        for (ball, transform) in (&mut balls, &transforms).join() {
            let ball_y = transform.translation().y;

            // Bounce at the top or the bottom of the arena.
            if (ball_y <= ball.radius && ball.velocity[1] < 0.0)
                || (ball_y >= ARENA_HEIGHT - ball.radius && ball.velocity[1] > 0.0)
            {
                ball.velocity.y = -ball.velocity.y;
                let sign = if ball_y < ARENA_HEIGHT / 2.0 {
                    1.0
                } else {
                    -1.0
                };
                ball.velocity +=
                    sign * GRIP_WALL * Vector3::new(ball.rot_velocity * ball.radius, 0.0, 0.0);
                ball.rot_velocity *= 1.0 - GRIP_WALL;
                play_bounce_wall_sound(
                    &*sounds,
                    &storage,
                    audio_output.as_ref().map(|o| o.deref()),
                );
            }

            // Bounce at the paddles.
            for (paddle, paddle_transform) in (&paddles, &transforms).join() {
                match collision_degree(&ball, &transform, &paddle, &paddle_transform) {
                    Some(degree) => {
                        let axis = Vector3::z_axis();
                        let sign = match paddle.side {
                            Side::Left => 1.0,
                            Side::Right => -1.0,
                        };
                        let unit = match paddle.side {
                            Side::Left => Vector3::new(1.0_f32, 0.0, 0.0),
                            Side::Right => Vector3::new(-1.0_f32, 0.0, 0.0),
                        };
                        let rotation = Rotation::from_axis_angle(
                            &axis,
                            sign * degree * MAX_ROTATION_ON_COLLISION,
                        );
                        // Add some rotation to the ball
                        // This adds the angular rotation to the balls rotational speed.
                        ball.rot_velocity += ROT_FACTOR * sign * paddle.velocity / ball.radius;
                        // Ignore reflection physics and use a rotated perpendicular vector
                        // of the same length as the incoming speed vector
                        ball.velocity = rotation * (ball.velocity.norm() / unit.norm() * unit);
                        ball.velocity *= 1.1;
                        play_bounce_paddle_sound(
                            &*sounds,
                            &storage,
                            audio_output.as_ref().map(|o| o.deref()),
                        );
                    }
                    None => {}
                }
            }
        }
    }
}

fn collision_degree(
    ball: &Ball,
    ball_transform: &Transform,
    paddle: &Paddle,
    paddle_transform: &Transform,
) -> Option<f32> {
    let ball_x = ball_transform.translation().x;
    let ball_y = ball_transform.translation().y;
    let paddle_x = paddle_transform.translation().x;
    let paddle_y = paddle_transform.translation().y;
    let paddle_w = paddle.width * 0.5;
    let paddle_h = paddle.height * 0.5;
    let vel = ball.velocity.x;
    if ball_y - ball.radius > paddle_y + paddle_h || ball_y + ball.radius < paddle_y - paddle_h {
        // The ball is not within the height of the paddle.
        return None;
    }
    match paddle.side {
        Side::Left if ball_x - ball.radius < paddle_x + paddle_w && vel < 0.0 => {
            Some((ball_y - paddle_y) / paddle_h)
        }
        Side::Right if ball_x + ball.radius > paddle_x - paddle_w && vel > 0.0 => {
            Some((ball_y - paddle_y) / paddle_h)
        }
        _ => None,
    }
}
