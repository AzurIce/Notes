# Texture Mapping

纹理映射，“纹理”是效果，“映射”是将一个空间映射到另一个空间的数学过程。

其中“纹理”并不局限于颜色信息，同样可以是亮度信息、凹凸信息，甚至是物体部分的存在与否的信息。

最常见的纹理映射就是将一张图片“贴”到一个物体表面，在实现中，目标物体上会有一个纹理坐标（一般表示为 $(u, v)$），纹理映射的过程即根据纹理坐标获取对应颜色的过程。

## 一、Texture Trait

根据上面的描述，不难将纹理抽象出一个 `Texture` Trait：

```rust
pub trait Texture {
    fn value(&self, u: f32, v: f32) -> Vec3;
}
```

## 二、实色材质

当然就是对于任何的 $(u, v)$ 都返回相同的一个固定的颜色：

```rust
pub struct SolidColor {
    albedo: Vec3,
}

impl SolidColor {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f32, _v: f32) -> Vec3 {
        self.albedo
    }
}
```

## 三、令 Material 使用 Texture

将 `Lambertian` 和 `Metal` 中的 `albedo` 都更改为 `texture: Arc<Box<dyn Texture>>`，并在 `scatter` 方法中使用 `texture` 的 `value()` 方法获取 `attenuation`。

那 `u` 和 `v` 怎么来？

所以还需要更改 `HitRecord`，添加 `u` 和 `v`。在未来需要为物体的 `hit` 方法实现计算碰撞点 `u` 和 `v` 的逻辑，不过对于目前的实色材质，可以随便填一个。

```diff
pub struct Lambertian {
-     albedo: Vec3,
+     texture: Arc<Box<dyn Texture + Send + Sync>>,

impl Lambertian {
-     pub fn new(albedo: Vec3) -> Self {
-         Lambertian { albedo }
+     pub fn new(texture: Arc<Box<dyn Texture + Send + Sync>>) -> Self {
+         Lambertian { texture }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, record: &HitRecord) -> Option<(Vec3, Ray)> {
        let mut scatter_direction = record.normal + random_in_unit_sphere();
        if scatter_direction.length_squared() <= f32::EPSILON {
            scatter_direction = record.normal;
        }

        let scattered_ray = Ray::new(record.point, scatter_direction);
-         Some((self.albedo, scattered_ray))
+         let attenuation = self.texture.value(record.u, record.v);
+         Some((attenuation, scattered_ray))
    }
}
```

```diff
pub struct Metal {
-     albedo: Vec3,
+     texture: Arc<Box<dyn Texture + Send + Sync>>,
    fuzz: f32,
}

impl Metal {
-     pub fn new(albedo: Vec3) -> Self {
+     pub fn new(texture: Arc<Box<dyn Texture + Send + Sync>>) -> Self {
        Metal {
-             albedo,
+             texture,
            fuzz: 0.0,
        }
    }

    pub fn fuzz(mut self, fuzz: f32) -> Self {
        self.fuzz = fuzz;
        self
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = ray.direction.reflect(record.normal);
        let reflected = reflected.normalize() + self.fuzz * random_in_unit_sphere();

        if reflected.dot(record.normal) > 0.0 {
            let scattered_ray = Ray::new(record.point, reflected);
-             Some((self.albedo, scattered_ray))
+             let attenuation = self.texture.value(record.u, record.v);
+             Some((attenuation, scattered_ray))
        } else {
            None
        }
    }
}
```

## 四、固体纹理 —— 棋盘网格纹理

固体纹理，又称空间纹理，仅取决于空间中每一个点的位置（相当于为空间中的每一个点设置了对应的颜色）

为了实现这种材质，需要对 `Texture` Trait 做一点修改，引入点的空间坐标：

```rust
pub trait Texture {
    fn value(&self, u: f32, v: f32, point: Vec3) -> Vec3;
}
```

棋盘网格纹理的最简单的实现方法就是对空间坐标的每一个分量取整，但是要注意不能直接忽略小数部分（否则 0 两侧颜色会相同），要向下或向上取整。然后对分量求和，根据其奇偶决定颜色。在这个基础之上可以添加一个缩放系数来控制棋盘的缩放比例。

```rust
pub struct SolidCheckerTexture {
    inv_scale: f32,
    even: Arc<Box<dyn Texture + Send + Sync>>,
    odd: Arc<Box<dyn Texture + Send + Sync>>,
}

impl SolidCheckerTexture {
    pub fn new(scale: f32, even: Arc<Box<dyn Texture + Send + Sync>>, odd: Arc<Box<dyn Texture + Send + Sync>>) -> Self {
        let inv_scale = 1.0 / scale;
        Self { inv_scale, even, odd }
    }
}

impl Texture for SolidCheckerTexture {
    fn value(&self, u: f32, v: f32, point: Vec3) -> Vec3 {
        let p = point.to_array().map(|v| (self.inv_scale * v).floor() as i32).iter().sum::<i32>();

        if p % 2 == 0 {
            self.even.value(u, v, point)
        } else {
            self.odd.value(u, v, point)
        }
    }
}
```

然后修改一下作为地面的最大的那个球的材质：

```rust
objects.push(Box::new(Sphere::new(
    Vec3::new(0.0, -1000.0, 0.0),
    1000.0,
    Box::new(Lambertian::new(Arc::new(Box::new(CheckerTexture::new(
        0.5,
        Arc::new(Box::new(SolidColor::new(Vec3::new(0.2, 0.3, 0.1)))),
        Arc::new(Box::new(SolidColor::new(Vec3::new(0.9, 0.9, 0.9)))),
    ))))),
)));
```

效果：

![check-board](./assets/image-texture-mapping-checker-board.png)

看起来不错，再看看另一个场景：

```rust
fn checkered_spheres() -> impl AabbHittable + Send + Sync {
    let mut objects = Vec::new();

    let checker_texture: Arc<Box<dyn Texture + Send + Sync>> =
        Arc::new(Box::new(SolidCheckerTexture::new(
            0.32,
            Arc::new(Box::new(SolidColor::new(Vec3::new(0.2, 0.3, 0.1)))),
            Arc::new(Box::new(SolidColor::new(Vec3::new(0.9, 0.9, 0.9)))),
        )));
    let material: Arc<Box<dyn Material + Send + Sync>> =
        Arc::new(Box::new(Lambertian::new(checker_texture)));

    objects.push(Box::new(Sphere::new(
        Vec3::new(0.0, -10.0, 0.0),
        10.0,
        material.clone(),
    )));
    objects.push(Box::new(Sphere::new(
        Vec3::new(0.0, 10.0, 0.0),
        10.0,
        material.clone(),
    )));

    let objects = objects
        .into_iter()
        .map(|obj| obj as Box<dyn AabbHittable + Send + Sync>)
        .collect();
    BvhNode::from_objects(objects)
}
```

![check-board](./assets/image-texture-mapping-checkered-spheres.png)

直观上看，好像有点不对？但是实际上空间网格被球体裁切所得到的图形就是这样的：

<img src="./assets/image-20240911213051621.png" alt="image-20240911213051621" style="zoom:67%;" />

那么如何解决？答案就是为球体构造一个 $(u, v)$ 坐标。

## 五、球体纹理坐标

