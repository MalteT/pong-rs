use amethyst::{
    core::math::{Rotation, Vector3},
    core::timing::Time,
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, System, SystemData, World, WriteStorage},
};

use crate::pong::Ball;

const SPEED_ROT_FACTOR: f32 = 0.01;

#[derive(SystemDesc)]
pub struct MoveBallsSystem;

impl<'s> System<'s> for MoveBallsSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut balls, mut locals, time): Self::SystemData) {
        // Move every ball according to its speed, and the time passed.
        for (ball, local) in (&mut balls, &mut locals).join() {
            if ball.hidden.is_some() {
                let timer = ball.hidden.as_mut().unwrap();
                if *timer <= 0.0 {
                    ball.hidden = None;
                } else {
                    *timer -= time.delta_seconds();
                }
            } else {
                let speed_rot = Rotation::from_axis_angle(
                    &Vector3::z_axis(),
                    SPEED_ROT_FACTOR * ball.rot_velocity * time.delta_seconds(),
                );
                ball.velocity = speed_rot * ball.velocity;
                local.prepend_translation(ball.velocity * time.delta_seconds());
                local.prepend_rotation_z_axis(ball.rot_velocity * time.delta_seconds());
            }
        }
    }
}
