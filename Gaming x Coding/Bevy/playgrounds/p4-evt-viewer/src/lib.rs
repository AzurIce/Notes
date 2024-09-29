use std::ops::Range;

use bevy::{math::Vec3, prelude::Transform};
use rand::random;

pub fn rand_events(cnt: usize, x_range: Range<u16>, y_range: Range<u16>) -> Vec<Event> {
    (0..cnt)
        .map(|_idx| Event {
            t: 0,
            x: random::<u16>() % (x_range.end - x_range.start) + x_range.start,
            y: random::<u16>() % (y_range.end - y_range.start) + y_range.start,
            p: random(),
        })
        .collect()
}

pub struct Event {
    pub t: u64,
    pub x: u16,
    pub y: u16,
    pub p: bool,
}

impl Event {
    pub fn transform(&self) -> Transform {
        Transform::default().with_translation(Vec3::new(
            self.x as f32 - 1280.0 / 2.0,
            self.y as f32 - 720.0 / 2.0,
            0.0,
        ))
    }
}
