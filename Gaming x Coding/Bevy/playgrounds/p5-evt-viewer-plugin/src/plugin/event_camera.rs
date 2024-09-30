use bevy::{
    app::{Plugin, PostUpdate, Update},
    input::{
        common_conditions::input_pressed,
        mouse::{MouseMotion, MouseWheel},
        ButtonInput,
    },
    math::Vec2,
    prelude::{
        Bundle, Camera, Camera2dBundle, Commands, Component, Entity, EventReader,
        IntoSystemConfigs, Local, MouseButton, OrthographicProjection, Query, Res, Transform, With,
    },
    render::camera::{ScalingMode, Viewport},
    utils::HashMap,
    window::{PrimaryWindow, Window},
};

pub struct EventCameraPlugin;

impl Plugin for EventCameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (
                update_is_controlling_system,
                (
                    view_zoom_system,
                    view_drag_system.run_if(input_pressed(MouseButton::Left)),
                ),
                view_offset_clamp_system,
            )
                .chain(),
        )
        .add_systems(PostUpdate, update_event_camera_system);
    }
}

#[derive(Component)]
struct IsControlling;

fn update_is_controlling_system(
    mut commands: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
    mut pressed_in_area: Local<HashMap<u32, bool>>,
    button_input: Res<ButtonInput<MouseButton>>,
    evt_cam_query: Query<(Entity, &ObjectFitContain), With<EventCamera>>,
) {
    let window = window.get_single().unwrap();
    for (entity, object_fit_contain) in evt_cam_query.iter() {
        if !pressed_in_area.contains_key(&entity.index()) {
            pressed_in_area.insert(entity.index(), false);
        }
        let pressed_in_area = pressed_in_area.get_mut(&entity.index()).unwrap();

        let cursor_in_area = window
            .cursor_position()
            .map(|pos| {
                pos.x > object_fit_contain.l
                    && pos.x < window.width() - object_fit_contain.r
                    && pos.y > object_fit_contain.t
                    && pos.y < window.height() - object_fit_contain.b
            })
            .unwrap_or(false);

        if button_input.just_pressed(MouseButton::Left) && cursor_in_area {
            *pressed_in_area = true;
        }
        if button_input.just_released(MouseButton::Left) {
            *pressed_in_area = false;
        }
        if cursor_in_area || *pressed_in_area {
            commands.entity(entity).insert(IsControlling);
        } else {
            commands.entity(entity).remove::<IsControlling>();
        }
    }
}

fn view_offset_clamp_system(mut evt_cam_query: Query<(&EventCamera, &mut ViewOptions)>) {
    for (event_camera, mut view_options) in evt_cam_query.iter_mut() {
        let width = (event_camera.width as f32 * (1.0 - 1.0 / view_options.scale)) / 2.0;
        let height = (event_camera.height as f32 * (1.0 - 1.0 / view_options.scale)) / 2.0;

        view_options.offset = view_options
            .offset
            .clamp(Vec2::new(-width, -height), Vec2::new(width, height));
    }
}

fn view_zoom_system(
    mut evt_cam_query: Query<&mut ViewOptions, (With<EventCamera>, With<IsControlling>)>,
    mut evr_scroll: EventReader<MouseWheel>,
) {
    use bevy::input::mouse::MouseScrollUnit;
    for mut view_options in evt_cam_query.iter_mut() {
        for ev in evr_scroll.read() {
            match ev.unit {
                MouseScrollUnit::Line => {
                    view_options.scale =
                        (view_options.scale + 0.1 * ev.y).clamp(1.0, view_options.max_scale);
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
}

fn view_drag_system(
    mut evt_cam_query: Query<&mut ViewOptions, (With<EventCamera>, With<IsControlling>)>,
    mut evr_motion: EventReader<MouseMotion>,
) {
    for mut view_options in evt_cam_query.iter_mut() {
        let scale = view_options.scale;
        for ev in evr_motion.read() {
            view_options.offset += Vec2::new(-ev.delta.x, ev.delta.y) * scale;
        }
    }
}

fn update_event_camera_system(
    mut evt_cam_query: Query<
        (
            &ViewOptions,
            &ObjectFitContain,
            &mut Transform,
            &mut Camera,
            &mut OrthographicProjection,
        ),
        With<EventCamera>,
    >,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let window = windows.single();
    for (view_options, object_fit_contain, mut transform, mut camera, mut projection) in
        evt_cam_query.iter_mut()
    {
        let logical_width = window.width() - object_fit_contain.l - object_fit_contain.r;
        let logical_height = window.height() - object_fit_contain.t - object_fit_contain.b;

        let logical_size = Vec2::new(logical_width, logical_height);
        let logical_position = Vec2::new(
            object_fit_contain.l + (logical_width - logical_size.x) / 2.0,
            object_fit_contain.t + (logical_height - logical_size.y) / 2.0,
        );

        let mut physical_position = logical_position * window.scale_factor();
        let mut physical_size = logical_size * window.scale_factor();
        if physical_size.x == 0.0 || physical_size.y == 0.0 {
            physical_position = Vec2::ZERO;
            physical_size = Vec2::splat(1.0);
        }

        camera.viewport = Some(Viewport {
            physical_position: physical_position.as_uvec2(),
            physical_size: physical_size.as_uvec2(),
            depth: 0.0..1.0,
        });

        projection.scale = 1.0 / view_options.scale;
        transform.translation.x = view_options.offset.x / view_options.scale;
        transform.translation.y = view_options.offset.y / view_options.scale;
    }
}

#[derive(Component)]
pub struct EventCamera {
    width: u32,
    height: u32,
}

impl EventCamera {
    pub fn aspect_ratio(&self) -> f32 {
        self.width as f32 / self.height as f32
    }
}

/// Logical size of the bound
#[derive(Component, Default)]
pub struct ObjectFitContain {
    pub l: f32,
    pub r: f32,
    pub t: f32,
    pub b: f32,
}

#[derive(Component)]
pub struct ViewOptions {
    pub max_scale: f32,
    pub scale: f32,
    pub offset: Vec2,
}

impl Default for ViewOptions {
    fn default() -> Self {
        Self {
            max_scale: 4.0,
            scale: 1.0,
            offset: Vec2::ZERO,
        }
    }
}

#[derive(Bundle)]
pub struct EventCameraBundle {
    pub camera2d_bundle: Camera2dBundle,
    pub event_camera: EventCamera,
    pub object_fit_contain: ObjectFitContain,
    pub view_options: ViewOptions,
}

impl EventCameraBundle {
    pub fn new_with_size(width: u32, height: u32) -> Self {
        Self {
            camera2d_bundle: Camera2dBundle {
                projection: OrthographicProjection {
                    near: -1000.0,
                    scaling_mode: ScalingMode::AutoMin {
                        min_width: width as f32,
                        min_height: height as f32,
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            event_camera: EventCamera { width, height },
            object_fit_contain: Default::default(),
            view_options: Default::default(),
        }
    }
}
