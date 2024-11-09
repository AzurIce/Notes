# p6-render-to-texture

![image-20241109115144350](./assets/image.gif)

```mermaid
flowchart LR
PbrCubeFirstPass --Camera3d--> Image

PointLight --Camera3d--> Image

subgraph StandardMaterial
	Image
end

subgraph PbrCubeMainPass
	StandardMaterial
end

PointLight --Camera3d--> Screen
PbrCubeMainPass --Camera3d--> Screen
```

