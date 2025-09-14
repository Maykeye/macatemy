use bevy::prelude::*;

use crate::game_state_plugin::{GameObject, GameState};

pub struct GameMapPlugin;

/// Observable event to move current layer renderer
#[derive(Event)]
pub struct ShiftActiveLayerEvent(pub isize);

/// A single cell on a game map.
/// TODO: use Entity?
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameMapCellFloor {
    None,
    Ground,
    Grass,
    Stone,
}

#[derive(Debug, Clone)]
pub struct GameMapCell {
    floor: GameMapCellFloor,
    floor_entity: Entity,
}

impl GameMapCell {
    pub fn new_empty() -> Self {
        Self {
            floor: GameMapCellFloor::None,
            floor_entity: Entity::PLACEHOLDER,
        }
    }
    pub fn from_floor(floor: GameMapCellFloor) -> Self {
        Self {
            floor,
            floor_entity: Entity::PLACEHOLDER,
        }
    }
}

#[derive(Component)]
pub struct GameMapLayerRenderer(usize);

pub type GameMapLayer = Vec<Vec<GameMapCell>>;

#[derive(Debug)]
pub struct GameMap {
    // Vector of layers [0..layers]
    // Where layer is a vecotor of rows [0..height]
    // Where row is a vector of GameMapCell [0..width]
    pub cells: Vec<GameMapLayer>,
    pub layers: usize,
    pub width: usize,
    pub height: usize,
}

impl GameMap {
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

    pub fn new_ground_layer(rows: usize, cols: usize) -> GameMapLayer {
        let rows = (0..rows)
            .map(|row_idx| {
                (0..cols)
                    .map(|col_idx| Self::make_ground_layer_floor(row_idx, col_idx))
                    .collect()
            })
            .collect();
        rows
    }

    pub fn new_empty_layer(rows: usize, cols: usize) -> GameMapLayer {
        (0..rows)
            .map(|_| (0..cols).map(|_| GameMapCell::new_empty()).collect())
            .collect()
    }

    pub fn new(layers: usize, rows: usize, cols: usize) -> Self {
        let mut map = vec![];
        map.push(Self::new_ground_layer(rows, cols));
        for _ in 1..layers {
            map.push(Self::new_empty_layer(rows, cols));
        }

        Self {
            cells: map,
            width: cols,
            height: rows,
            layers,
        }
    }
}

#[derive(Resource)]
struct GameMapData {
    grass: Vec<Handle<StandardMaterial>>,
    ground: Vec<Handle<StandardMaterial>>,
    stone: Vec<Handle<StandardMaterial>>,
    r#box: Handle<Mesh>,
    map: GameMap,
    current_layer: usize,
}

impl GameMapData {
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

impl FromWorld for GameMapData {
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
            map: GameMap::new(0, 0, 0),
            current_layer: 0,
        }
    }
}

fn spawn_map(
    mut commands: Commands,
    mut game_map_res: ResMut<GameMapData>,
    asset_server: Res<AssetServer>,
) {
    let mut map = GameMap::new(3, 10, 10);
    let mut children = vec![];

    for row_idx in 0..(map.height) {
        for col_idx in 0..(map.width) {
            let not_so_rng = row_idx * 17 + col_idx * 11;
            let materials = match map.cells[0][row_idx][col_idx].floor {
                GameMapCellFloor::None => continue,
                GameMapCellFloor::Ground => &game_map_res.ground,
                GameMapCellFloor::Grass => &game_map_res.grass,
                GameMapCellFloor::Stone => &game_map_res.stone,
            };
            let material = materials[not_so_rng % materials.len()].clone();
            let xf = col_idx as f32;
            let zf = row_idx as f32;
            let floor_entity = commands
                .spawn((
                    Name::new(format!("Floor#{row_idx}#{col_idx}")),
                    Mesh3d(game_map_res.r#box.clone()),
                    MeshMaterial3d(material),
                    Transform::from_xyz(xf, 0.0, zf),
                ))
                .id();
            map.cells[0][row_idx][col_idx].floor_entity = floor_entity;
            children.push(floor_entity);
        }
    }

    game_map_res.map = map;

    commands
        .spawn((
            GameObject,
            Name::new("Layer"),
            Transform::from_xyz(0.0, 0.0, 0.0),
            Visibility::default(),
            GameMapLayerRenderer(0),
        ))
        .add_children(&children);

    commands.spawn((
        Name::new("cat"),
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/cat3.glb"))),
        Transform::from_xyz(5.0, 0.0, 5.0),
    ));
}

fn shift_active_layer(
    ev: Trigger<ShiftActiveLayerEvent>,
    renderers: Query<&mut Transform, With<GameMapLayerRenderer>>,
    mut map_data: ResMut<GameMapData>,
) {
    let Some(next_current_layer) = map_data.current_layer.checked_add_signed(ev.0) else {
        return;
    };
    if next_current_layer >= map_data.map.layers {
        return;
    }
    map_data.current_layer = next_current_layer;

    let delta_y = ev.0 as f32;
    for mut renderer in renderers {
        renderer.translation.y -= delta_y;
    }
}

impl Plugin for GameMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_map.run_if(in_state(GameState::Init)));
        app.add_observer(shift_active_layer);
        app.init_resource::<GameMapData>();
    }
}

#[cfg(test)]
#[path = "tests/test_game_map_plugin.rs"]
mod test_game_map_plugin;
