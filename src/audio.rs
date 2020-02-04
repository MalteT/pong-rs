use amethyst::{
    assets::{AssetStorage, Loader},
    audio::{output::Output, AudioSink, OggFormat, Source, SourceHandle},
    ecs::{World, WorldExt},
};
use rand::{thread_rng, Rng};

use std::{iter::Cycle, vec::IntoIter};

const BOUNCE_WALL_SOUND: &str = "audio/bounce_wall.wav";
const BOUNCE_PADDLE_SOUND: &str = "audio/bounce_paddle.wav";
const SCORE_SOUND: &str = "audio/score.wav";
const WILHELM_SOUND: &str = "audio/wilhelm.ogx";
const ROBLOX_SOUND: &str = "audio/Roblox-death-sound.mp3";

const MUSIC_TRACKS: &[&str] = &[
    "audio/Computer_Music_All-Stars_-_Wheres_My_Jetpack.ogg",
    "audio/Computer_Music_All-Stars_-_Albatross_v2.ogg",
];

pub struct Sounds {
    pub score_sfx: SourceHandle,
    pub wilhelm_sfx: SourceHandle,
    pub roblox_death_sfx: SourceHandle,
    pub bounce_wall_sfx: SourceHandle,
    pub bounce_paddle_sfx: SourceHandle,
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
            bounce_wall_sfx: load_audio_track(&loader, &world, BOUNCE_WALL_SOUND),
            bounce_paddle_sfx: load_audio_track(&loader, &world, BOUNCE_PADDLE_SOUND),
            score_sfx: load_audio_track(&loader, &world, SCORE_SOUND),
            wilhelm_sfx: load_audio_track(&loader, &world, WILHELM_SOUND),
            roblox_death_sfx: load_audio_track(&loader, &world, ROBLOX_SOUND),
        };

        let mut sink = world.write_resource::<AudioSink>();
        sink.set_volume(0.05); // Music is a bit loud, reduce the volume.

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

pub fn play_bounce_wall_sound(
    sounds: &Sounds,
    storage: &AssetStorage<Source>,
    output: Option<&Output>,
) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&sounds.bounce_wall_sfx) {
            output.play_once(sound, 1.0);
        }
    }
}

pub fn play_bounce_paddle_sound(
    sounds: &Sounds,
    storage: &AssetStorage<Source>,
    output: Option<&Output>,
) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&sounds.bounce_paddle_sfx) {
            output.play_once(sound, 1.0);
        }
    }
}

pub fn play_score_sound(sounds: &Sounds, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(ref output) = output.as_ref() {
        let lucky_nr = thread_rng().gen_range(0.0, 1.0);
        if lucky_nr > 0.99 {
            if let Some(sound) = storage.get(&sounds.wilhelm_sfx) {
                output.play_once(sound, 0.3);
            }
        } else if lucky_nr > 0.95 {
            if let Some(sound) = storage.get(&sounds.roblox_death_sfx) {
                output.play_once(sound, 0.5);
            }
        } else {
            if let Some(sound) = storage.get(&sounds.score_sfx) {
                output.play_once(sound, 0.3);
            }
        }
    }
}
