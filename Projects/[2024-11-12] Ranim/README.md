# Ranim

用 Rust 重写 manim 的核心！

manim 的主要使用方式是通过 *manimgl* 命令从 py 文件中提取 Scene，然后将其运行。

## 架构

一旦涉及逻辑与渲染，必然离不开一个问题：如何合理地设计这两部分的数据交互。

*manim* 中的对象 `Mobject` 是所有 **数学对象** 的基类，简单来说就是一个存储着顶点信息的类，以及包含一个 `ShaderWrapper`，其中包装了渲染这个对象所需要用到的一切东西：程序、缓冲等，每一个子类会去覆盖渲染的方法以符合自己的需要。

Python / OOP当然可以这么设计，虽然各种晚初始化的 field 和各种子类对父类的覆盖很容易搞得很乱，但是既然是 **用 Rust 实现**，既然是 **我** 来，那必须要设计一个优雅的模式。

### 渲染相关设计

由于 *manim* 使用的是 *mordengl*，可以借助 **几何着色器** 来生成图元。

因此，其 `VMobject` 存储了贝塞尔曲线的原始控制点和宽度、转角等参数，而后通过几何着色器生成实际用于渲染的三角形：

```mermaid
flowchart LR
Object --变换--> Object
Object --> GPU
subgraph GPU
direction LR
	V((Vertex)) --> G((Geom)) --> F((Frag))
end
```

但是这个方案对于 *ranim* 并不可行，因为 *wgpu* 并不支持几何着色器，因此「由贝塞尔曲线原始控制点生成三角形」这件事只能在实际的渲染 Pass 前单独完成：

```mermaid
flowchart LR
Blueprint[Blueprint 结构] --> a[Object 对象] --解析--> b[Vertex 数据] --> GPU
a --变换--> a
subgraph GPU
	direction LR
	Vertex -->  Frag
end
```

这也导致整个渲染架构的分层会不可避免地变多。在这个条件下还要满足「对象」的类型擦除，还要在对应渲染时还原出其对应使用的管线。

定义 `Renderer` Trait 如下：



定义 `Renderable` Trait 如下：

```rust
pub trait Renderable {
    type Vertex;
    fn parse(&self) -> Vec<Vertex>;
}
```



设计出几种对象：

- `Renderer` 渲染器，接收某一种输入类型

- `RenderPipeline` 具有 `type Vertex`

    因为每一个管线所能输入的顶点类型是唯一的

- `Object` 具有 `type Renderer`

    因为每一个被创建出的对象其渲染方式是唯一的

- 

每一个 `Vertex` 却可以由多种 `Pipeline` 渲染，

- `Renderable<Vertex>`



*wgpu* 的逻辑与 *morderngl* 有很大的差别，在 *wgpu* 下，我们更适合以 **资源** 的方式来去考虑各种结构。

`Pipeline` 是资源，`BindGroup` 是资源，各种 `Buffer` 和 `Texture` 也是资源，在渲染管线中，真正的使用者是在实际进行渲染工作时使用的 `CommandEncoder` 以及 `RenderPass`，要想做出优雅的设计，必须捋清楚它们之间的关系。

以下是一部分对象的使用关系：

```mermaid
flowchart BT
VertexBufferLayout --> Pipeline
BindGroupLayout --> Pipeline --> RenderPass
UniformBuffer --> BindGroup --> RenderPass

subgraph buffer
    UniformBuffer
    VertexBuffer
end

subgraph texture
	DepthTexture
	MultisampleTexture
	TargetTexture
	OutputStageTexture
end

DepthTexture --view--> RenderPass
MultisampleTexture --view--> RenderPass
TargetTexture --view--> RenderPass
VertexBuffer --> RenderPass
OutputStageTexture --> CommandEncoder
```

第一个首先，就是一个 `WgpuContext`：

```rust
pub struct WgpuContext {
    pub instance: wgpu::Instance,
    pub adapter: wgpu::Adapter,

    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}
```

在后续的各种操作（比如创建纹理和缓冲、写入缓冲等等）都要用到其中的内容。

而第二个首先，就是对 `wgpu::Buffer` 进行简单封装，以包含其所存储的数据类型信息（当然还有各种辅助函数，比如传入对象切片，自动调整缓冲大小并用 `bytemuck` 转换数据写入）：

```rust
pub struct WgpuBuffer<T: bytemuck::Pod + bytemuck::Zeroable> {
    pub buffer: wgpu::Buffer,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: bytemuck::Pod + bytemuck::Zeroable> WgpuBuffer<T> {
    // ...
    pub fn len(&self) -> u64 {
        self.size() / std::mem::size_of::<T>() as u64
    }
}
```

这样就可以计算得到缓冲中的 **对象个数**，这一点在执行 `pass.draw` 的时候会很有用。

然后，在 `RenderPass` 中要使用“资源”的无非以下几个：

- `Pipeline`：用于 `pass.set_pipeline(&pipeline)`
- `BindGroup`：用于 `pass.set_bind_group(index, &bindgroup, &[]);`
- `WgpuBuffer`：用于 `pass.set_vertex_buffer(slot, buffer_slice)` 以及 `pass.draw(0..buffer.len())`

而在创建 `RenderPass` 的过程中则需要使用以下几个“资源”：

- `target_texture` 的 `view`
- `multisample_texture` 的 `view`
- `depth_texture` 的 `view`



不同对象渲染方式的不同，必定对应着 `Pipeline` 类型的不同，主要可能体现在几点：

- 使用的顶点数据类型不同
- 使用的 uniform（其实也就是 `BindGroup` 不同）
- 使用的 shader 不同（**这其实是导致上面两个不同的根本**）

其实这些也就是创建 `Pipeline` 时所用的 `RenderPipelineDescriptor` 中的字段。

所以，其实 `Pipeline` 是最底层的资源，有几种不同的 `Pipeline` 决定了有几种不同的可渲染对象。而且整个画面的渲染过程不可避免的会需要手动指定 `Pipeline` 类型及其顺序。

因此，创建了 `RenderPipeline` 这个 Trait：

```rust
/// A render pipeline.
pub trait RenderPipeline: Deref<Target = wgpu::RenderPipeline> {
    /// The vertex type.
    type Vertex: PipelineVertex;

    /// The uniform type.
    type Uniforms: bytemuck::Pod + bytemuck::Zeroable;

    fn new(ctx: &WgpuContext) -> Self
    where
        Self: Sized;
}
```

每一个 `RenderPipeline` 都有唯一的关联的 `Vertex` 和 `Uniforms` 类型，对于 `Vertex` 又创建了一个 `PipelineVertex` Trait：

```rust
pub trait PipelineVertex: bytemuck::Pod + bytemuck::Zeroable {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a>;

    fn position(&self) -> Vec3;
    fn set_position(&mut self, position: Vec3);

    fn color(&self) -> Vec4;
    fn set_color(&mut self, color: Vec4);

    fn interpolate(&self, other: &Self, t: f32) -> Self;
}
```

考虑几乎全部定点类型会包含 `position` 和 `color` 两个属性，因此暂时把他们两个写死在 Trait 中。（其实更好的方式是创建一个基础结构，对不同的顶点数据类型实现向这个基础结构的转化）

同时，`Pipeline` 也是一个全局的资源，因此我创建了一个 `RanimContext` 来管理它：

```rust
pub struct RanimContext {
    pub(crate) wgpu_ctx: WgpuContext,
    pub(crate) pipelines: HashMap<TypeId, Box<dyn Any>>,
}

impl RanimContext {
    pub fn new() -> Self {
        let wgpu_ctx = pollster::block_on(WgpuContext::new());
        let pipelines = HashMap::<TypeId, Box<dyn Any>>::new();

        Self {
            wgpu_ctx,
            pipelines,
        }
    }

    pub(crate) fn get_or_init_pipeline<P: RenderPipeline + 'static>(&mut self) -> &P {
        let id = std::any::TypeId::of::<P>();
        self.pipelines
            .entry(id)
            .or_insert_with(|| Box::new(P::new(&self.wgpu_ctx)))
            .downcast_ref::<P>()
            .unwrap()
    }
}
```

这里利用了 `std::any` 抹除了 `RenderPipeline` 的类型，因为带有不同关联类型的相同 Trait 不能作为同一个类型。

那么对于渲染的对象，对应着 *manim*，在 *ranim* 中，同样有一个结构体 `Mobject`，但是它是一个泛型结构体：

```rust
#[derive(Clone)]
pub struct Mobject<Vertex: PipelineVertex> {
    pub id: Id,
    pub pipeline_id: std::any::TypeId,
    points: Arc<RwLock<Vec<Vertex>>>,
}
```

其中：

- `id` 在对象创建时生成，可用于唯一标识该对象
- `pipeline_id` 标识在渲染该对象时应使用的 `Pipeline`
- `points` 则是该对象的顶点数据

这只是渲染对象的逻辑表示，真正的渲染的对象应当与一个顶点缓冲相关联，而这个缓冲应当在进入场景时被创建，在渲染前被更新，在离开场景时被移除，因此实现了一个与 `Mobject` 对应的 `ExtractedMobject`，可以通过 `Mobject::extract(&self, ctx: &WgpuContext)` 创建：

```rust
pub struct ExtractedMobject<Vertex: PipelineVertex> {
    pub id: Id,
    pub pipeline_id: std::any::TypeId,
    pub points: Arc<RwLock<Vec<Vertex>>>,
    pub(crate) buffer: WgpuBuffer<Vertex>,
}

impl<Vertex: PipelineVertex> Mobject<Vertex> {
    pub(crate) fn extract(&self, ctx: &WgpuContext) -> ExtractedMobject<Vertex> {
        let Mobject {
            id,
            pipeline_id,
            points,
        } = self.clone();
        let buffer = WgpuBuffer::new_init(
            &ctx,
            &self.points.read().unwrap(),
            wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        );
        ExtractedMobject {
            id,
            pipeline_id,
            points,
            buffer,
        }
    }
}
```

其中：

- `id`、`pipeline_id` 保持与 Extract 前的逻辑上的 Mobject 一致
- `points` 则通过 `Arc<RwLock<>>` 与 Extract 前的数据共享

对应在 `Scene` 中，存储的则是 `ExtractedMobject`，不过由于这个结构带有泛型类型，因此需要对其进行类型擦除才能够将不同泛型类型的 `ExrtactedMobject` 视为相同类型的对象存储在一个集合中：

```rust
pub struct Scene {
    /// (Mobject's id, Mobject's pipeline id, Mobject)
    pub mobjects: HashMap<TypeId, Vec<(Id, Box<dyn Any>)>>,
    // ...
}
```

此处使用以 `Pipeline` 的 `TypeId` 为键的 `HashMap` 是为了方便后续进行 `batch`，而保存的是包含 Id 的元组的原因是为了避免在频繁添加/移除对象时对 `Box<dyn Any>` 进行 `downcast` 操作的性能消耗。

- `add_mobject` 接受 `Mobject` 的不可变引用为参数，通过 `extract` 创建对应的 `ExtractedMobject` 加入到场景对象中



### 动画

```mermaid
flowchart LR

Mobject --clone--> ClonedMobject

subgraph Scene
	ClonedMobject
end
```

每一个 `Mobject` 在被创建时都有一个唯一的 `id`。

每一个动画可以被考虑为一个 **初始 Mobject** 和 **结束 Mobject** 的某种插值，有的还涉及 Mobject 的添加/移除：

- 移动：对顶点数据的位置进行插值
- 颜色变换：对顶点数据的颜色进行插值
    - FadeIn：从透明度 1.0 到透明度 0.0，**移除 Mobject**
    - FadeOut：**添加 Mobject**，从透明度 0.0 到透明度 1.0
- Transform：对两个 Mobject 的顶点数据进行插值，**移除第一个 Mobject，添加第二个 Mobject**
- ......

对其进行简化，可以认为每一个动画都拥有一个对应的 `Mobject`：

- 播放动画前向场景中插入这个 `Mobject`
- 随着动画的进行，对这个 `Mobject` 进行插值更新
- 最终根据是否移除对场景的 mobjects 进行变更

所有的 **插值函数** 都可以抽象为一个 Trait：

```rust
pub trait AnimationFunc<Vertex: PipelineVertex> {
    #[allow(unused)]
    fn pre_anim(&mut self, mobject: &mut Mobject<Vertex>) {}

    fn interpolate(&mut self, mobject: &mut Mobject<Vertex>, alpha: f32);

    #[allow(unused)]
    fn post_anim(&mut self, mobject: &mut Mobject<Vertex>) {}
}
```



无多采样：

<img src="./assets/image-20241119221206801.png" alt="image-20241119221206801" style="zoom:50%;" />

有多采样（4x）：

<img src="./assets/image-20241119221507949.png" alt="image-20241119221507949" style="zoom:50%;" />

有多采样（4x + alpha_to_coverageenabled)：

<img src="./assets/image-20241119221432210.png" alt="image-20241119221432210" style="zoom:50%;" />



## 思路整理

manim 中的基本的对象是 `Mobject` 类，以及继承自它的 `VMobject` 类。

每一个 `Mobject` 中都有一个 `data` 字段，保存着 **顶点数据**：

- `Mobject` 的 `data` 的 dtype 是这样的：

    ```python
    np.dtype([
        ('point', np.float32, (3,)),
        ('rgba', np.float32, (4,)),
    ])
    ```

- `VMobject` 的 `data` 的 `dtype` 是这样的：

    ```python
    np.dtype([
        ('point', np.float32, (3,)),
        ('stroke_rgba', np.float32, (4,)),
        ('stroke_width', np.float32, (1,)),
        ('joint_angle', np.float32, (1,)),
        ('fill_rgba', np.float32, (4,)),
        ('base_normal', np.float32, (3,)),  # Base points and unit normal vectors are interleaved in this array
        ('fill_border_width', np.float32, (1,)),
    ])
    ```

同时，每一个 `Mobject` 都有一个 `shader_wrapper` 字段，保存着渲染管线相关的资源：

- `Mobject` 使用的是 `ShaderWrapper`：

    有一个 `ctx` 字段用于保存 *moderngl* 的上下文。

    有一个 `shader_folder` 字段用于保存着色器所在的文件夹位置。

    有一个 `program_code` 字段用于保存 `vertex_shader`、`geometry_shader`、`fragment_shader` 三个着色器的代码，从 `shader_folder` 中的 `vert`、`geom`、`frag` 三个文件中获取，默认为空。

    有一个 `program` 用于保存 *moderngl* 中的 Program，用 `ctx` 和 `program_code` 创建。

    同时有一个 `programs` 保存所有的 Program，在这里就只是 `[program]`

    有一个 `vert_attributes` 字段用于保存顶点字段名，由传入的 `vert_data` 的 `dtype.names` 得到。

    有一个 `vert_format` 用于保存 *vao* 的格式，由 *moderngl* 根据 `program` 和 `vert_attributes` 推断而得。

    有一个 `render_permitive` 来保存图元类型，默认为 `TRIANGLE_STRIP`

    然后是关键的 `vbo` 和 `vao` 字段：

    - `vbo` 会在 `read_in` 读入顶点数据时创建/更新
    - `vao` 则是由 `programs` 中的每一个 `program` 创建

- `VMobject` 则使用的是继承自 `ShaderWrapper` 的 `VShaderWrapper`：

    它的 `programs` 包含四个：`stroke_program`、`fill_program`、`fill_border_program`、`fill_depth_program`。其中 `stroke` 和 `fill_border` 其实是一个，只是 `frag_color` 被替换了。

    它们的代码都是直接忽略 `shader_folder`，从 `quadratic_bezier` 文件夹中加载的，其中有 `stroke`、`fill`、`depth` 三个子文件夹，每个子文件夹中又都有 `vert`、`geom`、`frag` 三个着色器文件。

    它们的 `vert_format` 和 `vert_attributes` 都是手动写的：

    ```python
    # Full vert format looks like this (total of 4x23 = 92 bytes):
    # point 3
    # stroke_rgba 4
    # stroke_width 1
    # joint_angle 1
    # fill_rgba 4
    # base_normal 3
    # fill_border_width 1
    self.stroke_vert_format = '3f 4f 1f 1f 16x 3f 4x'
    self.stroke_vert_attributes = ['point', 'stroke_rgba', 'stroke_width', 'joint_angle', 'unit_normal']
    
    self.fill_vert_format = '3f 24x 4f 3f 4x'
    self.fill_vert_attributes = ['point', 'fill_rgba', 'base_normal']
    
    self.fill_border_vert_format = '3f 20x 1f 4f 3f 1f'
    self.fill_border_vert_attributes = ['point', 'joint_angle', 'stroke_rgba', 'unit_normal', 'stroke_width']
    
    self.fill_depth_vert_format = '3f 40x 3f 4x'
    self.fill_depth_vert_attributes = ['point', 'base_normal']
    ```

    `render` 也是分别进行 stroke 和 fill 进行渲染，fill 中又包含 depth、fill_border、fill_tx 的渲染。

由于 *wgpu* 与 *moderngl* 的思路和概念有所区别，所以经过思考后，得出结论应当以 *渲染管线* 为单位去构筑整个程序：

实现若干个管线，对于不同的 `Mobject` 将其转化为对应管线的数据并调用相关渲染函数。