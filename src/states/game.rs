use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::{math::Vector3, transform::Transform, ArcThreadPool, SystemExt},
    ecs::{world::Builder, Dispatcher, DispatcherBuilder},
    prelude::{GameData, SimpleState, SimpleTrans, StateData, StateEvent, World, WorldExt},
    renderer::{
        transparent::Transparent, Camera, ImageFormat, SpriteRender, SpriteSheet,
        SpriteSheetFormat, Texture,
    },
    ui::{Anchor, TtfFormat, UiText, UiTransform},
    utils::removal::{exec_removal, Removal},
};

use crate::{
    pong::{
        pause_requested, random_45_vec, Ai, Ball, Paddle, PausedOrRunning, ScoreBoard, ScoreText,
        Side, ARENA_HEIGHT, ARENA_WIDTH, BALL_RADIUS, BALL_RADIUS_COLLISION, BALL_TEXTURE_SIZE,
        INITIAL_BALL_SPEED, PADDLE_SIZE, PADDLE_SIZE_COLLISION, PADDLE_TEXTURE_SIZE,
        PADDLE_WALL_OFFSET,
    },
    states::{PauseState, State},
    systems,
};

pub struct GameState<'a, 'b> {
    dispatcher: Option<Dispatcher<'a, 'b>>,
    two_players: bool,
}

impl GameState<'_, '_> {
    pub fn with_single_player() -> Self {
        GameState {
            dispatcher: None,
            two_players: false,
        }
    }
    pub fn with_two_players() -> Self {
        GameState {
            dispatcher: None,
            two_players: true,
        }
    }
}

impl SimpleState for GameState<'_, '_> {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.register::<Removal<State>>();
        // Create a blank score board
        world.insert(ScoreBoard::default());

        // Create the `DispatcherBuilder` and register some `System`s
        // that should only run for this `State`.
        let mut dispatcher = DispatcherBuilder::new()
            .with(
                systems::PaddleSystem.pausable(PausedOrRunning::Running),
                "paddle_system",
                &[], //&["input_system"],
            )
            .with(
                systems::MoveBallsSystem.pausable(PausedOrRunning::Running),
                "ball_system",
                &[],
            )
            .with(
                systems::BounceSystem.pausable(PausedOrRunning::Running),
                "collision_system",
                &["paddle_system", "ball_system"],
            )
            .with(
                systems::WinnerSystem.pausable(PausedOrRunning::Running),
                "winner_system",
                &["ball_system"],
            )
            .with(
                systems::AiSystem.pausable(PausedOrRunning::Running),
                "ai_system",
                &["paddle_system", "ball_system"],
            )
            .with_pool((*world.read_resource::<ArcThreadPool>()).clone())
            .build();
        dispatcher.setup(world);
        self.dispatcher = Some(dispatcher);

        let sprites = load_sprite_sheet(world);
        initialize_scoreboard(world);
        initialize_camera(world);
        initialize_ball(world, sprites.clone());
        initialize_paddles(world, sprites.clone(), self.two_players);
    }
    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        exec_removal(&world.entities(), &world.read_storage(), State::Game);
    }
    fn handle_event(
        &mut self,
        _: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if pause_requested(&event) {
            return SimpleTrans::Push(Box::from(PauseState::default()));
        }

        SimpleTrans::None
    }
    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if let Some(dispatcher) = self.dispatcher.as_mut() {
            dispatcher.dispatch(&data.world);
        }

        SimpleTrans::None
    }
}

/// Initializes one paddle on the left, and one paddle on the right.
fn initialize_paddles(world: &mut World, sprite_sheet: Handle<SpriteSheet>, two_players: bool) {
    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();

    // Correctly position the paddles.
    let y = ARENA_HEIGHT / 2.0;
    left_transform.set_translation_xyz(PADDLE_WALL_OFFSET + PADDLE_SIZE_COLLISION[0] * 0.5, y, 0.0);
    left_transform.set_scale(
        [
            PADDLE_SIZE[0] / PADDLE_TEXTURE_SIZE[0],
            PADDLE_SIZE[1] / PADDLE_TEXTURE_SIZE[1],
            1.0,
        ]
        .into(),
    );
    right_transform.set_translation_xyz(
        ARENA_WIDTH - PADDLE_WALL_OFFSET - PADDLE_SIZE_COLLISION[0] * 0.5,
        y,
        0.0,
    );
    right_transform.set_scale(
        [
            PADDLE_SIZE[0] / PADDLE_TEXTURE_SIZE[0],
            PADDLE_SIZE[1] / PADDLE_TEXTURE_SIZE[1],
            1.0,
        ]
        .into(),
    );

    // Assign the sprites for the paddles
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0, // paddle is the first sprite in the sprite_sheet
    };

    // Create a left plank entity.
    world
        .create_entity()
        .with(Paddle::new(Side::Left))
        .with(sprite_render.clone())
        .with(left_transform)
        .with(Transparent)
        .with(Removal::new(State::Game))
        .build();

    // Create right plank entity.
    let mut right = world
        .create_entity()
        .with(Paddle::new(Side::Right))
        .with(sprite_render.clone())
        .with(right_transform)
        .with(Transparent)
        .with(Removal::new(State::Game));
    // Add AI if only one player is playing
    if !two_players {
        right = right.with(Ai);
    }
    right.build();
}

fn initialize_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena
    // and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .with(Removal::new(State::Game))
        .build();
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `texture_handle` is a cloneable reference to the texture
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/sprites.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/sprites.ron", // Here we load the associated ron file
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

/// Initialises one ball in the middle-ish of the arena.
fn initialize_ball(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    // Create the translation.
    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 0.0);
    local_transform.set_scale(
        [
            2.0 * BALL_RADIUS / BALL_TEXTURE_SIZE[0],
            2.0 * BALL_RADIUS / BALL_TEXTURE_SIZE[1],
            1.0,
        ]
        .into(),
    );

    // Assign the sprite for the ball
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 1, // ball is the second sprite on the sprite sheet
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(Ball {
            radius: BALL_RADIUS_COLLISION,
            velocity: random_45_vec(&Vector3::x_axis(), INITIAL_BALL_SPEED),
            hidden: Some(2.0),
            rot_velocity: 0.0,
        })
        .with(local_transform)
        .with(Transparent)
        .with(Removal::new(State::Game))
        .build();
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
        .with(Removal::new(State::Game))
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
        .with(Removal::new(State::Game))
        .build();

    world.insert(ScoreText { p1_score, p2_score });
}
