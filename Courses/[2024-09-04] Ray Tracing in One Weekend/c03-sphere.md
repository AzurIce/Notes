# c03-sphere

## 一、光线与球体的相交

简单的解析几何，光线 $\mathbf{P}(t) = \mathbf{Q} + t\mathbf{d}$ 与半径为 $r$ 中心位于 $\mathbf{C}$ 的球体相交的方程为：
$$
\begin{align}
r^2 &= (\mathbf{C} - \mathbf{P}) \cdot (\mathbf{C} - \mathbf{P})\\
r^2 &= (\mathbf{C} - (\mathbf{Q} + t\mathbf{d})) \cdot (\mathbf{C} - (\mathbf{Q} + t\mathbf{d}))\\
r^2 &= ((\mathbf{C} - \mathbf{Q}) - t\mathbf{d}) \cdot ((\mathbf{C} - \mathbf{Q}) - t\mathbf{d})\\
\end{align}
$$
令 $\mathbf{O_c} = \mathbf{C} - \mathbf{Q}$ 则可写作：
$$
\begin{align}
r^2 &= t^2\mathbf{d}^2 - 2t\mathbf{d}\cdot\mathbf{O_c} + \mathbf{O_c}^2\\
0 &= \mathbf{d}^2t^2 - 2\mathbf{d}\cdot\mathbf{O_c}t + \mathbf{O_c}^2 - r^2\\
\end{align}
$$


根据根的数量即可得到光线与球体的相交情况：
$$
\begin{align}
a &= \mathbf{d}^2\\
b &= 2 \mathbf{d} \cdot \mathbf{O_c}\\
c &= \mathbf{O_c}^2 - r^2
\end{align}
$$

$$
\Delta = b^2 - 4ac = 4(\mathbf{d} \cdot \mathbf{O_c})^2 - 4\mathbf{d}^2(\mathbf{O_c}^2 - r^2)
$$

```rust
fn hit_sphere(center: Vec3, radius: f32, ray: &Ray) -> bool {
    let oc = center - ray.origin;
    let a = ray.direction.dot(ray.direction);
    let b = oc.dot(ray.direction);
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - a * c;
    discriminant - 0.0 >= f32::EPSILON
}
```

```diff
pub fn ray_color(ray: &Ray) -> Vec3 {
+     if hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, ray) {
+         return Vec3::new(1.0, 0.0, 0.0);
+     }

    let unit_direction = ray.direction.normalize();
    // ...
}
```

可以得到这样的一张图像：

![image_c03](./assets/image_c03.png)

不过会有一个问题，如果将球心坐标设定为 $z=1$，会得到同样的一张图（因为方程计算的是光线直线与球体的交点），这个问题会在之后的环节解决。
