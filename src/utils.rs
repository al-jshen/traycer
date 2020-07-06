pub fn rand_in_range(low: f32, high: f32) -> f32 {
    (high - low) * fastrand::f32() + low
}
