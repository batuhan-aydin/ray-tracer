use crate::tuple::{Tuple, Environment};


#[derive(Debug)]
pub struct Projectile {
    pub point: Tuple,
    velocity: Tuple
}

impl Projectile {
    pub fn new(point: Tuple, velocity: Tuple) -> Projectile {
        Projectile {
            point: Tuple { w: 1.0, ..point },
            velocity: Tuple {w: 0.0, ..velocity }
        }
    }

    pub fn tick(self, env: &Environment) -> Projectile {
        let position = &self.point + &self.velocity;
        let velocity = &(&self.velocity + &env.gravity) + &env.wind;
        Projectile { point: position, velocity: velocity }
    }
}