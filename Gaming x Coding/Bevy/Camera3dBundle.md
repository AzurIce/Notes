# Camera3dBundle

[`Camera3dBundle`]([Camera3dBundle in bevy::core_pipeline::core_3d - Rust (docs.rs)](https://docs.rs/bevy/0.14.2/bevy/core_pipeline/core_3d/struct.Camera3dBundle.html#)) 是若干用于组成一个完整的三维相机的 `Component` 的 `Bundle`：

- `Camera`
- `CameraRenderGraph`
- `Projection`
- `VisibleEntities`
- `Frustum`
- `Transform`
- `GlobalTransform`
- `Camera3d`
- `Tonemapping`
- `DebandDither`
- `ColorGrading`
- `Exposure`
- `CameraMainTextureUsages`

## 1. Camera

[`Camera`]([Camera in bevy::render::prelude - Rust (docs.rs)](https://docs.rs/bevy/0.14.2/bevy/render/prelude/struct.Camera.html)) 组件定义了相机实体相关信息：

- `viewport`：表示用于输出相机渲染结果的位置、大小、深度信息

    > **例**：
    >
    > - 固定宽高比视口 fit-content 绘制：[p3-bevy-egui](./playgrounds/p3-bevy-egui/README.md)

    