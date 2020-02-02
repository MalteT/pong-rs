use amethyst::{
    assets::{Handle, Loader},
    core::math::{Rotation, Unit, Vector3},
    ecs::{
        prelude::{Component, DenseVecStorage, Entity},
        Component as ComponentDer,
    },
    input::InputEvent,
    prelude::*,
    shrev::EventChannel,
    ui::{Anchor, TtfFormat, UiEventType, UiLoader, UiPrefab, UiText, UiTransform},
    winit::{Event, WindowEvent},
};
use rand::{thread_rng, Rng};

use std::f32::consts::FRAC_PI_4;

use crate::states::MainMenuState;
use crate::{find_ui, take_and_delete_if_some};

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;
pub const INITIAL_BALL_SPEED: f32 = 65.0;

pub const PADDLE_SIZE_COLLISION: [f32; 2] = [0.8, 14.13];

#[derive(Default)]
pub struct PauseState {
    ui: Option<Handle<UiPrefab>>,
    root: Option<Entity>,
    resume: Option<Entity>,
    quit: Option<Entity>,
    main_menu: Option<Entity>,
}

#[derive(PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

#[derive(ComponentDer)]
pub struct Ai;

#[derive(PartialEq)]
pub enum PausedOrRunning {
    Running,
    Paused,
}

impl Default for PausedOrRunning {
    fn default() -> Self {
        Self::Running
    }
}

pub struct Paddle {
    pub side: Side,
    pub width: f32,
    pub height: f32,
    pub velocity: f32,
}

#[derive(Debug)]
pub struct Ball {
    pub velocity: Vector3<f32>,
    pub radius: f32,
    pub hidden: Option<f32>,
    pub rot_velocity: f32,
}

/// ScoreBoard contains the actual score data
#[derive(Default)]
pub struct ScoreBoard {
    pub score_left: i32,
    pub score_right: i32,
}

/// ScoreText contains the ui text components that display the score
pub struct ScoreText {
    pub p1_score: Entity,
    pub p2_score: Entity,
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}

impl Paddle {
    pub fn new(side: Side) -> Paddle {
        Paddle {
            side,
            width: PADDLE_SIZE_COLLISION[0],
            height: PADDLE_SIZE_COLLISION[1],
            velocity: 0.0,
        }
    }
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}

impl SimpleState for PauseState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        if self.ui.is_none() {
            self.ui = data
                .world
                .exec(|loader: UiLoader<'_>| loader.load("ui/pause.ron", ()))
                .into();
        }
        self.root = data
            .world
            .create_entity()
            .with(self.ui.clone().expect("UI not loaded"))
            .build()
            .into();

        data.world.insert(PausedOrRunning::Paused);
    }
    fn on_stop(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        if let Some(state) = data.world.get_mut::<PausedOrRunning>() {
            *state = PausedOrRunning::Running;
        }
        take_and_delete_if_some(&mut data.world, &mut self.root);
        take_and_delete_if_some(&mut data.world, &mut self.main_menu);
        take_and_delete_if_some(&mut data.world, &mut self.quit);
        take_and_delete_if_some(&mut data.world, &mut self.resume);
    }
    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        use InputEvent::*;
        use StateEvent::*;
        match event {
            Input(ActionPressed(action)) if action == "pause" => SimpleTrans::Pop,
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
            self.main_menu = data.world.exec(find_ui("main_menu"));
            self.resume = data.world.exec(find_ui("resume"));
            self.quit = data.world.exec(find_ui("quit"));
        }
        SimpleTrans::None
    }
}

pub fn pause_requested(event: &StateEvent) -> bool {
    use InputEvent::*;
    use StateEvent::*;
    match event {
        Input(ActionPressed(action)) if action == "pause" => true,
        Window(Event::WindowEvent {
            window_id: _,
            event: WindowEvent::Focused(false),
        }) => true,
        _ => false,
    }
}

/// Initialises a ui scoreboard
pub fn initialize_scoreboard(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );
    let p1_transform = UiTransform::new(
        "P1".to_string(),
        Anchor::TopMiddle,
        Anchor::TopMiddle,
        -110.,
        -20.,
        1.,
        400.,
        100.,
    );
    let p2_transform = UiTransform::new(
        "P2".to_string(),
        Anchor::TopMiddle,
        Anchor::TopMiddle,
        110.,
        -20.,
        1.,
        400.,
        100.,
    );

    let p1_score = world
        .create_entity()
        .with(p1_transform)
        .with(UiText::new(
            font.clone(),
            "0".to_string(),
            [1.0, 0.0, 0.0, 0.2],
            100.,
        ))
        .build();

    let p2_score = world
        .create_entity()
        .with(p2_transform)
        .with(UiText::new(
            font.clone(),
            "0".to_string(),
            [1.0, 0.0, 0.0, 0.2],
            100.,
        ))
        .build();

    world.insert(ScoreText { p1_score, p2_score });
}

pub fn random_45_vec(base: &Unit<Vector3<f32>>, norm: f32) -> Vector3<f32> {
    let mut rng = thread_rng();
    let angle = rng.gen_range(-FRAC_PI_4, FRAC_PI_4);
    let rotation = Rotation::from_axis_angle(&Vector3::z_axis(), angle);
    norm * (rotation * base.into_inner())
}
