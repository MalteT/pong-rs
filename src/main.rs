//! Pong Tutorial 1
#![warn(rust_2018_idioms)]
#![forbid(unsafe_code)]

use amethyst::{
    audio::{AudioBundle, DjSystem},
    core::transform::TransformBundle,
    ecs::prelude::Entity,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle, UiFinder},
    utils::application_root_dir,
};
use ron::de::Result as RonDeResult;
use serde::Deserialize;

use std::fs::File;
use std::path::Path;

mod audio;
mod pong;
mod states;
mod systems;

use audio::Music;
use pong::PausedOrRunning;
use states::MainMenuState;

fn main() -> amethyst::Result<()> {
    // Initialize logger
    amethyst::start_logger(Default::default());
    // Initialize display stuff
    let app_root = application_root_dir()?;
    // Define paths
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
                // The RenderToWindow plugin provides all the scaffolding for
                // opening a window and drawing on it
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear(background_color),
                )
                // RenderFlat2D plugin is used to render entities
                // with a `SpriteRender` component.
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(AudioBundle::default())?
        .with(
            DjSystem::new(|music: &mut Music| music.music.next())
                .pausable(PausedOrRunning::Running),
            "dj_system",
            &[],
        );

    // GAME!
    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir, MainMenuState::default(), game_data)?;
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

// Trys to delete the given entity, if some. If successful, the entity
// is taken from the Option, leaving `None` in place.
fn take_and_delete_if_some(world: &mut World, entity: &mut Option<Entity>) {
    match entity {
        Some(ref mut inner) => {
            if world.delete_entity(*inner).is_ok() {
                let _ = entity.take();
            }
        }
        None => {}
    }
}

/// Find's the UI Element by name
fn find_ui(name: &'static str) -> impl FnOnce(UiFinder<'_>) -> Option<Entity> {
    move |finder| finder.find(name)
}
