use std::{io, rc::Rc};

use bounce::{
    camera::Camera,
    color::Color,
    geometry::{Point, Ray},
    image::Image,
    object::{Hit, HittableList, Sphere},
};
use indicatif::ProgressIterator;

fn ray_color(r: Ray, world: &HittableList) -> Color {
    if let Some(hit) = world.hit(r, 0.0..f64::INFINITY) {
        return 0.5
            * Color::new(
                hit.normal.x() + 1.0,
                hit.normal.y() + 1.0,
                hit.normal.z() + 1.0,
            );
    }

    let unit = r.direction().unit();
    let t = 0.5 * (unit.y() + 1.0);

    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn main() -> io::Result<()> {
    // World
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));

    // Image
    let background_color = Color::new(0.0, 0.0, 0.0);
    let mut image = Image::new(400, (400 as f32 / (16.0 / 9.0)) as usize, background_color);
    let samples_per_pixel = 100;

    // Camera

    // TODO: Camera and image aspect ratio should match
    let camera = Camera::default();

    let width = image.width();
    let height = image.height();
    for (x, y, pixel) in image.pixels().progress() {
        let mut pixel_color = Color::new(0.0, 0.0, 0.0);

        for _ in 0..samples_per_pixel {
            let u = (x as f64 + rand::random::<f64>()) / (width - 1) as f64;
            let v = (y as f64 + rand::random::<f64>()) / (height - 1) as f64;

            let r = camera.ray_at(u, v);

            pixel_color += ray_color(r, &world);
        }

        let scale = 1.0 / (samples_per_pixel as f64);

        *pixel = Color::new(
            pixel_color.r() * scale,
            pixel_color.g() * scale,
            pixel_color.b() * scale,
        );
    }

    image.save("result/output.ppm")?;

    println!("Saved to file.");

    Ok(())
}
