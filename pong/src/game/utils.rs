// math utils
pub fn clamp(val: &mut f32, min: f32, max: f32) {
    if *val > max {
        *val = max;
    } else if *val < min {
        *val = min;
    }
}

pub fn max(val: f32, max: f32) -> f32 {
    if val < max {
        val
    } else {
        max
    }
}