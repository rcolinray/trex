use std::time::Duration;

pub fn calc_millis(dt: Duration) -> f32 {
    (dt.as_secs() as f32 * 1000.0) + (dt.subsec_nanos() as f32 / 1000000.0)
}
