use bounce::{
    color::Color,
    geometry::{Double, Point, Ray, Vec3},
};

fn ray_color(r: Ray) -> Color {

    let sphere_center = Point::new(0.0, 0.0, -1.0);
    let t = hit_sphere(sphere_center, 0.5, r);

    if t > 0.0 {
        // compute normal vector from sphere center to hit location
        let normal: Vec3<f64> = (r.at(t) - sphere_center).into();
        let normal = normal.unit();

        return 0.5 * Color::new(normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0);
    }

    let unit = r.direction().unit();
    let t = 0.5 * (unit.y() + 1.0);

    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

/// Checks whether a given ray will hit the a sphere of a given radius and center
fn hit_sphere(center: Point<Double>, radius: Double, r: Ray) -> f64 {
    // t^2 b . b + 2tb . (A - C) + (A - C) . (A - C) - r^2 = 0
    // A = origin
    // b = direction
    // C = circle center
    // r = radius

    let direction = r.direction();
    let ray_origin = r.origin();

    let offset = Vec3::from(ray_origin - center);

    // Form quadratic for sphere intersection checking (simplified)
    let a = direction.len().powi(2);
    let half_b = direction.dot(offset);
    let c = offset.len().powi(2) - radius.powi(2);

    let discriminant = half_b.powi(2) - a * c;

    if discriminant < 0.0 {
        // unable to find a hit
        -1.0
    } else {
        // return the full solution to the quadratic
        (-half_b - discriminant.sqrt()) / a
    }
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
