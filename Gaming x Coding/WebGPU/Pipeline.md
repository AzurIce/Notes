# Pipeline

Pipeline 用于表示 GPU 硬件、驱动等完成的一次完整的功能，在 WebGPU 中有两种 *pipeline*：

- `GPUComputePipeline`：计算管线，基于 compute shader
- `GPURenderPipeline`：渲染管线，基于 vertex shader 和 fragment shader

在某种意义上，一个 Pipeline 就相当于一个独立完整的 Program。

## 一、渲染管线

通过渲染管线绘制三角形的简化的整体结构如下图所示：

![webgpu-draw-diagram](./assets/webgpu-draw-diagram.svg)

在创建一个 `GPURenderPipeline` 时会需要提供一个 `GPURenderPipelineDescriptor`，在 *wgpu 0.23* 中，它长下面这样：

```rust
pub struct RenderPipelineDescriptor<'a> {
    // GPUObjectDescriptorBase fields
    pub label: Label<'a>,
    // GPUPipelineDescriptorBase fields
    pub layout: Option<&'a PipelineLayout>,
    // GPURenderPipelineDescriptor fields
    pub vertex: VertexState<'a>,
    pub primitive: PrimitiveState,
    pub depth_stencil: Option<DepthStencilState>,
    pub multisample: MultisampleState,
    pub fragment: Option<FragmentState<'a>>,
    // Others
    pub multiview: Option<NonZeroU32>,
    pub cache: Option<&'a PipelineCache>,
}
```

其中的各个属性均会控制渲染管线不同阶段的行为，一个渲染管线的工作阶段如下：

1. 获取顶点，由 `GPUVertexState.buffers` 控制
2. **顶点着色**，由 `GPUVertexState` 控制
3. 图元组装，由 `GPUPrimitiveState` 控制
4. 光栅化，由 `GPUPrimitiveState`、`GPUDepthStencilState` 和 `GPUMultisampleState` 控制
5. **片段着色**，由 `GPUFragmentState` 控制
6. 模板测试和操作，由 `GPUStencilState` 控制
7. 深度测试和写入，由 `GPUStencilState` 控制
8. 输出合并，由 `GPUFragmentState.targets` 控制

