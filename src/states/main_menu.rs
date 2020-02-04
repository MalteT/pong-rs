use amethyst::{
    assets::Handle,
    ecs::prelude::Entity,
    prelude::*,
    ui::{UiEventType, UiLoader, UiPrefab},
    utils::removal::{exec_removal, Removal},
};

use super::GameState;
use crate::audio::initialize_audio;
use crate::find_ui;
use crate::pong::State;

//const MENU_ROOT_ID: &'static str = "main_menu_root";
const MENU_BTN_SINGLE_PLAYER_ID: &'static str = "btn_single_player";
const MENU_BTN_TWO_PLAYER_ID: &'static str = "btn_two_player";
const MENU_BTN_QUIT_ID: &'static str = "btn_quit";

const MENU_RON: &'static str = "ui/main_menu.ron";

#[derive(Default)]
pub struct MainMenuState {
    ui: Option<Handle<UiPrefab>>,
    root: Option<Entity>,
    single_player: Option<Entity>,
    two_player: Option<Entity>,
    quit: Option<Entity>,
}

impl SimpleState for MainMenuState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.register::<Removal<State>>();
        initialize_audio(world);

        // Load main menu prefab
        self.ui = world
            .exec(|loader: UiLoader<'_>| loader.load(MENU_RON, ()))
            .into();

        // Create ui in the world
        self.root = world
            .create_entity()
            .with(self.ui.clone().expect("UI not loaded"))
            .with(Removal::new(State::MainMenu))
            .build()
            .into();
    }
    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        // Delete everything we have
        let world = data.world;
        exec_removal(&world.entities(), &world.read_storage(), State::MainMenu);
        self.root = None;
        self.single_player = None;
        self.two_player = None;
        self.quit = None;
    }
    fn handle_event(
        &mut self,
        _: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        use StateEvent::*;
        match event {
            Ui(ui_event) if ui_event.event_type == UiEventType::Click => {
                if Some(ui_event.target) == self.quit {
                    SimpleTrans::Quit
                } else if Some(ui_event.target) == self.single_player {
                    SimpleTrans::Switch(Box::from(GameState::with_single_player()))
                } else if Some(ui_event.target) == self.two_player {
                    SimpleTrans::Switch(Box::from(GameState::with_two_players()))
                } else {
                    SimpleTrans::None
                }
            }
            _ => SimpleTrans::None,
        }
    }
    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if self.single_player.is_none() || self.two_player.is_none() || self.quit.is_none() {
            self.single_player = data.world.exec(find_ui(MENU_BTN_SINGLE_PLAYER_ID));
            self.two_player = data.world.exec(find_ui(MENU_BTN_TWO_PLAYER_ID));
            self.quit = data.world.exec(find_ui(MENU_BTN_QUIT_ID));
        }
        SimpleTrans::None
    }
}
