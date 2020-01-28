//! Pong Tutorial 1

use amethyst::{
    audio::{AudioBundle, DjSystem},
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};
use ron::de::Result as RonDeResult;
use serde::Deserialize;

use std::fs::File;
use std::path::Path;

mod audio;
mod pong;
mod systems;

use audio::Music;
use pong::Pong;

#[derive(PartialEq, Eq)]
pub enum State {
    Paused,
    Running,
}

impl Default for State {
    fn default() -> Self {
        State::Paused
    }
}

fn main() -> amethyst::Result<()> {
    // Initialize logger
    amethyst::start_logger(Default::default());
    // Initialize display stuff
    let app_root = application_root_dir()?;

    let display_config_path = app_root.join("config").join("display.ron");
    let background_color_config_path = app_root.join("config").join("bg.ron");
    let binding_path = app_root.join("config").join("bindings.ron");

    let background_color: [f32; 4] =
        ron_de(background_color_config_path).expect("Background color valid");

    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear(background_color),
                )
                // RenderFlat2D plugin is used to render entities with a `SpriteRender` component.
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(AudioBundle::default())?
        .with(systems::PaddleSystem.pausable(State::Paused), "paddle_system", &["input_system"])
        .with(systems::MoveBallsSystem.pausable(State::Paused), "ball_system", &[])
        .with(
            DjSystem::new(|music: &mut Music| music.music.next()),
            "dj_system",
            &[],
        )
        .with(
            systems::BounceSystem,
            "collision_system",
            &["paddle_system", "ball_system"],
        )
        .with(systems::PauseSystem::default(), "pause_system", &["input_system"])
        .with(systems::WinnerSystem, "winner_system", &["ball_system"]);

    // GAME!
    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir, Pong::default(), game_data)?;
    game.run();

    Ok(())
}

fn ron_de<P, T>(path: P) -> RonDeResult<T>
where
    P: AsRef<Path>,
    T: for<'de> Deserialize<'de>,
{
    let file = File::open(path.as_ref()).expect("File not found");
    ron::de::from_reader(&file)
}
