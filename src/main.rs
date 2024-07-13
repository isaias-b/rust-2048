use core::prelude::v1;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::{mem::swap, str::FromStr};

use action::Action;
use bevy::{
    asset::io::memory::Dir,
    core_pipeline::core_2d::Transparent2d,
    prelude::*,
    render::view::WindowSurfaces,
    scene::ron::value,
    sprite::Anchor,
    text::{BreakLineOn, Text2dBounds},
    transform::commands,
    utils::HashMap,
};
use board::Board;
use direction::Direction;
use tile::value::{Value, EMPTY_TILE_BG_COLOR};
use tile::{position::Position, Tile};

mod action;
mod board;
mod direction;
mod tile;

const TILE_SIZE: f32 = 100.0;
const TILE_GAP: f32 = 20.0;
const BG_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);
const FONT_PATH: &str =
    "/Users/isaiasbartelborth/Projects/isaias/rust/animated_2048/assets/Arial.ttf";

#[derive(Component, Debug, Deref, DerefMut)]
struct Animating {
    #[deref]
    timer: Timer,
    animation: Animation,
}

#[derive(Debug, Clone)]
enum Animation {
    Sliding {
        entity: Entity,
        tile: Tile,
        to: Position,
    },
    Merging {
        entity1: Entity,
        entity2: Entity,
        tile1: Tile,
        tile2: Tile,
        to: Tile,
    },
    Spawning {
        entity: Entity,
        tile: Tile,
    },
}

#[derive(Component, Clone, Debug)]
struct Transparency(f32);

#[derive(Component)]
struct SquareMarker;
#[derive(Component)]
struct TextMarker;

#[derive(Component)]
struct SquareId(Entity);
#[derive(Component)]
struct TextId(Entity);

#[derive(Bundle)]
struct TileBundle {
    position: Position,
    value: Value,
    square_id: SquareId,
    text_id: TextId,
    transparency: Transparency,
    spatial: SpatialBundle,
}

#[derive(Resource)]
struct GameState {
    board: Board,
    entities: HashMap<Position, Entity>,
    board_entity: Entity,
    deferred_events: Vec<Action>,
    replay: Vec<Direction>,
    rng: ChaCha8Rng,
}

impl GameState {
    fn get_entity(&self, pos: &Position) -> Option<&Entity> {
        self.entities.get(pos)
    }
    fn move_entity(&mut self, from: &Position, to: &Position) {
        if let Some(entity) = self.entities.remove(from) {
            self.entities.insert(to.clone(), entity);
        } else {
            panic!("no entity found at position {:?}", from);
        }
    }
}

// impl Default for GameState {
//     // fn default() -> Self {
//     //     Self {
//     //         board: Board::new(4),
//     //         entities: HashMap::new(),
//     //         deferred_events: Vec::new(),
//     //     }
//     // }
// }

fn to_screen(pos: &Position) -> Vec2 {
    Vec2::new(
        pos.col as f32 * (TILE_SIZE + TILE_GAP),
        pos.row as f32 * (TILE_SIZE + TILE_GAP) * -1.0,
    )
}

fn spawn_tile(
    commands: &mut Commands,
    font: &Handle<Font>,
    pos: &Position,
    value: &Value,
) -> Entity {
    let text = Text2dBundle {
        text: Text::from_section(
            value.text_value(),
            // pos.to_string(),
            TextStyle {
                font: font.clone(),
                font_size: 40.0,
                // color: value.text_color(),
                color: Color::BLACK,
            },
        ),
        text_anchor: Anchor::Center,
        transform: Transform {
            translation: Vec2::ZERO.extend(0.3),
            ..Default::default()
        },
        ..Default::default()
    };
    let text_id = commands
        .spawn(text)
        .insert(TextMarker)
        .insert(Transparency(value.transparency_value()))
        .id();

    let square = SpriteBundle {
        sprite: Sprite {
            color: value.tile_color(),
            anchor: Anchor::Center,
            rect: Some(Rect {
                min: Vec2::new(0.0, 0.0),
                max: Vec2::new(TILE_SIZE, TILE_SIZE),
            }),
            ..Default::default()
        },
        transform: Transform {
            translation: Vec2::ZERO.extend(0.2),
            ..Default::default()
        },
        ..Default::default()
    };

    let square_id = commands
        .spawn(square)
        .insert(SquareMarker)
        .insert(Transparency(value.transparency_value()).clone())
        .id();

    let tile = TileBundle {
        position: pos.clone(),
        value: value.clone(),
        square_id: SquareId(square_id),
        text_id: TextId(text_id),
        transparency: Transparency(value.transparency_value()),
        spatial: SpatialBundle {
            transform: Transform {
                translation: to_screen(pos).extend(0.0),
                scale: Vec3::ZERO,

                ..Default::default()
            },
            ..Default::default()
        },
    };
    let tile_id = commands
        .spawn(tile)
        .add_child(square_id)
        .add_child(text_id)
        .id();

    return tile_id;
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    let font = asset_server.load(FONT_PATH);
    let headline = Text2dBundle {
        text: Text::from_section(
            "2048",
            TextStyle {
                font: font.clone(),
                font_size: 80.0,
                color: Color::BLACK,
            },
        ),
        text_anchor: Anchor::TopCenter,
        transform: Transform {
            translation: Vec2::new(0.0, 400.0 - 40.0).extend(0.1),
            ..Default::default()
        },
        ..Default::default()
    };
    commands.spawn(headline);

    // let mut game = GameState::default();
    // game.board = Board::from_str("1110000000000000").unwrap();
    // game.board.spawn_random_tile();
    // game.board.spawn_random_tile();

    let mut board = Board::from_str("3301100000000010").unwrap();
    // let mut replay = vec![
    //     Direction::Right,
    //     Direction::Up,
    //     Direction::Left,
    //     Direction::Down,
    //     Direction::Left,
    //     Direction::Up,
    //     Direction::Up,
    //     Direction::Left,
    //     Direction::Down,
    //     Direction::Left,
    //     Direction::Down,
    //     Direction::Down,
    //     Direction::Left,
    //     Direction::Left,
    //     Direction::Down,
    //     Direction::Right,
    //     Direction::Down,
    //     Direction::Right,
    //     Direction::Down,
    //     Direction::Left,
    //     Direction::Down,
    //     Direction::Right,
    //     Direction::Right,
    //     Direction::Right,
    //     Direction::Left,
    //     Direction::Right,
    //     Direction::Right,
    //     Direction::Right,
    //     Direction::Left,
    //     Direction::Down,
    //     Direction::Right,
    //     Direction::Down,
    //     Direction::Left,
    //     Direction::Down,
    //     Direction::Down,
    //     Direction::Left,
    //     Direction::Left,
    //     Direction::Down,
    //     Direction::Left,
    //     Direction::Left,
    //     Direction::Left,
    //     Direction::Down,
    //     Direction::Left,
    //     Direction::Left,
    //     Direction::Down,
    //     Direction::Down,
    //     Direction::Left,
    //     Direction::Left,
    //     Direction::Down,
    // ];
    let mut replay = vec![
        Direction::Right,
        Direction::Up,
        Direction::Right,
        Direction::Up,
        Direction::Left,
        Direction::Up,
        Direction::Up,
        Direction::Left,
        Direction::Up,
        Direction::Left,
        Direction::Up,
        Direction::Up,
    ];
    replay.reverse();
    // let mut board = Board::from_str("2300200010010000").unwrap();
    // let mut board = Board::from_str("1110000000000000").unwrap();

    // let mut board = Board::from_str("5321332020000000").unwrap();
    let mut entities = HashMap::new();
    let size = board.size as f32;
    let offset = (TILE_SIZE + TILE_GAP) * (size - 1.0) * 0.5;
    let offset_vec = Vec2::new(-offset, offset);

    let mut tile_ids = Vec::new();
    let mut empty_ids = Vec::new();

    println!("now rendering board...");
    let traversal = board.traversal_map.get(&Direction::Left).unwrap();
    for line in traversal.iter() {
        for pos in line.iter() {
            let vec = to_screen(pos);

            let empty = SpriteBundle {
                sprite: Sprite {
                    color: EMPTY_TILE_BG_COLOR,
                    anchor: Anchor::Center,
                    rect: Some(Rect {
                        min: Vec2::new(0.0, 0.0),
                        max: Vec2::new(TILE_SIZE, TILE_SIZE),
                    }),
                    ..Default::default()
                },
                transform: Transform {
                    translation: vec.extend(0.1),
                    ..Default::default()
                },
                ..Default::default()
            };
            empty_ids.push(commands.spawn(empty).id());

            let tile = board.get_tile(pos);

            if let Value::Empty = tile.value {
                continue;
            }

            let tile_id = spawn_tile(&mut commands, &font, pos, &tile.value);

            let duration = 0.1;
            let timer = Timer::from_seconds(duration, TimerMode::Once);
            commands.entity(tile_id).insert(Animating {
                timer,
                animation: Animation::Spawning {
                    entity: tile_id,
                    tile: tile.clone(),
                },
            });

            entities.insert(pos.clone(), tile_id);
            tile_ids.push(tile_id);
        }
    }

    let board_bundle = SpatialBundle {
        transform: Transform {
            translation: offset_vec.extend(0.0),
            ..Default::default()
        },
        ..Default::default()
    };
    let board_entity = commands
        .spawn(board_bundle)
        .push_children(&empty_ids)
        .push_children(&tile_ids)
        .id();

    // println!("done rendering board, now spawning 2 random tiles...");
    let game = GameState {
        board,
        entities,
        board_entity,
        deferred_events: Vec::new(),
        rng: ChaCha8Rng::from_seed([0; 32]),
        replay,
    };
    commands.insert_resource(game);
}

fn handle_input(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<GameState>,
    mut event_writer: EventWriter<Action>,
) {
    if !game_state.deferred_events.is_empty() {
        return;
    }
    let direction = match () {
        _ if keys.just_pressed(KeyCode::ArrowLeft) => Some(Direction::Left),
        _ if keys.just_pressed(KeyCode::ArrowRight) => Some(Direction::Right),
        _ if keys.just_pressed(KeyCode::ArrowUp) => Some(Direction::Up),
        _ if keys.just_pressed(KeyCode::ArrowDown) => Some(Direction::Down),
        _ if keys.just_pressed(KeyCode::Space) => {
            if let Some(direction) = game_state.replay.pop() {
                Some(direction)
            } else {
                None
            }
        }
        _ => None,
    };

    if let Some(direction) = direction {
        // commands.spawn(direction);
        let events = game_state.board.plan_slide_and_merge(&direction);
        // println!();
        // for event in events.iter() {
        //     println!("{:?}", event);
        // }
        // println!();
        event_writer.send_batch(events.iter().cloned());

        let mut g = game_state.as_mut();
        let before: String = g.board.to_string();
        for event in events.iter() {
            g.board.apply(event.clone());
        }
        let moved = events.len() > 0;
        let after: String = g.board.to_string();
        println!("{} --{}--> {}", before, direction, after);
        if moved {
            let spawn = g.board.plan_spawn_random_tile(&mut g.rng);
            if let Some(spawn) = spawn {
                g.deferred_events.push(spawn);
            }
        }
    }
}

fn check_animations(
    mut commands: Commands,
    mut game: ResMut<GameState>,
    mut event_writer: EventWriter<Action>,
    query: Query<&Animating>,
) {
    if query.iter().count() > 0 {
        return;
    }
    if !game.deferred_events.is_empty() {
        event_writer.send_batch(game.deferred_events.iter().cloned());
        game.deferred_events.clear();
    }
}

// fn propagate_transparency_to_children(
//     mut children: Query<(&Children, &mut Transparency), Changed<Transparency>>,
//     mut commands: Commands,
// ) {
//     for (children, mut transparency) in children.iter_mut() {
//         for child in children.iter() {
//             commands.entity(*child).insert(Transparency(transparency.0));
//         }
//     }
// }

fn update_transparency(
    mut materials: ResMut<Assets<ColorMaterial>>,
    sprites: Query<(&Transparency, &Handle<ColorMaterial>), Changed<Transparency>>,
    mut texts: Query<(&Transparency, &mut Text), Changed<Transparency>>,
) {
    for (transparency, handle) in sprites.iter() {
        let material = materials.get_mut(handle).unwrap();
        material.color.set_a(transparency.0);
    }
    for (transparency, mut text) in texts.iter_mut() {
        for section in &mut text.sections {
            section.style.color.set_a(transparency.0);
        }
    }
}

fn start_animate(
    mut commands: Commands,
    mut event_reader: EventReader<Action>,
    asset_server: Res<AssetServer>,
    mut game: ResMut<GameState>,
) {
    let font = asset_server.load(FONT_PATH);
    for action in event_reader.read() {
        println!("start animate: {:?}", action);
        match action {
            Action::SlideTile(tile, new_pos) => {
                let duration = 0.1;
                let timer = Timer::from_seconds(duration, TimerMode::Once);
                let entity = game.get_entity(&tile.position).unwrap();
                let tile = game.board.get_tile(&tile.position);
                commands.entity(*entity).insert(Animating {
                    timer,
                    animation: Animation::Sliding {
                        entity: *entity,
                        tile: tile,
                        to: new_pos.clone(),
                    },
                });
            }
            Action::MergeTiles(tile1, tile2, new_pos, new_val) => {
                let duration = 0.1;
                let timer1 = Timer::from_seconds(duration, TimerMode::Once);
                let timer2 = Timer::from_seconds(duration, TimerMode::Once);
                let e1 = game.get_entity(&tile1.position).unwrap();
                let e2 = game.get_entity(&tile2.position).unwrap();
                let tile1 = game.board.get_tile(&tile1.position);
                let tile2 = game.board.get_tile(&tile2.position);
                commands.entity(*e1).insert(Animating {
                    timer: timer1,
                    animation: Animation::Merging {
                        entity1: *e1,
                        entity2: *e2,
                        tile1: tile1.clone(),
                        tile2: tile2.clone(),
                        to: Tile {
                            position: new_pos.clone(),
                            value: new_val.clone(),
                        },
                    },
                });
                commands.entity(*e2).insert(Animating {
                    timer: timer2,
                    animation: Animation::Merging {
                        entity1: *e1,
                        entity2: *e2,
                        tile1: tile1.clone(),
                        tile2: tile2.clone(),
                        to: Tile {
                            position: new_pos.clone(),
                            value: new_val.clone(),
                        },
                    },
                });
            }
            Action::SpawnRandomTile(tile) => {
                let duration = 0.1;
                let timer = Timer::from_seconds(duration, TimerMode::Once);
                let entity = spawn_tile(&mut commands, &font, &tile.position, &tile.value);
                commands.entity(entity).insert(Animating {
                    timer,
                    animation: Animation::Spawning {
                        entity: entity,
                        tile: tile.clone(),
                    },
                });
                game.as_mut().entities.insert(tile.position.clone(), entity);
                game.as_mut().board.apply(action.clone());
                commands.entity(game.board_entity).add_child(entity);
            }
        }
    }
}
fn update_animations(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &mut Position,
        &mut Transform,
        &mut Animating,
        &SquareId,
        &TextId,
    )>,
    mut squares: Query<(Entity, &SquareMarker, &mut Transparency), Without<TextMarker>>,
    mut texts: Query<(Entity, &TextMarker, &mut Transparency, &mut Text), Without<SquareMarker>>,
    mut game: ResMut<GameState>,
) {
    for (entity, _, mut transform, mut animating, square_id, text_id) in query.iter_mut() {
        animating.timer.tick(time.delta());
        let t = animating.timer.fraction();
        let animation = &animating.animation;
        let mut game = game.as_mut();
        match animation {
            Animation::Sliding { entity, tile, to } => {
                let old_pos = transform.translation;
                let new_pos = to_screen(to).extend(old_pos.z);
                transform.translation = old_pos.lerp(new_pos, t);
            }
            Animation::Merging {
                entity1,
                entity2,
                tile1,
                tile2,
                to,
            } => {
                let is_target = entity == *entity1;
                // perform merge animation on the target entity by bouncing the scaling it up and down on the last 20% of the animation
                // perform fade out animation on the other entity and remove it
                let old_pos = transform.translation;
                let new_pos = to_screen(&to.position).extend(old_pos.z);
                transform.translation = old_pos.lerp(new_pos, t);
                if is_target {
                    transform.scale = Vec3::splat(if t > 0.8 {
                        1.0 + 0.2 * ((((t - 0.8) / 0.2) * std::f32::consts::PI).sin())
                    } else {
                        1.0
                    });
                } else {
                    transform.scale = Vec3::splat(if t < 0.8 {
                        (1.0 - t) / 0.8 * 0.5
                    } else {
                        (1.0 - t) / 0.2 * 0.5
                    });
                }
            }
            Animation::Spawning { entity, tile } => {
                let old_scale = Vec3::ZERO;
                let new_scale = Vec3::ONE;
                transform.scale = old_scale.lerp(new_scale, t);
            }
        }
        if animating.timer.finished() {
            commands.entity(entity).remove::<Animating>();

            match animation {
                Animation::Sliding {
                    entity,
                    tile,
                    to: new_pos,
                } => {
                    commands.entity(*entity).insert(new_pos.clone());
                    game.move_entity(&tile.position, new_pos);
                }
                Animation::Merging {
                    entity1,
                    entity2,
                    tile1,
                    tile2,
                    to: to,
                } => {
                    let new_pos = to.position;
                    let new_val = to.value;
                    let is_target = entity == *entity1;
                    if is_target {
                        println!("merging {} and {} to {}", tile1.value, tile2.value, new_val);
                        commands
                            .entity(*entity1)
                            .insert(new_pos.clone())
                            .insert(new_val.clone());
                        texts.get_mut(text_id.0).unwrap().3.sections[0].value =
                            new_val.text_value();
                        game.move_entity(&tile1.position, &new_pos);
                    } else {
                        commands
                            .entity(*entity2)
                            .remove_children(&[text_id.0, square_id.0])
                            .despawn();
                        commands.entity(text_id.0).despawn();
                        commands.entity(square_id.0).despawn();
                        game.entities.remove(&tile2.position);
                    }
                }
                Animation::Spawning { entity, tile } => {
                    transform.scale = Vec3::ONE;
                }
            }
        }
    }
}

fn swap_keys<K, V>(map: &mut HashMap<K, V>, key1: K, key2: K)
where
    K: std::hash::Hash + Eq,
    V: Copy,
{
    if let (Some(&value1), Some(&value2)) = (map.get(&key1).clone(), map.get(&key2).clone()) {
        map.insert(key1, value2);
        map.insert(key2, value1);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            file_path: "assets".to_string(),
            ..default()
        }))
        .insert_resource(ClearColor(BG_COLOR))
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                handle_input,
                start_animate,
                update_animations,
                check_animations,
            )
                .chain(),
        )
        .add_event::<Action>()
        .run();
}
