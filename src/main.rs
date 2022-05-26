use std::{io, path::PathBuf, rc::Rc};

use bounce::{
    camera::Camera,
    color::Color,
    geometry::{Point, Ray},
    image::Image,
    material::{Lambertian, Material, Metal},
    object::{Hit, HittableList, Sphere},
};
use clap::Parser;
use indicatif::ProgressIterator;

#[derive(Parser)]
#[clap(about)]
struct Args {
    /// Where to save the output image
    #[clap(parse(from_os_str))]
    output: PathBuf,

    #[clap(long, short, default_value_t = 100)]
    samples_per_pixel: u32,

    #[clap(long, short, default_value_t = 50)]
    max_depth: u32,

    #[clap(long, default_value_t = 400)]
    width: usize,

    #[clap(long, default_value_t = 225)]
    height: usize,
}

/// Prevents bounced rays from hitting at the same point
const HIT_TOLERANCE: f64 = 0.001;

fn ray_color(r: Ray, world: &HittableList, depth: u32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(hit) = world.hit(r, HIT_TOLERANCE..f64::INFINITY) {
        if let Some((scattered, attenuation)) = hit.material.scatter(r, &hit) {
            return attenuation * ray_color(scattered, &world, depth - 1);
        }

        return Color::new(0.0, 0.0, 0.0);
    }

    let unit = r.direction().unit();
    let t = 0.5 * (unit.y() + 1.0);

    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let samples_per_pixel = args.samples_per_pixel;
    let max_depth = args.max_depth;
    let image_width = args.width;
    let image_height = args.height;

    // World
    let mut world = HittableList::new();

    type MatRef = Rc<dyn Material>;

    let silver: MatRef = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8)));
    let gold: MatRef = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2)));
    let red: MatRef = Rc::new(Lambertian::new(Color::new(0.3, 0.0, 0.0)));

    world.add(Box::new(Sphere::new(
        Point::new(0.0, 0.0, -1.0),
        0.5,
        Rc::clone(&red),
    )));
    world.add(Box::new(Sphere::new(
        Point::new(1.0, 0.0, -1.0),
        0.5,
        Rc::clone(&silver),
    )));
    world.add(Box::new(Sphere::new(
        Point::new(-1.0, 0.0, -1.0),
        0.5,
        Rc::clone(&gold),
    )));
    world.add(Box::new(Sphere::new(
        Point::new(0.0, -100.5, -1.0),
        100.0,
        Rc::clone(&red),
    )));

    // Image
    let background_color = Color::new(0.0, 0.0, 0.0);
    let mut image = Image::new(image_width, image_height, background_color);

    // Camera

    // TODO: Camera and image aspect ratio should match

    let aspect_ratio = image_width as f64 / image_height as f64;
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let camera = Camera::new(viewport_height, viewport_width, 1.0);

    let width = image.width();
    let height = image.height();
    for (x, y, pixel) in image.pixels().progress() {
        let mut pixel_color = Color::new(0.0, 0.0, 0.0);

        for _ in 0..samples_per_pixel {
            let u = (x as f64 + rand::random::<f64>()) / (width - 1) as f64;
            let v = (y as f64 + rand::random::<f64>()) / (height - 1) as f64;

            let r = camera.ray_at(u, v);

            pixel_color += ray_color(r, &world, max_depth);
        }

        let scale = 1.0 / (samples_per_pixel as f64);

        *pixel = Color::new(
            (pixel_color.r() * scale).sqrt(),
            (pixel_color.g() * scale).sqrt(),
            (pixel_color.b() * scale).sqrt(),
        );
    }

    image.save(&args.output)?;

    println!(
        "Saved to {}.",
        args.output.into_os_string().into_string().unwrap()
    );

    Ok(())
}
