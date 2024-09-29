# p4-evt-viewer

二维相机 `Camera2dBundle`：

- `Viewport`

- `OrthographicProjection`：`scale` 缩放、`scaling_mode` 固定

Plugin：`FPSOverlayPlugin`

EntityCommands 的 `despawn`与 Commands 的 `spawn_batch`

`FromWorld` 的基本用法

System 的 `run_if` 以及 `EventReader<MouseWheel>`

![evt-viewer](./assets/evt-viewer.gif)

![evt-viewer-zoom](./assets/evt-viewer-zoom.gif)

---

界面大概框架是与 [p3-bevy-egui](../p3-bevy-egui/README.md) 一样的 egui 上下左右面板 + `object-fit: contain` 的 Camera。

## 一、事件表示

```rust
use std::ops::Range;

use bevy::{math::Vec3, prelude::Transform};
use rand::random;

/// generate cnt events in specific coord range
pub fn rand_events(cnt: usize, x_range: Range<u16>, y_range: Range<u16>) -> Vec<Event> {
    (0..cnt)
        .map(|_idx| Event {
            t: 0,
            x: random::<u16>() % (x_range.end - x_range.start) + x_range.start,
            y: random::<u16>() % (y_range.end - y_range.start) + y_range.start,
            p: random(),
        })
        .collect()
}

pub struct Event {
    pub t: u64,
    pub x: u16,
    pub y: u16,
    pub p: bool,
}

impl Event {
    pub fn transform(&self) -> Transform {
        Transform::default().with_translation(Vec3::new(
            self.x as f32 - 1280.0 / 2.0,
            self.y as f32 - 720.0 / 2.0,
            0.0,
        ))
    }
}


```

## 二、mesh 与 material 初始化

对于每一个事件，他们所用到的 mesh 和 material 是重复的，只需要向 `Assets` 中添加一次，之后重复使用即可。

为了实现在初始时添加一次这些 `Asset`，并在后续可以引用，可以实现一个保存 `Handle` 的 `Resource`，并为其实现 `FromWorld` 来初始化：

```rust
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

fn main() {
    App::new()
    	.init_resource::<RenderAssets>()
    	// ...
    	.run();
}
```

这样在后续就可以通过 `Res<RenderAssets>` 来访问这些资源了。

## 三、随机的一帧事件（们）

每切换一帧，需要移除（despawn）掉所有已经存在的事件，并添加（spawn）新的一帧的所有事件。

```rust
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
```

## 四、FixedUpdate 固定间隔生成新的帧

`Update` 运行的间隔并不固定，当帧率高的时候频率会很高，而当帧率低的时候频率会很低，但 `FixedUpdate` 会尝试以固定间隔去执行（默认 65Hz）。

有时 `FixedUpdate` 会通过跳过一次调用或者多进行一次调用来调节自己的执行频率，适合进行 GamePlay 相关（网络、逻辑、物理等等）的运算，而 `Update` 更适合进行与帧率相关（UI、动画、视觉效果、相机控制）的运算。

至于间隔那就很简单，用 [p1-basic](../p1-basic/README.md) 中用过的 Timer + Time 即可，再配合上 UI 的控制状态：

```rust
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
```

UI 就略了，没什么东西

## 五、帧率显示

为 bevy 启用 `bevy_dev_tools` feature 后，可以使用 `FpsOverlayPlugin` 来添加一个 FpsOverlay：

```rust
fn main() {
    App::new()
        .add_plugins(FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_config: TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                    font: default(),
                },
            },
        })
    	// ...
    	.run();
}
```

至于添加一个开关来控制显示与否，可以通过设置其颜色的 alpha 值来实现：

```rust
ui.checkbox(&mut state.display_fps, "display fps");
if state.display_fps {
    overlay.text_config.color.set_alpha(1.0);
} else {
    overlay.text_config.color.set_alpha(0.0);
}
```

## 六、滚动缩放

很简单，设置 `OrthographicProjection` 的 `scale` 属性即可。

还有就是在鼠标位于 Camera 区域内时读取鼠标滚轮来调整：

```rust
fn scroll_zoom_condition(
    occupied_screen_space: Res<OccupiedScreenLogicalSpace>,
    query_window: Query<&Window, With<PrimaryWindow>>,
) -> bool {
    let window = query_window.get_single().unwrap();
    if let Some(pos) = window.cursor_position() {
        return pos.x > occupied_screen_space.left
            && pos.y < window.width() - occupied_screen_space.right
            && pos.y > occupied_screen_space.top
            && pos.y < window.height() - occupied_screen_space.bottom;
    }
    false
}

fn scroll_zoom_system(mut state: ResMut<AppState>, mut evr_scroll: EventReader<MouseWheel>) {
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
```

```rust
func main() {
    App::new()
    	.add_systems(Update, scroll_zoom_system.run_if(scroll_zoom_condition))
    	// ...
    	.run();
}
```

