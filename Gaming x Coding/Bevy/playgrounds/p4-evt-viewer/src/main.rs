use std::time::Duration;

use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    input::{
        common_conditions::input_pressed,
        mouse::{MouseMotion, MouseWheel},
    },
    prelude::*,
    render::camera::{ScalingMode, Viewport},
    sprite::MaterialMesh2dBundle,
    window::PrimaryWindow,
    winit::WinitSettings,
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use p4_evt_viewer::rand_events;

/// A marker Component to mark event entities
#[derive(Component)]
struct EventMarker;

// A func to remove previous frame(existing event entity) and
// generate a new frame(9000 random event entities)
fn generate_random_frame(
    commands: &mut Commands,
    render_assets: Res<RenderAssets>,
    query_events: Query<Entity, With<EventMarker>>,
) {
    for entity in query_events.iter() {
        commands.entity(entity).despawn();
    }
    let rect_mesh = render_assets.rect_mesh.clone_weak();
    let positive_material = render_assets.positive_material.clone_weak();
    let negative_material = render_assets.negative_material.clone_weak();
    commands.spawn_batch(
        rand_events(9000, 0..1280, 0..720)
            .into_iter()
            .map(move |evt| {
                (
                    MaterialMesh2dBundle {
                        mesh: rect_mesh.clone_weak().into(),
                        transform: evt.transform(),
                        material: if evt.p {
                            positive_material.clone_weak()
                        } else {
                            negative_material.clone_weak()
                        },
                        ..default()
                    },
                    EventMarker,
                )
            }),
    );
}

/// Used to make camera result fit-content
#[derive(Default, Resource)]
struct OccupiedScreenLogicalSpace {
    left: f32,
    top: f32,
    right: f32,
    bottom: f32,
}

fn main() {
    App::new()
        .insert_resource(RandTimer(Timer::new(
            Duration::from_millis(10),
            TimerMode::Repeating,
        )))
        .insert_resource(WinitSettings::desktop_app())
        .add_plugins(DefaultPlugins)
        .add_plugins(FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_config: TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                    font: default(),
                },
            },
        })
        .add_plugins(EguiPlugin)
        .init_resource::<OccupiedScreenLogicalSpace>()
        .init_resource::<RenderAssets>()
        .init_resource::<AppState>()
        .add_systems(Startup, setup_system)
        .add_systems(
            Update,
            (
                (
                    view_zoom_system,
                    view_drag_system.run_if(input_pressed(MouseButton::Left)),
                )
                    .run_if(view_control_condition),
                view_offset_clamp_system,
            )
                .chain(),
        )
        .add_systems(Update, ui_example_system)
        .add_systems(Update, update_camera_system)
        .add_systems(FixedUpdate, rand_events_system)
        .run();
}

/// [`Handle`]s that we need to reuse
#[derive(Resource)]
pub struct RenderAssets {
    pub rect_mesh: Handle<Mesh>,
    pub positive_material: Handle<ColorMaterial>,
    pub negative_material: Handle<ColorMaterial>,
}

impl FromWorld for RenderAssets {
    fn from_world(world: &mut World) -> Self {
        let rect_mesh = {
            let mut meshes = world.resource_mut::<Assets<Mesh>>();
            meshes.add(Rectangle::default())
        };

        let mut materials = world.resource_mut::<Assets<ColorMaterial>>();
        let positive_material = materials.add(Color::srgb_u8(0x40, 0x7e, 0xc9));
        let negative_material = materials.add(Color::WHITE);

        Self {
            rect_mesh,
            positive_material,
            negative_material,
        }
    }
}

#[derive(Resource)]
pub struct AppState {
    enable_random: bool,
    display_fps: bool,
    /// Scaling for camera's projection
    scale: f32,
    offset: Vec2,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            enable_random: false,
            display_fps: true,
            scale: 1.0,
            offset: Vec2::ZERO,
        }
    }
}

fn view_control_condition(
    occupied_screen_space: Res<OccupiedScreenLogicalSpace>,
    query_window: Query<&Window, With<PrimaryWindow>>,
    mut pressed_in_area: Local<bool>,
    button_input: Res<ButtonInput<MouseButton>>,
) -> bool {
    let window = query_window.get_single().unwrap();
    let cursor_in_area = window
        .cursor_position()
        .map(|pos| {
            pos.x > occupied_screen_space.left
                && pos.x < window.width() - occupied_screen_space.right
                && pos.y > occupied_screen_space.top
                && pos.y < window.height() - occupied_screen_space.bottom
        })
        .unwrap_or(false);

    if button_input.just_pressed(MouseButton::Left) && cursor_in_area {
        *pressed_in_area = true;
    }
    if button_input.just_released(MouseButton::Left) {
        *pressed_in_area = false;
    }
    cursor_in_area || *pressed_in_area
}

fn view_offset_clamp_system(mut state: ResMut<AppState>) {
    let width = ((1280.0 - 1.0 / state.scale * 1280 as f32) / 2.0).max(0.0);
    let height = ((720.0 - 1.0 / state.scale * 720 as f32) / 2.0).max(0.0);

    state.offset = state
        .offset
        .clamp(Vec2::new(-width, -height), Vec2::new(width, height));
}

fn view_zoom_system(mut state: ResMut<AppState>, mut evr_scroll: EventReader<MouseWheel>) {
    use bevy::input::mouse::MouseScrollUnit;
    for ev in evr_scroll.read() {
        match ev.unit {
            MouseScrollUnit::Line => {
                state.scale = (state.scale + 0.1 * ev.y).clamp(1.0, 3.0);
            }
            MouseScrollUnit::Pixel => {
                println!(
                    "Scroll (pixel units): vertical: {}, horizontal: {}",
                    ev.y, ev.x
                );
            }
        }
    }
}

fn view_drag_system(mut state: ResMut<AppState>, mut evr_motion: EventReader<MouseMotion>) {
    let scale = 1.0 / state.scale;
    for ev in evr_motion.read() {
        state.offset += Vec2::new(-ev.delta.x, ev.delta.y) * scale;
    }
}

/// egui ui
fn ui_example_system(
    mut contexts: EguiContexts,
    mut occupied_screen_space: ResMut<OccupiedScreenLogicalSpace>,
    mut commands: Commands,
    render_assets: Res<RenderAssets>,
    mut state: ResMut<AppState>,
    query_events: Query<Entity, With<EventMarker>>,
    mut overlay: ResMut<FpsOverlayConfig>,
) {
    let ctx = contexts.ctx_mut();

    occupied_screen_space.left = egui::SidePanel::left("left_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Left resizeable panel");

            ui.checkbox(
                &mut state.enable_random,
                "enable random generator in update",
            );

            ui.checkbox(&mut state.display_fps, "display fps");
            if state.display_fps {
                overlay.text_config.color.set_alpha(1.0);
            } else {
                overlay.text_config.color.set_alpha(0.0);
            }

            if ui.button("generate random frame").clicked() {
                generate_random_frame(&mut commands, render_assets, query_events);
            }
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();
    occupied_screen_space.right = egui::SidePanel::right("right_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Right resizeable panel");
            if ui.button("reset camera").clicked() {
                state.offset = Vec2::ZERO;
                state.scale = 1.0;
            }
            ui.add(egui::Slider::new(&mut state.scale, 1.0..=3.0).text("zoom"));
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

    // Request repaint every frame
    ctx.request_repaint();
}

#[derive(Resource)]
struct RandTimer(Timer);

fn rand_events_system(
    mut commands: Commands,
    render_assets: Res<RenderAssets>,
    state: Res<AppState>,
    query_events: Query<Entity, With<EventMarker>>,
    time: Res<Time>,
    mut timer: ResMut<RandTimer>,
) {
    if state.enable_random && timer.0.tick(time.delta()).just_finished() {
        generate_random_frame(&mut commands, render_assets, query_events);
    }
}

fn setup_system(
    mut commands: Commands,
    render_assets: Res<RenderAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query_events: Query<Entity, With<EventMarker>>,
) {
    // Background 1280 x 720 black rectangle
    commands.spawn(MaterialMesh2dBundle {
        mesh: render_assets.rect_mesh.clone_weak().into(),
        transform: Transform::default()
            .with_translation(-Vec3::Z)
            .with_scale(Vec3::new(2560.0, 720.0, 1.0)), // Use -1 to make sure background is behind all events
        material: materials.add(Color::srgb(0.0, 0.0, 0.0)),
        ..default()
    });
    generate_random_frame(&mut commands, render_assets, query_events);

    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            near: -1000.0,
            scaling_mode: ScalingMode::AutoMin {
                min_width: 1280.0,
                min_height: 720.0,
            },
            ..Default::default()
        },
        ..Default::default()
    });
}

fn update_camera_system(
    occupied_screen_space: Res<OccupiedScreenLogicalSpace>,
    windows: Query<&Window, With<PrimaryWindow>>,
    state: Res<AppState>,
    mut camera_transform: Query<&mut Transform, With<Camera>>,
    mut camera: Query<&mut Camera>,
    mut projection: Query<&mut OrthographicProjection>,
) {
    let window = windows.single();

    let logical_width = window.width() - occupied_screen_space.left - occupied_screen_space.right;
    let logical_height = window.height() - occupied_screen_space.top - occupied_screen_space.bottom;
    let logical_size = Vec2::new(logical_width, logical_height);
    let logical_position = Vec2::new(
        occupied_screen_space.left + (logical_width - logical_size.x) / 2.0,
        occupied_screen_space.top + (logical_height - logical_size.y) / 2.0,
    );
    let mut physical_position = (logical_position * window.scale_factor()).as_uvec2();
    let mut physical_size = (logical_size * window.scale_factor()).as_uvec2();
    if physical_size.x == 0 || physical_size.y == 0 {
        physical_position = UVec2::ZERO;
        physical_size = UVec2::new(1, 1);
    }

    let mut camera = camera.get_single_mut().unwrap();
    camera.viewport = Some(Viewport {
        physical_position,
        physical_size,
        depth: 0.0..1.0,
    });

    let mut projection = projection.get_single_mut().unwrap();
    projection.scale = 1.0 / state.scale;

    let mut camera_transform = camera_transform.get_single_mut().unwrap();
    camera_transform.translation.x = state.offset.x;
    camera_transform.translation.y = state.offset.y;
}
