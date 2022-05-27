use std::{io, path::PathBuf, sync::Arc};

use bounce::{
    camera::Camera,
    color::Color,
    geometry::{Point, Ray, Vec3},
    image::Image,
    material::{Lambertian, Material, Metal},
    object::{Hit, HittableList, Sphere},
};
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};

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
    let world = random_scene();

    // Image
    let background_color = Color::new(0.0, 0.0, 0.0);
    let mut image = Image::new(image_width, image_height, background_color);

    // Camera

    let aspect_ratio = image_width as f64 / image_height as f64;
    let aperture = 0.1;
    let look_from = Point::new(13.0, 2.0, 3.0);
    let look_at = Point::new(0.0, 0.0, 0.0);
    let camera = Camera::new(
        look_from,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
        aperture,
        10.0,
    );

    let width = image.width();
    let height = image.height();

    let world = Arc::new(world);

    let pb = ProgressBar::new((image.height() * image.width()) as u64);

    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {wide_bar} {pos}/{len} ({eta})")
            .progress_chars("#>-"),
    );

    image.apply_parallel(|x, y, pixel| {
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

        pb.inc(1);
    });

    image.save(&args.output)?;

    println!(
        "Saved to {}.",
        args.output.into_os_string().into_string().unwrap()
    );

    Ok(())
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    type MatRef = Arc<dyn Material>;

    world.add(Box::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5))),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let center = Point::new(
                a as f64 + 0.9 * rand::random::<f64>(),
                0.2,
                b as f64 + 0.9 * rand::random::<f64>(),
            );

            if Vec3::from(center - Point::new(4.0, 0.2, 0.0)).len() > 0.9 {
                let material: MatRef = if rand::random::<f64>() < 0.8 {
                    Arc::new(Lambertian::new(Color::new(
                        rand::random::<f64>() * rand::random::<f64>(),
                        rand::random::<f64>() * rand::random::<f64>(),
                        rand::random::<f64>() * rand::random::<f64>(),
                    )))
                } else {
                    Arc::new(Metal::new(
                        Color::new(
                            0.5 * (1.0 + rand::random::<f64>()),
                            0.5 * (1.0 + rand::random::<f64>()),
                            0.5 * (1.0 + rand::random::<f64>()),
                        ),
                        0.5 * rand::random::<f64>(),
                    ))
                };

                world.add(Box::new(Sphere::new(center, 0.2, Arc::clone(&material))));
            }
        }
    }

    world
}
