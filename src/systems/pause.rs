use amethyst::core::SystemDesc;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Read, System, SystemData, World, Write};
use amethyst::input::{InputHandler, StringBindings};

use crate::State;

#[derive(SystemDesc, Default)]
pub struct PauseSystem {
    key_up: bool,
}

impl<'s> System<'s> for PauseSystem {
    type SystemData = (Read<'s, InputHandler<StringBindings>>, Write<'s, State>);

    fn run(&mut self, (input, mut state): Self::SystemData) {
        match input.action_is_down("pause") {
            Some(true) if self.key_up => {
                if *state == State::Paused {
                    *state = State::Running;
                } else {
                    *state = State::Paused
                }
                self.key_up = false;
            }
            Some(false) if !self.key_up => {
                self.key_up = true;
            }
            None => {
                panic!("pause keybinding is undefined!");
            }
            _ => {}
        }
    }
}
