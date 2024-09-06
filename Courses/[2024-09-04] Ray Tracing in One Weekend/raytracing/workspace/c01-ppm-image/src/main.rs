use std::{
    fs::File,
    io::{BufWriter, Write},
};

use env_logger::Env;
use indicatif::{MultiProgress, ProgressBar};
use indicatif_log_bridge::LogWrapper;
use log::info;

fn main() {
    let logger = env_logger::Builder::from_env(Env::default().default_filter_or("info")).build();
    let level = logger.filter();
    let multi = MultiProgress::new();

    LogWrapper::new(multi.clone(), logger).try_init().unwrap();
    log::set_max_level(level);

    // Image
    let image_width = 256;
    let image_height = 256;

    let file = File::create("image.ppm").unwrap();
    let mut writer = BufWriter::new(file);

    // Render
    writer
        .write_all(format!("P3\n{} {}\n255", image_width, image_height).as_bytes())
        .unwrap();

    info!("generating image");
    let pg = multi.add(ProgressBar::new(image_height * image_width));
    for j in 0..image_height {
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.0;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            writer
                .write_all(format!("\n{} {} {}", ir, ig, ib).as_bytes())
                .unwrap();
            pg.inc(1);
        }
    }
    pg.finish();
    multi.remove(&pg);
}
