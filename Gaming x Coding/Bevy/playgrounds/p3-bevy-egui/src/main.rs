use std::time::Duration;

use bevy::{
    prelude::*,
    render::{camera::Viewport, primitives::Frustum},
    window::PrimaryWindow,
    winit::WinitSettings,
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};

#[derive(Default, Resource)]
struct OccupiedScreenLogicalSpace {
    left: f32,
    top: f32,
    right: f32,
    bottom: f32,
}

const CAMERA_TARGET: Vec3 = Vec3::ZERO;

#[derive(Resource, Deref, DerefMut)]
struct OriginalCameraTransform(Transform);

fn main() {
    App::new()
        .insert_resource(WinitSettings::desktop_app())
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .init_resource::<OccupiedScreenLogicalSpace>()
        .add_systems(Startup, setup_system)
        .add_systems(Update, ui_example_system)
        .add_systems(Update, update_camera_transform_system)
        .run();
}

fn ui_example_system(
    mut contexts: EguiContexts,
    mut occupied_screen_space: ResMut<OccupiedScreenLogicalSpace>,
) {
    let ctx = contexts.ctx_mut();

    occupied_screen_space.left = egui::SidePanel::left("left_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Left resizeable panel");
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();
    occupied_screen_space.right = egui::SidePanel::right("right_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Right resizeable panel");
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();
    occupied_screen_space.top = egui::TopBottomPanel::top("top_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Top resizeable panel");
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .height();
    occupied_screen_space.bottom = egui::TopBottomPanel::bottom("bottom_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Bottom resizeable panel");
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .height();
}

fn setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(5.0, 5.0)),
        material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
        ..Default::default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(Color::srgb(0.8, 0.7, 0.6)),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    });
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    let camera_pos = Vec3::new(-2.0, 2.5, 5.0);
    let camera_transform =
        Transform::from_translation(camera_pos).looking_at(CAMERA_TARGET, Vec3::Y);
    commands.insert_resource(OriginalCameraTransform(camera_transform));

    commands.spawn(Camera3dBundle {
        transform: camera_transform,
        ..Default::default()
    });
}

fn update_camera_transform_system(
    occupied_screen_space: Res<OccupiedScreenLogicalSpace>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut camera: Query<&mut Camera>,
) {
    let window = windows.single();

    let aspect_ratio = 1280.0 / 720.0;
    let logical_width = window.width() - occupied_screen_space.left - occupied_screen_space.right;
    let logical_height = window.height() - occupied_screen_space.top - occupied_screen_space.bottom;
    let logical_size = if logical_width / logical_height < aspect_ratio {
        Vec2::new(logical_width, logical_width / aspect_ratio)
    } else {
        Vec2::new(logical_height * aspect_ratio, logical_height)
    };
    let logical_position = Vec2::new(
        occupied_screen_space.left + (logical_width - logical_size.x) / 2.0,
        occupied_screen_space.top + (logical_height - logical_size.y) / 2.0,
    );
    let mut physical_position = (logical_position * window.scale_factor()).as_uvec2();
    let mut physical_size = (logical_size * window.scale_factor()).as_uvec2();
    if physical_size.x * physical_size.y == 0 {
        physical_position = UVec2::ZERO;
        physical_size = UVec2::new(1, 1);
    }
    // println!("position: {:?} -> {:?}", logical_position, physical_position);
    // println!("size: {:?} -> {:?}", logical_size, physical_size);

    let mut camera = camera.get_single_mut().unwrap();
    camera.viewport = Some(Viewport {
        physical_position,
        physical_size,
        depth: 0.0..1.0,
    });
}
