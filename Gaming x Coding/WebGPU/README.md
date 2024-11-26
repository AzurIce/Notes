# WebGPU

## 目录

- [compute_texture_store](./playgrounds/vertex_from_storage)：用 Compute Shader 直接向 Storage Texture 写入
    
- [vertex_from_storage](./playgrounds/vertex_from_storage)：在 Vertex Shader 中使用 Storage Buffer 作为顶点数据来源
    
    涉及 Compute Shader、Storage Buffer、wgsl 内存布局
    
    
    
    https://sotrh.github.io/learn-wgpu/showcase/alignment/#how-to-deal-with-alignment-issues
    https://gist.github.com/teoxoy/936891c16c2a3d1c3c5e7204ac6cd76c#21-storage-address-space

## 概述

下图是使用 WebGPU 的 RenderPass 绘制三角形的简化结构：

![img](./assets/webgpu-draw-diagram.svg)



## 资源

[WebGPU Fundamentals](https://webgpufundamentals.org/webgpu/lessons/webgpu-fundamentals.html)

[WebGPU Samples](https://webgpu.github.io/webgpu-samples/)

[gpuweb/gpuweb: Where the GPU for the Web work happens!](https://github.com/gpuweb/gpuweb)：

- [WebGPU](https://gpuweb.github.io/gpuweb/)

- [WebGPU Shading Language](https://gpuweb.github.io/gpuweb/wgsl/)