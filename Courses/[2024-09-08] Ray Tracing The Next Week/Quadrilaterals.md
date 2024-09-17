# Quadrilaterals

说是四边形，其实是一个平行四边形，其定义如下：

- $\mathbf{Q}$：起始顶点
- $\mathbf{u}$：第一条边
- $\mathbf{v}$：第二条边

那么另外三个顶点就分别是 $\mathbf{Q} + \mathbf{u}$，$\mathbf{Q} + \mathbf{v}$，$\mathbf{Q} + \mathbf{u} + \mathbf{v}$。

```rust
pub struct Quad {
    q: Vec3,
    u: Vec3,
    v: Vec3,
    material: Arc<Box<dyn Material + Send + Sync>>,
}

impl Quad {
    pub fn new(q: Vec3, u: Vec3, v: Vec3, material: Arc<Box<dyn Material + Send + Sync>>) -> Self {
        Quad { q, u, v, material }
    }
}
```

计算一个四边形是否与光线相交的过程可以分为以下几步：

1. 获取四边形所在平面
2. 计算光线与平面交点
3. 检查交点是否落在平面内

一个平面的方程如下：
$$
Ax + By + Cz = D
$$
令平面的法向量 $(A, B, C)$ 为 $\mathbf{n}$，点的坐标为 $\mathbf{v}$，那么也就有：
$$
\begin{align}
\mathbf{n} \cdot \mathbf{v} &= D\\
\mathbf{n} \cdot (\mathbf{P} + t\mathbf{d}) &= D\\
\mathbf{n} \cdot \mathbf{P} + t\mathbf{n} \cdot \mathbf{d} &= D
\end{align}
$$
当 $\mathbf{n} \cdot \mathbf{d} \neq 0$ 即光线不与平面平行时可以解出：
$$
t = \dfrac{D - \mathbf{n} \cdot \mathbf{P}}{\mathbf{n} \cdot \mathbf{d}}
$$
那么法向量和 $D$ 怎么求呢？非常简单：
$$
\mathbf{u} \times \mathbf{v} = \mathbf{n}\\
\mathbf{n} \cdot \mathbf{Q}  = D
$$
接下来是将交点 $\mathbf{I}$ 放到四边形的坐标中，然后解出 $\alpha$ 和 $\beta$ 即可判断是否位于四边形内：
$$
\mathbf{I} = \mathbf{Q} + \alpha\mathbf{u} + \beta\mathbf{v}
$$
可以通过叉乘 $\mathbf{u}$ 或 $\mathbf{v}$ 来消去他们中的另一个，如下可以解出 $\alpha$：
$$
\alpha \mathbf{n} = (\mathbf{I} - \mathbf{Q}) \times \mathbf{v}\\
\alpha |\mathbf{n}|^2 = (\mathbf{I} - \mathbf{Q}) \times \mathbf{v} \cdot n\\
\alpha = \dfrac{(\mathbf{I} - \mathbf{Q}) \times \mathbf{v} \cdot \mathbf{n}}{\mathbf{n} \cdot \mathbf{n}}
$$
同理可以解出 $\beta$：
$$
\beta = \dfrac{\mathbf{u} \times (\mathbf{I} - \mathbf{Q}) \cdot \mathbf{n}}{\mathbf{n} \cdot \mathbf{n}}
$$
在实际的计算中，令 $\mathbf{w} = \dfrac{\mathbf{n}}{\mathbf{n} \cdot \mathbf{n}}$，由于 $\mathbf{n}$ 仅由 $\mathbf{u}$ 和 $\mathbf{v}$ 决定，所以 $\mathbf{w}$ 和 $\mathbf{n}$ 都可以在创建 Quad 时计算好缓存下来，节约后续计算资源。

```rust
pub struct Quad {
    q: Vec3,
    u: Vec3,
    v: Vec3,
    material: Arc<Box<dyn Material + Send + Sync>>,
    
    /// followings are cached values
    normal: Vec3, // normalized
    w: Vec3,
}

impl Quad {
    pub fn new(q: Vec3, u: Vec3, v: Vec3, material: Arc<Box<dyn Material + Send + Sync>>) -> Self {
        let n = u.cross(v);
        
        let w = n / n.dot(n);
        let normal = n.normalize();
        Quad { q, u, v, material, normal, w }
    }
}

impl Hittable for Quad {
    fn hit(&self, ray: &crate::Ray, t_range: std::ops::Range<f32>) -> Option<crate::HitRecord> {
        let denom = self.normal.dot(ray.direction);
        if denom.abs() < f32::EPSILON {
            return None;
        }

        let d = self.normal.dot(self.q);
        let t = (d - self.normal.dot(ray.origin)) / denom;
        if t < t_range.start || t > t_range.end {
            return None;
        }

        let point = ray.at(t);
        let point_on_plane = point - self.q;
        let u = self.w.dot(point_on_plane.cross(self.v));
        let v = self.w.dot(self.u.cross(point_on_plane));
        if !(0.0 <= u && u <= 1.0 && 0.0 <= v && v <= 1.0) {
            return None;
        }

        let front_face = self.normal.dot(ray.direction) > 0.0;

        Some(HitRecord {
            t,
            point,
            normal: self.normal,
            front_face,
            u,
            v,
            material: Some(self.material.clone()),
        })
    }
}

impl HasAabb for Quad {
    fn aabb(&self) -> Aabb {
        Aabb::new(self.q, self.q + self.u + self.v)
            .union(&Aabb::new(self.q + self.u, self.q + self.v))
    }
}
```

渲染一下下面的场景：

```rust
fn quads() -> impl AabbHittable + Send + Sync {
    let mut objects = Vec::new();

    objects.push(Box::new(Quad::new(
        Vec3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        Arc::new(Box::new(Lambertian::new(Arc::new(Box::new(
            SolidColor::new(Vec3::new(1.0, 0.2, 0.2)),
        ))))),
    )));
    objects.push(Box::new(Quad::new(
        Vec3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        Arc::new(Box::new(Lambertian::new(Arc::new(Box::new(
            SolidColor::new(Vec3::new(1.0, 0.2, 0.2)),
        ))))),
    )));
    objects.push(Box::new(Quad::new(
        Vec3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        Arc::new(Box::new(Lambertian::new(Arc::new(Box::new(
            SolidColor::new(Vec3::new(0.2, 1.0, 0.2)),
        ))))),
    )));
    objects.push(Box::new(Quad::new(
        Vec3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        Arc::new(Box::new(Lambertian::new(Arc::new(Box::new(
            SolidColor::new(Vec3::new(0.2, 0.2, 1.0)),
        ))))),
    )));
    objects.push(Box::new(Quad::new(
        Vec3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        Arc::new(Box::new(Lambertian::new(Arc::new(Box::new(
            SolidColor::new(Vec3::new(1.0, 0.5, 0.0)),
        ))))),
    )));
    objects.push(Box::new(Quad::new(
        Vec3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        Arc::new(Box::new(Lambertian::new(Arc::new(Box::new(
            SolidColor::new(Vec3::new(0.2, 0.8, 0.8)),
        ))))),
    )));

    let objects = objects
        .into_iter()
        .map(|obj| obj as Box<dyn AabbHittable + Send + Sync>)
        .collect();
    BvhNode::from_objects(objects)
}
```

```rust
let camera = Camera::new(aspect_ratio)
    .samples_per_pixel(500)
    .fov(80.0)
    .pos(Vec3::new(0.0, 0.0, 9.0))
    .look_at(Vec3::ZERO)
    .focus_distance(10.0);

// i9-9900k: cost: 419.357666s
// let objects = objects
//     .into_iter()
//     .map(|obj| obj as Box<dyn Hittable + Send + Sync>)
//     .collect();
// let world = List::from_objects(objects);
// camera.render_to_path(&world, image_width, "image.png");

// i9-9900k: ramdom axis cost: 76.5858159s
// i9-9900k: longest axis cost: 69.4391338s
let world = quads();
camera.render_to_path(&world, image_width, "image.png");
```

结果如下：

![quads-1](./assets/image-quads-1.png)

不错，搞定了么？

不，没有。

如果试试下面这个两个 Quad 在同一个平面的场景：

```rust
objects.push(Box::new(Quad::new(
    Vec3::new(-3.0, -2.0, 5.0),
    Vec3::new(0.0, 0.0, -4.0),
    Vec3::new(0.0, 4.0, 0.0),
    Arc::new(Box::new(Lambertian::new(Arc::new(Box::new(
        SolidColor::new(Vec3::new(1.0, 0.2, 0.2)),
    ))))),
)));
objects.push(Box::new(Quad::new(
    Vec3::new(-3.0, -2.0, 0.0),
    Vec3::new(0.0, 0.0, -4.0),
    Vec3::new(0.0, 4.0, 0.0),
    Arc::new(Box::new(Lambertian::new(Arc::new(Box::new(
        SolidColor::new(Vec3::new(1.0, 0.2, 0.2)),
    ))))),
)));
```

会发现输出的画面空无一物。

这是因为 Quad 的 AABB 有一个维度的长度是 0，如果两个 Quad 在同一个平面，会导致构造出的 BVH 树中包围他们两个的 AABB 同样有一个维度是 0，这会导致解出的 `t_min` 和 `t_max` 相等，进而导致他们被忽略掉。

解决办法也很简单，就是为 AABB 的每一个为 0 的维度稍微扩展一点：

```diff
- pub fn new(min: Vec3, max: Vec3) -> Self {
+ pub fn new(min: Vec3, mut max: Vec3) -> Self {
+     const DELTA: f32 = 0.0001;
+ 
+     if max.x - min.x < DELTA {
+         max.x += DELTA;
+     }
+     if max.y - min.y < DELTA {
+         max.y += DELTA;
+     }
+     if max.z - min.z < DELTA {
+         max.z += DELTA;
+     }

    Aabb { min, max }
}
```

![quads-2](./assets/image-quads-2.png)
