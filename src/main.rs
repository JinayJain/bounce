use bounce::{
    camera::Camera,
    color::Color,
    geometry::{Point, Ray, Vec3},
    object::{Hittable, HittableList, Sphere},
};

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

fn main() {
    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 100;

    // Camera
    let camera = Camera::default();

    // Required metadata at the beginning of PPM files
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);

            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rand::random::<f64>()) / (image_width - 1) as f64;
                let v = (j as f64 + rand::random::<f64>()) / (image_height - 1) as f64;

                let r = camera.ray_at(u, v);

                // TODO: fix clamped add
                pixel_color += ray_color(r, &world);
            }

            let scale = 1.0 / (samples_per_pixel as f64);

            let final_color = Color::new(
                pixel_color.r() * scale,
                pixel_color.g() * scale,
                pixel_color.b() * scale,
            );

            println!("{}", final_color);
        }
    }
}
