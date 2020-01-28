use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, System, SystemData, World, WriteStorage},
};

use crate::pong::Ball;

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
                local.prepend_translation_x(ball.velocity[0] * time.delta_seconds());
                local.prepend_translation_y(ball.velocity[1] * time.delta_seconds());
            }
        }
    }
}
