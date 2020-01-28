use amethyst::{
    assets::{AssetStorage, Loader},
    audio::{AudioSink, output::Output, OggFormat, Source, SourceHandle},
    ecs::{World, WorldExt},
};

use std::iter::Cycle;
use std::vec::IntoIter;

const BOUNCE_SOUND: &str = "audio/bounce.ogg";
const SCORE_SOUND: &str = "audio/score.ogg";
const MUSIC_TRACKS: &[&str] = &[
    "audio/Computer_Music_All-Stars_-_Wheres_My_Jetpack.ogg",
    "audio/Computer_Music_All-Stars_-_Albatross_v2.ogg",
];

pub struct Sounds {
    pub score_sfx: SourceHandle,
    pub bounce_sfx: SourceHandle,
}

pub struct Music {
    pub music: Cycle<IntoIter<SourceHandle>>,
}

/// Loads an ogg audio track.
fn load_audio_track(loader: &Loader, world: &World, file: &str) -> SourceHandle {
    loader.load(file, OggFormat, (), &world.read_resource())
}

/// Initialise audio in the world. This will eventually include
/// the background tracks as well as the sound effects, but for now
/// we'll just work on sound effects.
pub fn initialize_audio(world: &mut World) {
    let (sound_effects, music) = {
        world.insert(AudioSink::new(&Default::default()));
        let loader = world.read_resource::<Loader>();

        let sound = Sounds {
            bounce_sfx: load_audio_track(&loader, &world, BOUNCE_SOUND),
            score_sfx: load_audio_track(&loader, &world, SCORE_SOUND),
        };

        let mut sink = world.write_resource::<AudioSink>();
        sink.set_volume(0.15); // Music is a bit loud, reduce the volume.

        let music = MUSIC_TRACKS
            .iter()
            .map(|file| load_audio_track(&loader, &world, file))
            .collect::<Vec<_>>()
            .into_iter()
            .cycle();
        let music = Music { music };

        (sound, music)
    };

    // Add sound effects to the world. We have to do this in another scope because
    // world won't let us insert new resources as long as `Loader` is borrowed.
    world.insert(sound_effects);
    world.insert(music);
}

pub fn play_bounce_sound(sounds: &Sounds, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&sounds.bounce_sfx) {
            output.play_once(sound, 1.0);
        }
    }
}

pub fn play_score_sound(sounds: &Sounds, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&sounds.score_sfx) {
            output.play_once(sound, 0.5);
        }
    }
}
