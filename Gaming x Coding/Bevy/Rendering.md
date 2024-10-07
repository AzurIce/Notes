# Rendering

## 一、基本概念

### 1. SubApp

一个 `App` 可以有若干个 `SubApp`，每一个 `SubApp` 都有自己的 `World`、`Schedule`、`Resource` 等等，可以独立运行：

```rust
#[derive(Resource, Default)]
struct Val(pub i32);

// Create an app with a certain resource.
let mut app = App::new();
app.insert_resource(Val(10));

// Create a sub-app with the same resource and a single schedule.
let mut sub_app = SubApp::new();
sub_app.insert_resource(Val(100));
```

数据可以从 `App` 通过 Extract 流向 `SubApp`：

```rust
// Setup an extract function to copy the resource's value in the main world.
sub_app.set_extract(|main_world, sub_world| {
    sub_world.resource_mut::<Val>().0 = main_world.resource::<Val>().0;
});

// Schedule a system that will verify extraction is working.
sub_app.add_systems(Main, |counter: Res<Val>| {
    // The value will be copied during extraction, so we should see 10 instead of 100.
    assert_eq!(counter.0, 10);
});
```

```rust
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, AppLabel)]
struct ExampleApp;

// Add the sub-app to the main app.
app.insert_sub_app(ExampleApp, sub_app);

// Update the application once (using the default runner).
app.run();
```

### 2. Render App 与 Pipelined Rendering

Bevy 中的渲染就是发生在一个 SubApp —— `RenderApp` 中的，那么很显然它有自己的 World 和 Schedule。

渲染的过程可以主要分为几个主要的 Schedule：Extract、Prepare、Queue、Render、Cleanup

由于 RenderApp 这个 SubApp 在运行是无法访问到 Main World 中的数据的，因此 Extract 就承担了从 Main World 获取渲染所需必要数据到 Render World 的作用。

可以发现，除了 Extract 外，后续的几个步骤都不再与 Main World 有关系，因此逻辑与渲染各自操作各自的 World，互不干扰，可以并行运行，这也就是 Pipelined Rendering：

![pipelined rendering stages](./assets/pipelined_rendering_stages.svg)

### 3. Render Graphs

用于组织不同渲染内容之间的依赖关系。

## 二、ExtractComponent

为了让一个 Component 可以被 Extract 到 RenderApp 中，需要为它实现一个 `ExtractComponent` Trait：

```rust
pub trait ExtractComponent: Component {
    type QueryData: ReadOnlyQueryData;
    type QueryFilter: QueryFilter;
    type Out: Bundle;

    // Required method
    fn extract_component(
        item: <Self::QueryData as WorldQuery>::Item<'_>,
    ) -> Option<Self::Out>;
}
```

也就是基于 Query 获取需要 Extract 的信息，然后以 Bundle 的形式返回。

为了方便，Bevy 提供了一个 Derive 宏，可以直接为 `Clone` 的 `Component` 实现这个 Trait（Extract 时就是直接 `clone`）

## RenderPhase

在 Bevy 中，每一个 View（如相机、会产生阴影的光源等等）都有一个或多个 Render Phase（如 Opaque、Transparent、Shadow等等），每一个 Render Phase 中都可以查询需要绘制的实体。

之所以需要不同的 Phase，是因为不同的 Phase 中可能对 Sorting 或 Batching 的行为有不同的要求（比如 Opaque 需要从前到后排序，而 Transparent 需要从后到前排序）同时后一个 Phase 可能对前一个 Phase 的渲染结果有依赖（比如屏幕空间反射）。

要想绘制一个实体，需要为每一个能看到这个实体的 View 添加一个对应的 PhaseItem 到一个或多个 Render Phase 中。这个过程需要在 `RenderSet::Queue` 中完成。

在这之后，Render Phase 会将他们在 `RenderSet::PhaseSort` 中排序，最终使用一个 `TrackedRenderPass` 在 `RenderSet::Render` 中渲染。

所以对于每一个 `PhaseItem` 都需要一个 `Draw` 函数来设置好 `TrackedRenderPass` 的状态（选择 `RenderPipeline`、设置 `BindGroup` 等等）并且发起一个绘制调用。

`Draw` 函数可以被直接从 Trait 实现，或者通过组合多个 `RenderCommand` 实现。



## DrawFunctions



## RenderCommand

RenderCommand 是用于组件渲染逻辑的模块化的标准组件，最终会被转换为 `Draw` 函数，比如 `DrawMaterial` 其实就是这样的一个元组：

```rust
pub type DrawMaterial<M> = (
    SetItemPipeline,
    SetMeshViewBindGroup<0>,
    SetMeshBindGroup<1>,
    SetMaterialBindGroup<M, 2>,
    DrawMesh,
);
```



有 DrawFunctions 的 Resource：

```rust
pub struct DrawFunctions<P>
where
    P: PhaseItem,
{ /* private fields */ }
```

在其中会为一个 PhaseItem 存储所有的 Draw Functions，通过 `SubApp::add_render_command` 可以将一个 `RenderCommand` 注册为对应 `PhaseItem` 的 `Draw` 函数：

```rust
fn add_render_command<P, C>(&mut self) -> &mut SubApp
where
    P: PhaseItem,
    C: RenderCommand<P> + Send + Sync + 'static,
    <C as RenderCommand<P>>::Param: ReadOnlySystemParam,
```



## 二、如何为某个 Component 实现渲染的逻辑

Bevy 中的 `MaterialMesh2dBundle` 之所以可以渲染，是因为其中的 `Mesh2dHandle` 在 `Mesh2dRenderPlugin` 以及 `Material2dPlugin` 中实现了对应的渲染逻辑，同理 `SpriteBundle` 之所以可以渲染，是因为其中的 `Sprite` 在 `SpritePlugin` 以及 `Material2dPlugin` 中实现了对应的渲染逻辑。

如果观察它们的结构，可以发现都存在一些相同的部分：

```diff
pub struct SpriteBundle {
    pub sprite: Sprite,
+   pub transform: Transform,
+   pub global_transform: GlobalTransform,
    pub texture: Handle<Image>,
+   pub visibility: Visibility,
+   pub inherited_visibility: InheritedVisibility,
+   pub view_visibility: ViewVisibility,
}

pub struct MaterialMesh2dBundle<M>
where
    M: Material2d,
{
    pub mesh: Mesh2dHandle,
    pub material: Handle<M>,
+   pub transform: Transform,
+   pub global_transform: GlobalTransform,
+   pub visibility: Visibility,
+   pub inherited_visibility: InheritedVisibility,
+   pub view_visibility: ViewVisibility,
}
```

其实有一个专门的 Bundle 包含这些部分：

```rust
pub struct SpatialBundle {
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}
```

这些部分的作用就是为实体实现正确的位置渲染。



下面是一个简单的对一个 `CustomMaterial` 的渲染支持的 Plugin：

```rust
struct CustomMaterialPlugin;

impl Plugin for CustomMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ExtractComponentPlugin::<InstanceMaterialData>::default());
        app.sub_app_mut(RenderApp)
            .add_render_command::<Transparent3d, DrawCustom>()
            .init_resource::<SpecializedMeshPipelines<CustomPipeline>>()
            .add_systems(
                Render,
                (
                    queue_custom.in_set(RenderSet::QueueMeshes),
                    prepare_instance_buffers.in_set(RenderSet::PrepareResources),
                ),
            );
    }

    fn finish(&self, app: &mut App) {
        app.sub_app_mut(RenderApp).init_resource::<CustomPipeline>();
    }
}
```

## ExtractComponentPlugin

由于在 Render World 中无法访问到 Main World 的数据，因此需要通过 Extract 来讲需要用的数据提取到 Render World 中。

Bevy 提供了一个简单的 `ExtractComponentPlugin`，通过它可以使一个 Component 在 Extract 阶段自动提取到 Render World 中。



se140145

## 参考

[Render Architecture Overview - Unofficial Bevy Cheat Book (bevy-cheatbook.github.io)](https://bevy-cheatbook.github.io/gpu/intro.html)

[Bevy 0.6 (bevyengine.org)](https://bevyengine.org/news/bevy-0-6/)

[Bevy Rendering Demystified (youtube.com)](https://www.youtube.com/watch?v=5oKEPZ6LbNE)