pub fn bonded_linear(value: f32, max_value: f32) -> f32 {
    (1. - value).max(max_value)
}


pub fn inverse(value: f32, factor: f32, offset: f32) -> f32 {
    1. / (value * factor + offset)
}

pub fn above_zero(value: f32) -> f32 {
    if value > 0. {
        return 1.0;
    }
    0.
}
