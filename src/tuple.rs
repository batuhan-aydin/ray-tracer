use std::ops::{Add, Sub};

use crate::helper::equal;

#[derive(Debug)]
pub struct Tuple {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}

#[derive(Debug)]
pub struct Environment {
    pub gravity: Tuple,
    pub wind: Tuple
}

impl Environment {
    pub fn new(gravity: Tuple, wind: Tuple) -> Environment {
        Environment { gravity: Tuple { w: 0.0, ..gravity}, 
                    wind: Tuple { w: 0.0, ..wind } }
    }
}

impl Tuple {
    fn new(x: f32, y: f32, z: f32, w: f32) -> Tuple {
        Tuple {
            x,
            y,
            z,
            w
        }
    }

    pub fn new_point(x: f32, y: f32, z: f32) -> Tuple {
        Tuple {
            x,
            y,
            z,
            w: 1_f32
        }
    }

    pub fn new_vector(x: f32, y: f32, z: f32) -> Tuple {
        Tuple {
            x,
            y,
            z,
            w: 0_f32
        }
    }

    pub fn is_a_point(&self) -> bool {
        self.w == 1.0
    }

    pub fn is_a_vector(&self) -> bool {
        self.w == 0.0
    }

    pub fn negate(&self) -> Tuple {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w
        }
    }

    pub fn multiply(&self, value: f32) -> Tuple {
        Tuple {
            x: self.x * value,
            y: self.y * value,
            z: self.z * value,
            w: self.w * value
        }
    }

    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Tuple {
        let magnitude = self.magnitude();
        Tuple {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
            w: self.w / magnitude
        }
    }

    pub fn dot(&self, other: &Tuple) -> f32 {
            self.x * other.x +
            self.y * other.y +
            self.z * other.z +
            self.w * other.w 
    }

    pub fn cross(&self, other: &Tuple) -> Tuple {
        Tuple::new_vector(self.y * other.z - self.z * other.y, 
                            self.z * other.x - self.x * other.z, 
                            self.x * other.y - self.y * other.x)
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        equal(self.x, other.x) && 
        equal(self.y, other.y) && 
        equal(self.z, other.z) && 
        equal(self.w, other.w)
    }
}

impl<'a, 'b> Add<&'b Tuple> for &'a Tuple {
    type Output = Tuple;

    fn add(self, rhs:&'b Tuple) -> Self::Output {
        Tuple::new(self.x + rhs.x, self.y + rhs.y,
                self.z + rhs.z, self.w + rhs.w)
    }
}


impl<'a, 'b> Sub<&'b Tuple> for &'a Tuple {
    type Output = Tuple;

    fn sub(self, rhs:&'b Tuple) -> Self::Output {
        Tuple::new(self.x - rhs.x, self.y - rhs.y,
                self.z - rhs.z, self.w - rhs.w)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuple_is_point_when_w_is_1() {
        let a = Tuple::new(4.3, -4.2, 3.1, 1.0);
        assert!(a.is_a_point());
        assert!(!a.is_a_vector());
    }

    #[test]
    fn tuple_is_vector_when_w_is_0() {
        let a = Tuple::new(4.3, -4.2, 3.1, 0.0);
        assert!(!a.is_a_point());
        assert!(a.is_a_vector());
    }

    #[test]
    fn creates_position_with_w_is_1() {
        let a = Tuple::new_point(4.0, -4.0, 3.0);
        assert_eq!(a, Tuple::new(4.0, -4.0, 3.0, 1.0));
    }

    #[test]
    fn creates_position_with_w_is_0() {
        let a = Tuple::new_vector(4.0, -4.0, 3.0);
        assert_eq!(a, Tuple::new(4.0, -4.0, 3.0, 0.0));
    }

    #[test]
    fn add_two_positions() {
        let a = Tuple::new(3.0, -2.0, 5.0, 1.0);
        let b = Tuple::new(-2.0, 3.0, 1.0, 0.0);
        let expected = Tuple::new(1.0, 1.0, 6.0, 1.0);
        let result = &a + &b;
        assert_eq!(result, expected);
    }

    #[test]
    fn subtract_two_positions() {
        let a = Tuple::new_point(3.0, 2.0, 1.0);
        let b = Tuple::new_point(5.0, 6.0, 7.0);
        let expected = Tuple::new_vector(-2.0, -4.0, -6.0);
        let result = &a - &b;
        assert_eq!(result, expected);
    }

    #[test]
    fn subtract_vector_from_point() {
        let a = Tuple::new_point(3.0, 2.0, 1.0);
        let b = Tuple::new_vector(5.0, 6.0, 7.0);
        let expected = Tuple::new_point(-2.0, -4.0, -6.0);
        let result = &a - &b;
        assert_eq!(result, expected);
    }

    #[test]
    fn subtract_two_vectors() {
        let v1 = Tuple::new_vector(3.0, 2.0, 1.0);
        let v2 = Tuple::new_vector(5.0, 6.0, 7.0);
        let expected = Tuple::new_vector(-2.0, -4.0, -6.0);
        let result = &v1 - &v2;
        assert_eq!(result, expected);
    }

    #[test]
    fn negate_a_position() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let expected = Tuple::new(-1.0, 2.0, -3.0, 4.0);
        let result = a.negate();
        assert_eq!(result, expected);
    }

    #[test]
    fn multiply_a_position() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a.multiply(3.5), Tuple::new(3.5, -7.0, 10.5, -14.0));
        assert_eq!(a.multiply(0.5), Tuple::new(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn compute_magnitude() {
        let v1 = Tuple::new_vector(1.0, 0.0, 0.0);
        let v2 = Tuple::new_vector(0.0, 1.0, 0.0);
        let v3 = Tuple::new_vector(0.0, 0.0, 1.0);
        let v4 = Tuple::new_vector(1.0, 2.0, 3.0);
        let v5 = Tuple::new_vector(-1.0, -2.0, -3.0);

        assert_eq!(v1.magnitude(), 1.0);
        assert_eq!(v2.magnitude(), 1.0);
        assert_eq!(v3.magnitude(), 1.0);
        assert_eq!(v4.magnitude(), 3.7416575);
        assert_eq!(v5.magnitude(), 3.7416575);
    }

    #[test]
    fn compute_normalizing() {
        let v1 = Tuple::new_vector(4.0, 0.0, 0.0);
        let v2 = Tuple::new_vector(1.0, 2.0, 3.0);

        assert_eq!(v1.normalize(), Tuple::new_vector(1.0, 0.0, 0.0));
        assert_eq!(v2.normalize(), Tuple::new_vector(0.26726, 0.53452, 0.80178));
    }

    #[test]
    fn magnitude_of_a_normalized_vector() {
        let v = Tuple::new_vector(1.0, 2.0, 3.0);
        let norm = v.normalize();
        // f32 is not precise enough so this fails, but it's fine
        //assert_eq!(norm.magnitude(), 1.0);
        assert!(equal(norm.magnitude(), 1.0));
    }

    #[test]
    fn dot_product_of_two_vectors_are_correct() {
        let a = Tuple::new_vector(1.0, 2.0, 3.0);
        let b = Tuple::new_vector(2.0, 3.0, 4.0);
        assert_eq!(a.dot(&b), 20.0);
    }

    #[test]
    fn cross_product_of_two_vectors_are_correct() {
        let a = Tuple::new_vector(1.0, 2.0, 3.0);
        let b = Tuple::new_vector(2.0, 3.0, 4.0);
        assert_eq!(a.cross(&b), Tuple::new_vector(-1.0, 2.0, -1.0));
        assert_eq!(b.cross(&a), Tuple::new_vector(1.0, -2.0, 1.0));
    }
}