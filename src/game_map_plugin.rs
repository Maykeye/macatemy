use bevy::prelude::*;

use crate::game_state_plugin::{GameObject, GameState};

pub struct GameMapPlugin;

/// A single cell on a game map.
/// TODO: use Entity?
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum GameMapCellFloor {
    None,
    Ground,
    Grass,
    Stone,
}

#[derive(Debug, Clone)]
struct GameMapCell {
    floor: GameMapCellFloor,
    floor_entity: Entity,
}

impl GameMapCell {
    pub fn from_floor(floor: GameMapCellFloor) -> Self {
        Self {
            floor,
            floor_entity: Entity::PLACEHOLDER,
        }
    }
}

#[derive(Component)]
pub struct GameMapLayer {
    rows: Vec<Vec<GameMapCell>>,
}

impl GameMapLayer {
    fn make_ground_layer_floor(row_idx: usize, col_idx: usize) -> GameMapCell {
        use GameMapCellFloor::*;
        GameMapCell::from_floor(match row_idx {
            1 => Stone,
            2 => Ground,
            3 => {
                let divider_blocks = [Ground, Ground, Stone, Stone];
                divider_blocks[col_idx % divider_blocks.len()]
            }
            4 => Ground,
            5 => Stone,
            _ => Grass,
        })
    }

    pub fn new(rows: usize, cols: usize) -> Self {
        let rows = (0..rows)
            .map(|row_idx| {
                (0..cols)
                    .map(|col_idx| Self::make_ground_layer_floor(row_idx, col_idx))
                    .collect()
            })
            .collect();
        Self { rows }
    }
}

#[derive(Resource)]
struct GameMapResources {
    grass: Vec<Handle<StandardMaterial>>,
    ground: Vec<Handle<StandardMaterial>>,
    stone: Vec<Handle<StandardMaterial>>,
    r#box: Handle<Mesh>,
}

impl GameMapResources {
    fn init_grass_mat() -> Vec<StandardMaterial> {
        const NUM: usize = 16;
        (1..NUM)
            .map(|i| {
                let i = i as f32;
                let d = (i / (NUM as f32)) * 0.5;
                let base_color = Color::linear_rgb(0.5 + d, 1.0, 0.25 + d);
                StandardMaterial {
                    base_color,
                    ..Default::default()
                }
            })
            .collect()
    }

    fn init_ground() -> Vec<StandardMaterial> {
        const NUM_PER_TWEAK: usize = 4;

        fn make_ground(i: usize, r: u32, g: u32, b: u32) -> StandardMaterial {
            let (r, g, b) = ((r as f32) / 255.0, (g as f32) / 255.0, (b as f32) / 255.0);
            let i = i as f32;
            let delta = (i / (NUM_PER_TWEAK as f32)) * 0.15;
            let base_color = Color::linear_rgb(r, g, b + delta);
            StandardMaterial {
                base_color,
                ..Default::default()
            }
        }

        [].into_iter()
            .chain((0..NUM_PER_TWEAK).map(|i| {
                // Gingerbread
                make_ground(i, 0x5E, 0x2C, 0x04)
            }))
            .chain(
                // Brunette
                (0..NUM_PER_TWEAK).map(|i| make_ground(i, 0x3B, 0x1E, 0x08)),
            )
            .collect()
    }

    fn init_stone() -> Vec<StandardMaterial> {
        const NUM_PER_TWEAK: usize = 4;
        fn make_stone(i: usize, r: f32, g: f32, b: f32) -> StandardMaterial {
            let i = i as f32;
            let delta = (i / (NUM_PER_TWEAK as f32)) * 0.16 - 0.08;
            let base_color = Color::linear_rgb(0.5 + delta * r, 0.5 + delta * g, 0.5 + delta * b);
            StandardMaterial {
                base_color,
                ..Default::default()
            }
        }

        (0..NUM_PER_TWEAK)
            .map(|i| make_stone(i, 1.0, 1.0, 1.0))
            .chain((0..NUM_PER_TWEAK).map(|i| make_stone(i, 1.0, 0.2, 0.2)))
            .chain((0..NUM_PER_TWEAK).map(|i| make_stone(i, 0.0, 1.0, 0.2)))
            .chain((0..NUM_PER_TWEAK).map(|i| make_stone(i, 0.1, -0.1, 1.0)))
            .collect()
    }
}

impl FromWorld for GameMapResources {
    fn from_world(world: &mut World) -> Self {
        let mut meshes = world.resource_mut::<Assets<Mesh>>();
        let box_mesh = meshes.add(Cuboid::from_length(1.0));

        let mut material_assets = world.resource_mut::<Assets<StandardMaterial>>();

        let grass = Self::init_grass_mat()
            .into_iter()
            .map(|mat| material_assets.add(mat))
            .collect();
        let ground = Self::init_ground()
            .into_iter()
            .map(|mat| material_assets.add(mat))
            .collect();
        let stone = Self::init_stone()
            .into_iter()
            .map(|mat| material_assets.add(mat))
            .collect();
        Self {
            grass,
            ground,
            stone,
            r#box: box_mesh,
        }
    }
}

fn spawn_map(mut commands: Commands, game_map_res: Res<GameMapResources>) {
    let mut layer = GameMapLayer::new(10, 10);
    let mut children = vec![];

    for z in 0..10 {
        for x in 0..10 {
            let not_so_rng = z * 17 + x * 11;
            let materials = match layer.rows[z][x].floor {
                GameMapCellFloor::None => continue,
                GameMapCellFloor::Ground => &game_map_res.ground,
                GameMapCellFloor::Grass => &game_map_res.grass,
                GameMapCellFloor::Stone => &game_map_res.stone,
            };
            let material = materials[not_so_rng % materials.len()].clone();
            let xf = x as f32;
            let zf = z as f32;
            let entity = commands
                .spawn((
                    Name::new(format!("Floor#{z}#{x}")),
                    Mesh3d(game_map_res.r#box.clone()),
                    MeshMaterial3d(material),
                    Transform::from_xyz(xf, 0.0, zf),
                ))
                .id();
            layer.rows[z][x].floor_entity = entity;
            children.push(entity);
        }
    }
    commands
        .spawn((
            GameObject,
            Name::new("Layer"),
            Transform::from_xyz(0.0, 0.0, 0.0),
            Visibility::default(),
            layer,
        ))
        .add_children(&children);
}

impl Plugin for GameMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_map.run_if(in_state(GameState::Init)));
        app.init_resource::<GameMapResources>();
    }
}

#[cfg(test)]
#[path = "tests/test_game_map_plugin.rs"]
mod test_game_map_plugin;
