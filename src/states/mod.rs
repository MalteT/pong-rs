//! The possible game states
mod game;
mod main_menu;
mod pause;

pub use game::GameState;
pub use main_menu::MainMenuState;
pub use pause::PauseState;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum State {
    MainMenu,
    Game,
    Pause,
}
