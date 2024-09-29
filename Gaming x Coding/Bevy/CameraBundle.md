# CameraBundle

## 一、2d / 3d 包含的 Component

[`Camera3dBundle`]([Camera3dBundle in bevy::core_pipeline::core_3d - Rust (docs.rs)](https://docs.rs/bevy/0.14.2/bevy/core_pipeline/core_3d/struct.Camera3dBundle.html#)) 和 [`Camera3dBundle`]([Camera3dBundle in bevy::core_pipeline::core_3d - Rust (docs.rs)](https://docs.rs/bevy/0.14.2/bevy/core_pipeline/core_3d/struct.Camera3dBundle.html#)) 都是若干用于组成一个完整的三维相机的 `Component` 的 `Bundle`。

相同的 Component：

- `Camera`
- `CameraRenderGraph`
- `VisibleEntities`
- `Frustum`
- `Transform`
- `GlobalTransform`
- `Tonemapping`
- `DebandDither`
- `CameraMainTextureUsages`

不同的 Component：

- `ColorGrading`（3d 特有）

- `Exposure`（3d 特有）

- `Camera3d`（3d）/ `Camera2d`（2d）

    都是用于标记的 Component，有 `With<Camera>` 的额外 `Filter`

- `Projection`（3d）/ `OrthographicProjection`（2d）

    `Projection` 就是一个包含 `PerspectiveProjection` 或 `OrthographicProjection` 的枚举

## 二、相同 Component

### 1. Camera

[`Camera`]([Camera in bevy::render::prelude - Rust (docs.rs)](https://docs.rs/bevy/0.14.2/bevy/render/prelude/struct.Camera.html)) 组件定义了相机实体相关信息：

- `viewport`：表示用于输出相机渲染结果的位置、大小、深度信息

    > **例**：
    >
    > - 固定宽高比视口 fit-content 绘制：[p3-bevy-egui](./playgrounds/p3-bevy-egui/README.md)

### 2. Transform

[`Transform`]([Transform in bevy::transform::components - Rust (docs.rs)](https://docs.rs/bevy/latest/bevy/transform/components/struct.Transform.html)) 组件定义了物体的位置等变换属性：

```rust
pub struct Transform {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}
```

默认值是 `Transform::IDENTITY`：

```rust
pub const IDENTITY: Self = Transform {
    translation: Vec3::ZERO,
    rotation: Quat::IDENTITY,
    scale: Vec3::ONE,
};
```

也就是位置 `(0.0, 0.0, 0.0)` 朝向 -Z 方向，向上方向为 Y。

## 三、不同 Component

## 1. OrthographicProjection

正交投影：

```rust
pub struct OrthographicProjection {
    pub near: f32,
    pub far: f32,
    pub viewport_origin: Vec2,
    pub scaling_mode: ScalingMode,
    pub scale: f32,
    pub area: Rect,
}
```

默认 `near` 为 `0.0`，但是 `Camera2dBundle` 会将其设为 `-1000.0`

