use bevy::{
    color::palettes::tailwind::*,
    ecs::world::Command,
    pbr::wireframe::{Wireframe, WireframePlugin},
    prelude::*,
    render::{
        mesh::VertexAttributeValues,
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
        settings::{RenderCreation, WgpuFeatures, WgpuSettings},
        RenderPlugin,
    },
    tasks::{block_on, futures_lite::future, AsyncComputeTaskPool, Task},
    utils::HashMap,
};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use noise::{BasicMulti, NoiseFn, Perlin, Seedable};
use std::f32::consts::PI;

fn main() {
    App::new()
        .init_resource::<TerrainStore>()
        .init_resource::<GeneratingChunk>()
        .add_plugins((
            DefaultPlugins.set(RenderPlugin {
                render_creation: RenderCreation::Automatic(WgpuSettings {
                    // WARN this is a native only feature. It will not work with webgl or webgpu
                    features: WgpuFeatures::POLYGON_MODE_LINE,
                    ..default()
                }),
                ..default()
            }),
            // You need to add this plugin to enable wireframe rendering
            WireframePlugin,
            PanOrbitCameraPlugin,
        ))
        .add_systems(Startup, startup)
        .add_systems(Update, toggle_wireframe)
        .add_systems(
            Update,
            (
                player_control_system,
                sync_player_camera_system,
                chunk_manage_system,
                receive_generated_chunk_system,
            ),
        )
        .run();
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::default()),
            material: debug_material.clone(),
            transform: Transform::from_xyz(0., 50., 0.),
            ..default()
        },
        Player,
    ));
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 20., 75.0)
                .looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
            ..default()
        },
        PanOrbitCamera::default(),
        PlayerCam,
    ));

    // directional 'sun' light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        ..default()
    });
}

#[derive(Resource, Default)]
struct TerrainStore(HashMap<IVec2, Handle<Mesh>>);

#[derive(Resource, Default)]
struct GeneratingChunk(HashMap<IVec2, Task<Mesh>>);

fn receive_generated_chunk_system(
    mut commands: Commands,
    mut generating_chunk: ResMut<GeneratingChunk>,
    mut terrain_store: ResMut<TerrainStore>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    generating_chunk.0.retain(|chunk, task| {
        let status = block_on(future::poll_once(task));
        let retain = status.is_none();

        if let Some(chunk_mesh) = status {
            let mesh_size = 1000.;

            let mesh = meshes.add(chunk_mesh);
            let material = materials.add(Color::WHITE);

            terrain_store.0.insert(*chunk, mesh.clone());
            commands.spawn((
                PbrBundle {
                    mesh,
                    material,
                    transform: Transform::from_xyz(
                        chunk.x as f32 * mesh_size,
                        0.,
                        chunk.y as f32 * mesh_size,
                    ),
                    ..default()
                },
                Terrain,
            ));
        }

        retain
    });
}

struct SpawnTerrain(IVec2);

fn generate_chunk(chunk: IVec2) -> Mesh {
    let mesh_size = 1000.;
    let terrain_height = 100.;
    let noise = BasicMulti::<Perlin>::new(900);

    let mut terrain = Mesh::from(
        Plane3d::default()
            .mesh()
            .size(mesh_size, mesh_size)
            .subdivisions(1000),
    );

    if let Some(VertexAttributeValues::Float32x3(positions)) =
        terrain.attribute_mut(Mesh::ATTRIBUTE_POSITION)
    {
        for pos in positions.iter_mut() {
            let val = noise.get([
                (pos[0] as f64 + (mesh_size as f64 * chunk.x as f64)) / 300.,
                (pos[2] as f64 + (mesh_size as f64 * chunk.y as f64)) / 300.,
            ]);

            pos[1] = val as f32 * terrain_height;
        }

        let colors: Vec<[f32; 4]> = positions
            .iter()
            .map(|[_, g, _]| {
                let g = *g / terrain_height * 2.;

                if g > 0.8 {
                    (Color::LinearRgba(LinearRgba {
                        red: 20.,
                        green: 20.,
                        blue: 20.,
                        alpha: 1.,
                    }))
                    .to_linear()
                    .to_f32_array()
                } else if g > 0.3 {
                    Color::from(AMBER_800).to_linear().to_f32_array()
                } else if g < -0.8 {
                    Color::BLACK.to_linear().to_f32_array()
                } else {
                    (Color::from(GREEN_400).to_linear()).to_f32_array()
                }
            })
            .collect();
        terrain.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    }
    terrain.compute_normals();

    terrain
}

impl Command for SpawnTerrain {
    fn apply(self, world: &mut World) {
        if world
            .get_resource_mut::<TerrainStore>()
            .expect("TerrainStore to be available")
            .0
            .get(&self.0)
            .is_some()
        {
            // mesh already exists
            // do nothing for now
            warn!("mesh {} already exists", self.0);
            return;
        };

        let mut generating_chunk = world
            .get_resource_mut::<GeneratingChunk>()
            .expect("GeneratingChunk to be available");

        if generating_chunk.0.get(&self.0).is_some() {
            warn!("mesh {} is already generating", self.0);
            return;
        }

        let task_pool = AsyncComputeTaskPool::get();
        let task = task_pool.spawn(async move { generate_chunk(self.0) });
        generating_chunk.0.insert(self.0, task);
    }
}

fn chunk_manage_system(
    mut commands: Commands,
    mut current_chunk: Local<IVec2>,
    player: Query<&Transform, With<Player>>,
    mut terrain_store: ResMut<TerrainStore>,
    terrain_entities: Query<(Entity, &Handle<Mesh>), With<Terrain>>,
) {
    // same as mesh_size for us
    let chunk_size = 1000.;

    let Ok(transform) = player.get_single() else {
        warn!("no player!");
        return;
    };

    // Convert from world location to chunk index
    let xz = (transform.translation.xz() / chunk_size).trunc().as_ivec2();

    if *current_chunk != xz || terrain_store.0.get(&xz).is_none() {
        *current_chunk = xz;
        let chunks_to_render = [
            *current_chunk + IVec2::new(-1, -1),
            *current_chunk + IVec2::new(-1, 0),
            *current_chunk + IVec2::new(-1, 1),
            *current_chunk + IVec2::new(0, -1),
            *current_chunk + IVec2::new(0, 0),
            *current_chunk + IVec2::new(0, 1),
            *current_chunk + IVec2::new(1, -1),
            *current_chunk + IVec2::new(1, 0),
            *current_chunk + IVec2::new(1, 1),
        ];
        let chunks_to_despawn: Vec<(IVec2, Handle<Mesh>)> = terrain_store
            .0
            .extract_if(|key, _| !chunks_to_render.contains(&key))
            .collect();

        chunks_to_despawn
            .iter()
            .filter_map(|(chunk, handle)| {
                terrain_entities
                    .iter()
                    .find(|(_, h)| h == &handle)
                    .map(|(entity, _)| (entity, chunk))
            })
            .for_each(|(entity, chunk)| {
                commands.entity(entity).despawn_recursive();
                terrain_store.0.remove(chunk);
            });

        for chunk in chunks_to_render {
            commands.add(SpawnTerrain(chunk));
        }
    }
}

#[derive(Component)]
struct Terrain;

fn toggle_wireframe(
    mut commands: Commands,
    landscapes_wireframes: Query<Entity, (With<Terrain>, With<Wireframe>)>,
    landscapes: Query<Entity, (With<Terrain>, Without<Wireframe>)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Space) {
        for terrain in &landscapes {
            commands.entity(terrain).insert(Wireframe);
        }
        for terrain in &landscapes_wireframes {
            commands.entity(terrain).remove::<Wireframe>();
        }
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct PlayerCam;

fn player_control_system(
    input: Res<ButtonInput<KeyCode>>,
    mut players: Query<&mut Transform, With<Player>>,
) {
    let mut direction = Vec2::new(0., 0.);
    if input.pressed(KeyCode::KeyW) {
        direction.y += 1.;
    }
    if input.pressed(KeyCode::KeyS) {
        direction.y -= 1.;
    }
    if input.pressed(KeyCode::KeyA) {
        direction.x += 1.;
    }
    if input.pressed(KeyCode::KeyD) {
        direction.x -= 1.;
    }
    if input.pressed(KeyCode::ShiftLeft) {
        direction *= 4.;
    }
    for mut player in &mut players {
        player.translation.x += direction.x * 2.;
        player.translation.z += direction.y * 2.;
    }
}

fn sync_player_camera_system(
    players: Query<&Transform, (With<Player>, Without<PlayerCam>)>,
    mut camera: Query<&mut PanOrbitCamera, With<PlayerCam>>,
) {
    let Ok(player) = players.get_single() else {
        return;
    };
    let mut orbit = camera.single_mut();

    orbit.target_focus = Vec3::new(
        player.translation.x,
        player.translation.y,
        player.translation.z,
    );
}

fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    )
}
