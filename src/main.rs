use bevy::prelude::*;

mod board;
mod tile;

use board::Board;
use tile::{Tile, Direction};

const TILE_SIZE : f32 = 100.0;
const TILE_GAP : f32 = 20.0;


#[derive(Resource)]
struct GameState {
    board: Board,
}

impl Default for GameState {
    fn default() -> Self {
        let mut board = Board::new(4);

        board.spawn_random_tile();
        board.spawn_random_tile();

        Self { board }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(GameState::default());
}

fn handle_input(keys: Res<Input<KeyCode>>, mut game_state: ResMut<GameState>) {
    let direction = match () {
        _ if keys.just_pressed(KeyCode::Left)  => Some(Direction::Left),
        _ if keys.just_pressed(KeyCode::Right) => Some(Direction::Right),
        _ if keys.just_pressed(KeyCode::Up)    => Some(Direction::Up),
        _ if keys.just_pressed(KeyCode::Down)  => Some(Direction::Down),
        _ => None,
    };

    if let Some(direction) = direction {
        let before: String = game_state.board.to_string();
        let moved = game_state.board.slide_and_merge(direction);
        let after: String = game_state.board.to_string();
        println!("{} --{}--> {}", before, direction, after);
        if moved {
            game_state.board.spawn_random_tile();
        }
    }
}

fn remove_scene(
    commands: &mut Commands,
    query: &Query<Entity, With<Sprite>>,
    text_query: &Query<Entity, With<Text>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
    for entity in text_query.iter() {
        commands.entity(entity).despawn();
    }
}

fn render_background(commands: &mut Commands, x: f32, y: f32, color: Color) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(x, y, 0.1),
            scale: Vec3::splat(TILE_SIZE),
            ..Default::default()
        },
        ..Default::default()
    });
}

fn render_tile(commands: &mut Commands, x: f32, y: f32, value: u32, font_handle: Handle<Font>) {
    commands.spawn(Text2dBundle {
        text: Text::from_section(
            value.to_string(),
            TextStyle {
                font: font_handle,
                font_size: 40.0,
                color: Color::BLACK,
            },
        ),
        transform: Transform {
            translation: Vec3::new(x, y, 0.2),
            ..Default::default()
        },
        ..Default::default()
    });
}

fn render_board(
    mut commands: Commands,
    game_state: Res<GameState>,
    asset_server: Res<AssetServer>,
    query: Query<Entity, With<Sprite>>,
    text_query: Query<Entity, With<Text>>,
) {
    remove_scene(&mut commands, &query, &text_query);

    let font_handle = asset_server.load("Arial.ttf");

    let board_size = game_state.board.size as f32;
    let center_offset = - (board_size * TILE_SIZE + (board_size - 1.0) * TILE_GAP) / 2.0 + TILE_SIZE / 2.0;

    for r in 0..game_state.board.size {
        for c in 0..game_state.board.size {
            let tile = game_state.board.get(r, c);
            let x = c as f32 * (TILE_SIZE + TILE_GAP) + center_offset;
            let y = -(r as f32 * (TILE_SIZE + TILE_GAP) + center_offset);

            let bg_color = tile.color();
            render_background(&mut commands, x, y, bg_color);

            if let Tile::Number(value) = tile {
                render_tile(&mut commands, x, y, value, font_handle.clone());
            }
        }
    }
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            asset_folder: "assets".to_string(),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, handle_input)
        .add_systems(Update, render_board)
        .run();
}
