use std::time::Duration;

use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    prelude::*,
    sprite::MaterialMesh2dBundle,
    window::PrimaryWindow,
    winit::WinitSettings,
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use p5_evt_viewer_plugin::plugin::event_camera::{
    EventCameraBundle, ObjectFitContain, ViewOptions,
};
use p5_evt_viewer_plugin::plugin::EventCameraPlugin;
use p5_evt_viewer_plugin::{rand_events, CDEvent};

fn evt_transform(evt: &CDEvent, width: u16, height: u16) -> Transform {
    Transform::default().with_translation(Vec3::new(
        evt.x as f32 - width as f32 / 2.0,
        evt.y as f32 - height as f32 / 2.0,
        0.0,
    ))
}

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
    let (width, height) = (1280, 720);
    for entity in query_events.iter() {
        commands.entity(entity).despawn();
    }
    let rect_mesh = render_assets.rect_mesh.clone_weak();
    let positive_material = render_assets.positive_material.clone_weak();
    let negative_material = render_assets.negative_material.clone_weak();
    commands.spawn_batch(
        rand_events(9000, 0..width, 0..height)
            .into_iter()
            .map(move |evt| {
                (
                    MaterialMesh2dBundle {
                        mesh: rect_mesh.clone_weak().into(),
                        transform: evt_transform(&evt, width, height),
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
        .add_plugins(EventCameraPlugin)
        .init_resource::<RenderAssets>()
        .init_resource::<AppState>()
        .add_systems(Startup, setup_system)
        .add_systems(Update, ui_example_system)
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
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            enable_random: false,
            display_fps: true,
        }
    }
}

/// egui ui
fn ui_example_system(
    mut contexts: EguiContexts,
    mut commands: Commands,
    render_assets: Res<RenderAssets>,
    mut state: ResMut<AppState>,
    query_events: Query<Entity, With<EventMarker>>,
    mut overlay: ResMut<FpsOverlayConfig>,
    mut cam1_query: Query<(&mut ViewOptions, &mut ObjectFitContain), With<Camera1>>,
    mut cam2_query: Query<
        (&mut ViewOptions, &mut ObjectFitContain),
        (With<Camera2>, Without<Camera1>),
    >,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let (mut view_options1, mut object_fit_contain1) = cam1_query.single_mut();
    let (mut view_options2, mut object_fit_contain2) = cam2_query.single_mut();
    let ctx = contexts.ctx_mut();

    let top = egui::TopBottomPanel::top("top_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Top resizeable panel");
        })
        .response
        .rect
        .height();
    let bottom = egui::TopBottomPanel::bottom("bottom_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Bottom resizeable panel");
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .height();

    let left = egui::SidePanel::left("left_panel")
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
    let right = egui::SidePanel::right("right_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Right resizeable panel");
            if ui.button("reset camera1").clicked() {
                view_options1.offset = Vec2::ZERO;
                view_options1.scale = 1.0;
            }
            let max_scale1 = view_options1.max_scale;
            ui.add(egui::Slider::new(&mut view_options1.scale, 1.0..=max_scale1).text("zoom1"));

            if ui.button("reset camera2").clicked() {
                view_options2.offset = Vec2::ZERO;
                view_options2.scale = 1.0;
            }
            let max_scale2 = view_options2.max_scale;
            ui.add(egui::Slider::new(&mut view_options2.scale, 1.0..=max_scale2).text("zoom2"));
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();

    let window = window.single();
    let width = window.width() - left - right;
    object_fit_contain1.l = left;
    object_fit_contain1.r = right + width / 2.0;
    object_fit_contain1.t = top;
    object_fit_contain1.b = bottom;

    object_fit_contain2.l = left + width / 2.0;
    object_fit_contain2.r = right;
    object_fit_contain2.t = top;
    object_fit_contain2.b = bottom;

    // Request repaint every frame
    ctx.request_repaint();
}

#[derive(Resource)]
struct RandTimer(Timer);

#[derive(Component)]
struct Camera1;

#[derive(Component)]
struct Camera2;

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
            .with_scale(Vec3::new(1280.0, 720.0, 1.0)), // Use -1 to make sure background is behind all events
        material: materials.add(Color::srgb(0.0, 0.0, 0.0)),
        ..default()
    });
    generate_random_frame(&mut commands, render_assets, query_events);

    commands.spawn((EventCameraBundle::new_with_size(1280, 720), Camera1));
    let mut cam2 = EventCameraBundle::new_with_size(1280, 720);
    cam2.camera2d_bundle.camera.order = 1; // Renders the right camera after the left camera, which has a default priority of 0
    commands.spawn((cam2, Camera2));
}
