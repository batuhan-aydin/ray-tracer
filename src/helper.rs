const EPSILON: f32 = 0.00001;

pub enum Axis {
    X,
    Y,
    Z
}

pub fn equal(a: f32, b: f32) -> bool{
    f32::abs(a - b) < EPSILON
}
