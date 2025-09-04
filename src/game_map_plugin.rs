use bevy::prelude::*;

pub struct GameMapPlugin;

/// A single cell on a game map.
/// TODO: use Entity?
#[derive(Debug)]
enum GameMapCellFloor {
    None,
    Ground,
    Grass,
}

struct GameMapCell {
    floor: GameMapCellFloor,
    floor_entity: Entity,
}

#[derive(Component)]
pub struct GameMapLayer {
    rows: Vec<Vec<GameMapCell>>,
}

impl GameMapLayer {
    pub fn new(rows: usize, cols: usize) -> Self {
        let rows = (0..rows)
            .map(|z| {
                (0..cols)
                    .map(|_x| GameMapCell {
                        floor: if z == 1 {
                            GameMapCellFloor::Ground
                        } else {
                            GameMapCellFloor::Grass
                        },
                        floor_entity: Entity::PLACEHOLDER,
                    })
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

        // Chocolate: (7B3F00)
        (0..NUM_PER_TWEAK)
            .map(|i| {
                let i = i as f32;
                let delta = (i / (NUM_PER_TWEAK as f32)) * 0.25;
                let base_color = Color::linear_rgb(0.4823529411764706, 0.24705882352941178, delta);
                StandardMaterial {
                    base_color,
                    ..Default::default()
                }
            })
            .chain(
                // Gingerbread: (5E2C04)
                (0..NUM_PER_TWEAK).map(|i| {
                    let i = i as f32;
                    let delta = (i / (NUM_PER_TWEAK as f32)) * 0.15 - 0.05;
                    let base_color = Color::linear_rgb(
                        0.3686274509803922,
                        0.17254901960784313 - delta,
                        0.01568627450980392 + delta,
                    );
                    StandardMaterial {
                        base_color,
                        ..Default::default()
                    }
                }),
            )
            .chain(
                // Coffee: (6F4E37)
                (0..NUM_PER_TWEAK).map(|i| {
                    let i = i as f32;
                    let delta = (i / (NUM_PER_TWEAK as f32)) * 0.15 - 0.05;
                    let base_color = Color::linear_rgb(
                        0.43529411764705883,
                        0.3058823529411765 + delta,
                        0.21568627450980393 + delta,
                    );
                    StandardMaterial {
                        base_color,
                        ..Default::default()
                    }
                }),
            )
            .chain(
                // Brunette: (3B1E08)
                (0..NUM_PER_TWEAK).map(|i| {
                    let i = i as f32;
                    let delta = (i / (NUM_PER_TWEAK as f32)) * 0.10;
                    let base_color = Color::linear_rgb(
                        0.23137254901960785,
                        0.11764705882352941 + delta,
                        0.03137254901960784 + delta * 1.2,
                    );
                    StandardMaterial {
                        base_color,
                        ..Default::default()
                    }
                }),
            )
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
        Self {
            grass,
            ground,
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
            let material = match layer.rows[z][x].floor {
                GameMapCellFloor::None => continue,
                GameMapCellFloor::Ground => {
                    game_map_res.ground[not_so_rng % game_map_res.ground.len()].clone()
                }
                GameMapCellFloor::Grass => {
                    game_map_res.grass[not_so_rng % game_map_res.grass.len()].clone()
                }
            };
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
            Name::new("Layer"),
            Transform::from_xyz(0.0, 0.0, 0.0),
            Visibility::default(),
            layer,
        ))
        .add_children(&children);
}

impl Plugin for GameMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_map);
        app.init_resource::<GameMapResources>();
    }
}

#[cfg(test)]
#[path = "tests/test_game_map_plugin.rs"]
mod test_game_map_plugin;
