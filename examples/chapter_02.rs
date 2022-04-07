use ray_tracer::{tuple::{Tuple, Environment}, projectile::Projectile, color::Color, canvas::Canvas};

fn main() {
    let start = Tuple::new_point(0.0, 1.0, 0.0);
    let velocity = Tuple::new_vector(1.0, 1.8, 0.0).normalize().multiply(11.25);
    let mut p = Projectile::new(start, velocity);
    let color = Color::new(1.0, 0.0, 0.0);

    let gravity = Tuple::new_vector(0.0, -0.1, 0.0);
    let wind = Tuple::new_vector(-0.01, 0.0, 0.0);
    let e = Environment::new(gravity, wind);
    let mut c = Canvas::new(900, 550);

    loop {
        p  = p.tick(&e);

        c.write_pixel(p.point.x as usize, c.height - p.point.y as usize, color);

        if p.point.y <= 0.0 {
            break;
        }
    }

    c.write_to_file("images/chapter_02.ppm");
}