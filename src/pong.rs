use amethyst::{
    core::math::{Rotation, Unit, Vector3},
    ecs::{
        prelude::{Component, DenseVecStorage, Entity},
        Component as ComponentDer,
    },
    input::InputEvent,
    prelude::*,
    winit::{Event, WindowEvent},
};
use rand::{thread_rng, Rng};

use std::f32::consts::FRAC_PI_4;

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;
pub const INITIAL_BALL_SPEED: f32 = 65.0;

pub const PADDLE_SIZE_COLLISION: [f32; 2] = [0.8, 14.13];

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum State {
    MainMenu,
    Game,
    Pause,
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

pub fn random_45_vec(base: &Unit<Vector3<f32>>, norm: f32) -> Vector3<f32> {
    let mut rng = thread_rng();
    let angle = rng.gen_range(-FRAC_PI_4, FRAC_PI_4);
    let rotation = Rotation::from_axis_angle(&Vector3::z_axis(), angle);
    norm * (rotation * base.into_inner())
}
