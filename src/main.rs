use bounce::{
    color::Color,
    geometry::{Point, Ray, Vec3},
    object::{Hittable, Sphere},
};

fn ray_color(r: Ray) -> Color {
    let sphere = Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5);

    if let Some(hit) = sphere.hit(r, 0.0..f64::INFINITY) {
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
    // define image parameters
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    // define the parameters of the image plane in world space
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    // compute useful points and vectors about the world space / image plane
    let origin = Point::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin
        - (horizontal / 2.0).into()
        - (vertical / 2.0).into()
        - Vec3::new(0.0, 0.0, focal_length).into();

    // Required metadata at the beginning of PPM files
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;

            let r = Ray::new(
                origin,
                horizontal * u + vertical * v - origin.into() + lower_left_corner.into(),
            );

            let c = ray_color(r);

            println!("{}", c);
        }
    }
}
