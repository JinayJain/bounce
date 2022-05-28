use std::{io, path::PathBuf};

use bounce::{
    color::Color,
    geometry::{Point, Vec3},
    image::Image,
    scene::Scene,
    sky::Day,
};
use clap::Parser;
use rand::Rng;

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

fn main() -> io::Result<()> {
    let args = Args::parse();

    let samples_per_pixel = args.samples_per_pixel;
    let max_depth = args.max_depth;
    let image_width = args.width;
    let image_height = args.height;

    let mut scene = Scene::new();

    scene.sky(Day::new());

    let metal = scene.metal_material(Color::new(0.8, 0.8, 0.9), 0.8);
    let glass = scene.dielectric_material(1.5);

    let num_diffuse = 50;
    let num_glass = 30;

    let coord_range = -10.0..10.0;
    let mut rng = rand::thread_rng();

    for _ in 0..num_diffuse {
        let diffuse = scene.diffuse_material(Color::new(
            rng.gen_range(0.0..1.0),
            rng.gen_range(0.0..1.0),
            rng.gen_range(0.0..1.0),
        ));

        scene.sphere(
            Point::new(
                rng.gen_range(coord_range.clone()),
                rng.gen_range(0.0..coord_range.clone().end),
                rng.gen_range(coord_range.clone()),
            ),
            0.5,
            &diffuse,
        );
    }

    for _ in 0..num_glass {
        scene.sphere(
            Point::new(
                rng.gen_range(coord_range.clone()),
                rng.gen_range(0.0..coord_range.clone().end),
                rng.gen_range(coord_range.clone()),
            ),
            0.5,
            &glass,
        );
    }

    scene.plane(Point::new(0.0, -0.5, 0.0), Vec3::new(0.0, 1.0, 0.0), &metal);

    let look_from = Point::new(10.0, 10.0, -35.0);
    let look_at = Point::new(0.0, 5.0, -1.0);
    scene.camera(
        look_from,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        40.0,
        (image_width as f64) / (image_height as f64),
        0.1,
        Vec3::from(look_from - look_at).len(),
    );

    let mut image = Image::new(image_width, image_height, Color::new(0.0, 0.0, 0.0));

    scene.render(&mut image, samples_per_pixel, max_depth);

    image.save(&args.output)?;

    Ok(())
}
