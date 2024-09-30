pub mod plugin;

use std::ops::Range;

use rand::random;

pub fn rand_events(cnt: usize, x_range: Range<u16>, y_range: Range<u16>) -> Vec<CDEvent> {
    (0..cnt)
        .map(|_idx| CDEvent {
            t: 0,
            x: random::<u16>() % (x_range.end - x_range.start) + x_range.start,
            y: random::<u16>() % (y_range.end - y_range.start) + y_range.start,
            p: random(),
        })
        .collect()
}

pub struct CDEvent {
    pub t: u64,
    pub x: u16,
    pub y: u16,
    pub p: bool,
}
