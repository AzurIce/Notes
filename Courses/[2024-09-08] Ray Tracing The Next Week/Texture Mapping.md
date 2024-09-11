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

