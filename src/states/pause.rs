use amethyst::{
    assets::Handle,
    ecs::prelude::Entity,
    input::InputEvent,
    prelude::{
        Builder, GameData, SimpleState, SimpleTrans, StateData, StateEvent, TransEvent, WorldExt,
    },
    shrev::EventChannel,
    ui::{UiEventType, UiLoader, UiPrefab},
    utils::removal::{exec_removal, Removal},
};

use crate::{
    find_ui,
    pong::PausedOrRunning,
    states::{MainMenuState, State},
};

const MENU_BTN_MAIN_MENU_ID: &str = "btn_main_menu";
const MENU_BTN_RESUME_ID: &str = "btn_resume";
const MENU_BTN_QUIT_ID: &str = "btn_quit";
const MENU_RON: &str = "ui/pause.ron";
const ACTION_PAUSE: &str = "pause";

#[derive(Default)]
pub struct PauseState {
    ui: Option<Handle<UiPrefab>>,
    root: Option<Entity>,
    resume: Option<Entity>,
    quit: Option<Entity>,
    main_menu: Option<Entity>,
}

impl SimpleState for PauseState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.register::<Removal<State>>();

        if self.ui.is_none() {
            self.ui = world
                .exec(|loader: UiLoader<'_>| loader.load(MENU_RON, ()))
                .into();
        }
        self.root = world
            .create_entity()
            .with(self.ui.clone().expect("UI not loaded"))
            .with(Removal::new(State::Pause))
            .build()
            .into();

        world.insert(PausedOrRunning::Paused);
    }
    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.insert(PausedOrRunning::Running);

        exec_removal(&world.entities(), &world.read_storage(), State::Pause);
        self.root = None;
        self.main_menu = None;
        self.quit = None;
        self.resume = None;
    }
    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        use InputEvent::*;
        use StateEvent::*;
        match event {
            Input(ActionPressed(action)) if action == ACTION_PAUSE => SimpleTrans::Pop,
            Ui(ui_event) if ui_event.event_type == UiEventType::Click => {
                if Some(ui_event.target) == self.quit {
                    SimpleTrans::Quit
                } else if Some(ui_event.target) == self.resume {
                    SimpleTrans::Pop
                } else if Some(ui_event.target) == self.main_menu {
                    data.world
                        .write_resource::<EventChannel<TransEvent<GameData<'_, '_>, StateEvent>>>()
                        .single_write(Box::from(|| {
                            SimpleTrans::Switch(Box::from(MainMenuState::default()))
                        }));
                    SimpleTrans::Pop
                } else {
                    SimpleTrans::None
                }
            }
            _ => SimpleTrans::None,
        }
    }
    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if self.main_menu.is_none() || self.resume.is_none() || self.quit.is_none() {
            self.main_menu = data.world.exec(find_ui(MENU_BTN_MAIN_MENU_ID));
            self.resume = data.world.exec(find_ui(MENU_BTN_RESUME_ID));
            self.quit = data.world.exec(find_ui(MENU_BTN_QUIT_ID));
        }
        SimpleTrans::None
    }
}
