# p2-2d-shapes

重要概念：`Assets`、`Handle`、`AssetId`

Component Bundle：`Camera2dBundle` 相机简单使用、`MaterialMesh2dBundle` 2d 图形、`TextBundle` 简单文字

Plugin：`Wireframe2dPlugin` 2d 图形线框显示

`Mesh2dHandle`、`Mesh`、`Transform`

![image-20240928151411878](./assets/image-20240928151411878.png)

![image-20240928151405644](./assets/image-20240928151405644.png)

---

## 一、2D 相机

Bevy 提供了一个 `Camera2dBundle` 作为相机实体：

```rust
fn main() {
    App::new().add_plugins((
        DefaultPlugins,
    ))
    .add_systems(Startup, setup)
    .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
}
```

2D 相机其实就是一个使用正交投影的 3D 相机，可以将 3D 控件按照平行的线投影到一个 2D 平面上。

默认的 Camera2dBundle 使用 `far=1000, near=-1000` 的正交投影

有关更多 `Camera2dBundle` 见 [TODO]()。

## 二、Assets 与 Handle

对于 `Mesh` 以及 `Material` 数据，需要一个全局的管理器，这时候就需要用到 `Assets`。

`Assets<A: Asset>` 类似于一个资源管理器，可以存储若干 `A`，每个 `Asset` 可以由 `AssetId` 来标识，由 `Handle` 来引用。

使用 `Assets` 的 `add` 方法可以添加一个 `asset`，它会返回一个 `Handle`：

```rust
pub fn add(&mut self, asset: impl Into<A>) -> Handle<A>
```

`Handle` 有点类似于 `Rc` 或 `Arc` 这种引用，通过 `Handle` 的 `id` 方法可以获取到 `AssetId`：

```rust
pub fn id(&self) -> AssetId<A>
```

### Index 与 Uuid 类型的 AssetId

通过 `add` 方法添加的 `Asset` 会默认以 `AssetId::Index` 类型的 `AssetId` 来标识，这个 `Asset` 会被存储在一个稠密的结构中（类似于一个向量），更加高效一些。

与之相比也可以通过 `insert` 方法手动指定 `AssetId`，于是就可以以 `AssetId::Uuid` 来标识，这样 `Asset` 会被存储在一个哈希表中，不那么高效，但是好处就是可以在编译器去以 `Uuid` 引用它：

```rust
pub fn insert(&mut self, id: impl Into<AssetId<A>>, asset: A)
```

比如这样：

```rust
/// Handle to the custom shader with a unique random ID
pub const COLORED_MESH2D_SHADER_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(13828845428412094821);

// ...

shaders_assets.insert(
    &COLORED_MESH2D_SHADER_HANDLE,
    Shader::from_wgsl(COLORED_MESH2D_SHADER, file!()),
);
```

### Weak 与 Strong 的 Handle

`Handle` 也有两种变体，分别是 `Weak` 和 `Strong`。

`Handle::Strong` 类似于 `Arc`（其实其内部就是个 `Arc`），当所有 `Strong` 的 `Handle` 都被丢弃后，这个 `Asset` 也会被释放。

而 `Handle::Weak` 内部保存的是一个 `AssetId`，与 `Asset` 的生命周期并没有关系。

通过 `Assets::add` 得到的 `Handle` 就是 `Strong` 的，而通过 `Handle::weak_from_u128` 指定 `AssetId::Uuid` 得到的 `Handle` 就是 `Weak` 的

## 二、基本 2d 图形

Bevy 提供了一个 `MaterialMesh2dBundle` 作为可渲染的 2d 图形的实体，主要包含三个部分：

- `Mesh2dHandle`：2D 形状的 Handle
- `Handle<M: Material2d>`：材质的 Handle
- `Transform`：变换

```rust
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

fn main() {
    App::new().add_plugins(DefaultPlugins)
    .add_systems(Startup, setup)
    .run();
}

const X_EXTENT: f32 = 900.;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let shapes = [
        Mesh2dHandle(meshes.add(Circle { radius: 50.0 })),
        Mesh2dHandle(meshes.add(CircularSector::new(50.0, 1.0))),
        Mesh2dHandle(meshes.add(CircularSegment::new(50.0, 1.25))),
        Mesh2dHandle(meshes.add(Ellipse::new(25.0, 50.0))),
        Mesh2dHandle(meshes.add(Annulus::new(25.0, 50.0))),
        Mesh2dHandle(meshes.add(Capsule2d::new(25.0, 50.0))),
        Mesh2dHandle(meshes.add(Rhombus::new(75.0, 100.0))),
        Mesh2dHandle(meshes.add(Rectangle::new(50.0, 100.0))),
        Mesh2dHandle(meshes.add(RegularPolygon::new(50.0, 6))),
        Mesh2dHandle(meshes.add(Triangle2d::new(
            Vec2::Y * 50.0,
            Vec2::new(-50.0, -50.0),
            Vec2::new(50.0, -50.0),
        ))),
    ];
    let num_shapes = shapes.len();

    for (i, shape) in shapes.into_iter().enumerate() {
        // Distribute colors evenly across the rainbow.
        let color = Color::hsl(360. * i as f32 / num_shapes as f32, 0.95, 0.7);

        commands.spawn(MaterialMesh2dBundle {
            mesh: shape,
            material: materials.add(color),
            transform: Transform::from_xyz(
                // Distribute shapes from -X_EXTENT/2 to +X_EXTENT/2.
                -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
                0.0,
                0.0,
            ),
            ..default()
        });
    }
}
```

## 三、Wireframe2dPlugin

Bevy 提供了一个 `Wireframe2dPlugin` 来渲染出图元的线框（不过目前不支持 webgl 或者 webgpu）：

```rust
#[cfg(not(target_arch = "wasm32"))]
use bevy::sprite::{Wireframe2dConfig, Wireframe2dPlugin};
```

```diff
fn main() {
    let mut app = App::new().add_plugins((
        DefaultPlugins,
+         #[cfg(not(target_arch = "wasm32"))]
+         Wireframe2dPlugin,
    ))
    .add_systems(Startup, setup);
+     #[cfg(not(target_arch = "wasm32"))]
+     app.add_systems(Update, toggle_wireframe);
    app.run();
}

#[cfg(not(target_arch = "wasm32"))]
fn toggle_wireframe(
    mut wireframe_config: ResMut<Wireframe2dConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        wireframe_config.global = !wireframe_config.global;
    }
}
```

## 三、简单文字

```rust
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // ...
    #[cfg(not(target_arch = "wasm32"))]
    commands.spawn(
        TextBundle::from_section("Press space to toggle wireframes", TextStyle::default())
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(12.0),
                left: Val::Px(12.0),
                ..default()
            }),
    );
}
```

