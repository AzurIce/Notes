# C05-refactor

全局 Logger、Camera 重构、使用 Range 表示 tmin 与 tmax

---

## 一、全局 Logger

首先封装一个全局的 logger：

```rust
pub struct Logger {
    multi: MultiProgress,
}

impl Logger {
    pub fn init() -> Self {
        let logger =
            env_logger::Builder::from_env(Env::default().default_filter_or("info")).build();
        let level = logger.filter();
        let multi = MultiProgress::new();

        LogWrapper::new(multi.clone(), logger).try_init().unwrap();
        log::set_max_level(level);

        Self { multi }
    }

    pub fn multi(&self) -> &MultiProgress {
        &self.multi
    }
}

pub fn logger() -> &'static Logger {
    static LOGGER: OnceLock<Logger> = OnceLock::new();
    LOGGER.get_or_init(|| Logger::init())
}
```

## 二、Camera

然后封装一下 Camera。

```rust
pub struct Camera {
    focal_length: f32,
    aspect_ratio: f32,
    viewport_height: f32,
    viewport_width: f32,
    camera_center: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * aspect_ratio;

        Self {
            focal_length: 1.0,
            aspect_ratio,
            viewport_height,
            viewport_width,
            camera_center: Vec3::ZERO,
        }
    }
}

impl Camera {
    pub fn new(aspect_ratio: f32) -> Self {
        let viewport_height = 2.0;
        let viewport_width = viewport_height * aspect_ratio;

        Camera {
            aspect_ratio,
            viewport_height,
            viewport_width,
            ..Default::default()
        }
    }

    pub fn camera_center(mut self, camera_center: Vec3) -> Self {
        self.camera_center = camera_center;
        self
    }

    pub fn focal_length(mut self, focal_length: f32) -> Self {
        self.focal_length = focal_length;
        self
    }
}

impl Camera {
    pub fn render_to_ppm(
        &self,
        world: &Vec<Box<dyn Hittable>>,
        output_width: u32,
        path: impl AsRef<Path>,
    ) {
        let file = File::create(path).unwrap();
        let mut writer = BufWriter::new(file);

        // Camera
        let viewport_u = Vec3::new(self.viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -self.viewport_height, 0.0);

        let output_height = (output_width as f32 / self.aspect_ratio) as u32;

        let pixel_delta_u = viewport_u / output_width as f32;
        let pixel_delta_v = viewport_v / output_height as f32;

        let viewport_upper_left = self.camera_center
            - Vec3::new(0.0, 0.0, self.focal_length)
            - viewport_u / 2.0
            - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        info!("generating image...");
        writer
            .write_all(format!("P3\n{} {}\n255", output_width, output_height).as_bytes())
            .unwrap();

        let multi = logger().multi();
        let pb = multi.add(ProgressBar::new(output_height as u64));
        for j in 0..output_height {
            for i in 0..output_width {
                let pixel_center =
                    pixel00_loc + (i as f32 * pixel_delta_u) + (j as f32 * pixel_delta_v);
                let ray = Ray::new(self.camera_center, pixel_center - self.camera_center);

                let color = ray_color(&ray, world);
                let color_u8 = (255.999 * color).clamp(Vec3::ZERO, Vec3::splat(255.0));
                writer
                    .write_all(
                        format!(
                            "\n{} {} {}",
                            color_u8.x as u32, color_u8.y as u32, color_u8.z as u32
                        )
                        .as_bytes(),
                    )
                    .unwrap();
                pb.inc(1);
            }
        }
        pb.finish();
        multi.remove(&pb);
    }
}
```

## 三、使用 Range 表示 tmin 与 tmax

```rust
pub trait Hittable {
    fn hit(&self, ray: &Ray, t_range: Range<f32>) -> Option<HitRecord>;
}
```

用 `t_range.start()` 替代 `tmin`，用 `t_range_end` 替代 `tmax`。

```rust
impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_range: Range<f32>) -> Option<HitRecord> {
        // ...
        if t < t_range.start || t_range.end < t {
            t = (-b + sqrtd) / a;
        }
        if t < t_range.start || t_range.end < t {
            return None;
        }

        // ...
    }
}
```



```rust
impl Hittable for World {
    fn hit(&self, ray: &Ray, t_range: Range<f32>) -> Option<HitRecord> {
        let mut closest = t_range.end;
        let mut hit_record = None;
        for object in self.iter() {
            if let Some(record) = object.hit(ray, t_range.start..closest) {
                closest = record.t;
                hit_record = Some(record);
            }
        }
        hit_record
    }
}
```

## 四、修改后的 main

简洁而优雅

```rust
use c05_camera::{camera::Camera, Hittable, Sphere};
use glam::Vec3;

// Ideal aspect ratio
const ASPECT_RATIO: f32 = 16.0 / 9.0;

fn main() {
    // Setup world
    let world: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)),
    ];

    // Image
    let image_width = 400;
    let image_height = (image_width as f32 / ASPECT_RATIO) as usize;
    // let (image_width, image_height) = (400, 225);
    let aspect_ratio = image_width as f32 / image_height as f32; // real aspect ratio

    let camera = Camera::new(aspect_ratio);

    camera.render_to_ppm(&world, image_width, "image_c05.ppm");
}

```





