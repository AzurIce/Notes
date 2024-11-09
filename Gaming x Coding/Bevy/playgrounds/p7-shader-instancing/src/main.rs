//! A shader that renders a mesh multiple times in one draw call.

use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    prelude::*,
    render::{
        camera::{RenderTarget, ScalingMode},
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        texture::BevyDefault,
    },
};
use p7_shader_instancing::{CustomMaterialPlugin, Frame, MeshHandle};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins(CustomMaterialPlugin)
        .add_plugins(FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_config: TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                    font: default(),
                },
            },
        });
    app.add_systems(Startup, setup).run();
    // app.add_systems(Startup, setup_simple).run();
}

const MAX_SIZE: u32 = 600;
const MAX_WIDTH: u32 = 200;
const MAX_HEIGHT: u32 = 200;

fn setup_simple(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Cuboid::new(0.5, 0.5, 0.5));
    for x in 1..=MAX_SIZE {
        for y in 1..=MAX_SIZE {
            let x = x as f32 / MAX_SIZE as f32;
            let y = y as f32 / MAX_SIZE as f32;

            commands.spawn(PbrBundle {
                mesh: mesh.clone(),
                transform: Transform::from_xyz(
                    x * MAX_SIZE as f32 - MAX_SIZE as f32 / 2.0,
                    y * MAX_SIZE as f32 - MAX_SIZE as f32 / 2.0,
                    0.0,
                ),
                material: materials.add(Color::hsla(x as f32 * 360., y as f32, 0.5, 1.0)),
                ..Default::default()
            });
        }
    }

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 30.0).looking_at(Vec3::ZERO, Vec3::Y),
        projection: Projection::Perspective(PerspectiveProjection {
            fov: 10.0,
            ..Default::default()
        }),
        ..Default::default()
    });
}

fn setup(mut commands: Commands, mesh_handle: Res<MeshHandle>, mut images: ResMut<Assets<Image>>) {
    let frame = commands
        .spawn((
            mesh_handle.0.clone(),
            SpatialBundle::INHERITED_IDENTITY,
            Frame,
        ))
        .id();
    let events = (1..=MAX_WIDTH)
        .flat_map(|x| {
            (1..=MAX_HEIGHT)
                .map(move |y| (x as f32 / MAX_WIDTH as f32, y as f32 / MAX_HEIGHT as f32))
        })
        .map(|(x, y)| {
            commands
                .spawn(p7_shader_instancing::Event {
                    position: Vec3::new(
                        x * MAX_WIDTH as f32 - MAX_WIDTH as f32 / 2.0,
                        y * MAX_HEIGHT as f32 - MAX_HEIGHT as f32 / 2.0,
                        0.0,
                    ),
                    color: LinearRgba::from(Color::hsla(x * 360., y, 0.5, 1.0)).to_f32_array(),
                })
                .id()
        })
        .collect::<Vec<_>>();
    commands.entity(frame).replace_children(&events);

    let size = Extent3d {
        width: MAX_HEIGHT,
        height: MAX_WIDTH,
        depth_or_array_layers: 1,
    };

    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::bevy_default(), // Bgra8UnormSrgb
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..Default::default()
    };
    image.resize(size);

    let image = images.add(image);

    // camera
    commands.spawn(Camera3dBundle {
        camera: Camera {
            target: RenderTarget::Image(image.clone()),
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 1000.0).looking_at(Vec3::ZERO, Vec3::Y),
        projection: Projection::Orthographic(OrthographicProjection {
            near: -1000.0,
            scaling_mode: ScalingMode::AutoMin {
                min_width: MAX_HEIGHT as f32,
                min_height: MAX_WIDTH as f32,
            },
            ..Default::default()
        }),
        ..Default::default()
    });

    commands.spawn(SpriteBundle {
        texture: image,
        ..Default::default()
    });
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            near: -1000.0,
            scaling_mode: ScalingMode::AutoMin {
                min_width: MAX_HEIGHT as f32,
                min_height: MAX_WIDTH as f32,
            },
            ..Default::default()
        },
        ..Default::default()
    });
}
