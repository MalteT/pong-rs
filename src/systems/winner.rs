use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source},
    core::math::Vector3,
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadExpect, System, SystemData, Write, WriteStorage},
    ui::UiText,
};

use std::ops::Deref;

use crate::audio::{play_score_sound, Sounds};
use crate::pong::{
    random_45_vec, Ball, ScoreBoard, ScoreText, ARENA_HEIGHT, ARENA_WIDTH, INITIAL_BALL_SPEED,
};

#[derive(SystemDesc)]
pub struct WinnerSystem;

impl<'s> System<'s> for WinnerSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, UiText>,
        Write<'s, ScoreBoard>,
        ReadExpect<'s, ScoreText>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
    );

    fn run(
        &mut self,
        (mut balls, mut locals, mut ui_text, mut scores, score_text, storage, sounds, audio_output): Self::SystemData,
    ) {
        for (ball, transform) in (&mut balls, &mut locals).join() {
            let ball_x = transform.translation().x;

            let did_hit = if ball_x <= ball.radius {
                // Right player scored on the left side.
                // We top the score at 999 to avoid text overlap.
                scores.score_right = (scores.score_right + 1).min(999);

                if let Some(text) = ui_text.get_mut(score_text.p2_score) {
                    text.text = scores.score_right.to_string();
                }
                true
            } else if ball_x >= ARENA_WIDTH - ball.radius {
                // Left player scored on the right side.
                // We top the score at 999 to avoid text overlap.
                scores.score_left = (scores.score_left + 1).min(999);
                if let Some(text) = ui_text.get_mut(score_text.p1_score) {
                    text.text = scores.score_left.to_string();
                }
                true
            } else {
                false
            };

            if did_hit {
                let base_speed = if ball_x < ARENA_WIDTH / 2.0 {
                    Vector3::x_axis()
                } else {
                    -Vector3::x_axis()
                };
                ball.velocity = random_45_vec(&base_speed, INITIAL_BALL_SPEED); // Reverse Direction
                ball.rot_velocity = 0.0;
                ball.hidden = Some(1.0);
                transform.set_translation_x(ARENA_WIDTH / 2.0); // Reset Position
                transform.set_translation_y(ARENA_HEIGHT / 2.0); // Reset Position
                println!(
                    "Score: | {:^3} | {:^3} |",
                    scores.score_left, scores.score_right
                );
                play_score_sound(&*sounds, &storage, audio_output.as_ref().map(|o| o.deref()));
            }
        }
    }
}
