use std::{io, path::PathBuf, rc::Rc};

use bounce::{
    camera::Camera,
    color::Color,
    geometry::{Point, Ray},
    image::Image,
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
}

fn ray_color(r: Ray, world: &HittableList) -> Color {
    if let Some(hit) = world.hit(r, 0.0..f32::INFINITY) {
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
    let args = Args::parse();

    // World
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));

    // Image
    let background_color = Color::new(0.0, 0.0, 0.0);
    let mut image = Image::new(400, (400 as f32 / (16.0 / 9.0)) as usize, background_color);
    let samples_per_pixel = args.samples_per_pixel;

    // Camera

    // TODO: Camera and image aspect ratio should match
    let camera = Camera::default();

    let width = image.width();
    let height = image.height();
    for (x, y, pixel) in image.pixels().progress() {
        let mut pixel_color = Color::new(0.0, 0.0, 0.0);

        for _ in 0..samples_per_pixel {
            let u = (x as f32 + rand::random::<f32>()) / (width - 1) as f32;
            let v = (y as f32 + rand::random::<f32>()) / (height - 1) as f32;

            let r = camera.ray_at(u, v);

            pixel_color += ray_color(r, &world);
        }

        let scale = 1.0 / (samples_per_pixel as f32);

        *pixel = Color::new(
            pixel_color.r() * scale,
            pixel_color.g() * scale,
            pixel_color.b() * scale,
        );
    }

    image.save(&args.output)?;

    println!(
        "Saved to {}.",
        args.output.into_os_string().into_string().unwrap()
    );

    Ok(())
}
