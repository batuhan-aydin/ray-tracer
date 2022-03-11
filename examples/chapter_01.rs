use ray_tracer::{projectile::Projectile, tuple::{Tuple, Environment}};

fn main() {
    let mut p = Projectile::new(Tuple::new_point(0.0, 1.0, 0.0), 
    Tuple::new_vector(1.0, 1.0, 0.0).normalize());
    let e = Environment::new(Tuple::new_vector(0.0, -0.1, 0.0), 
    Tuple::new_vector(-0.01, 0.0, 0.0));
    
    loop {
        p  = p.tick(&e);
        println!("{:?}", p);
    
        if p.point.y <= 0.0 {
            break;
        }
    }
    
}