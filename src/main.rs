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

    let aspect_ratio = (image_width as f64) / (image_height as f64);

    let mut scene = Scene::new();

    scene.sky(Day::new());

    let mat = scene.diffuse_material(Color::new(0.1, 0.5, 0.1));
    let sphere_mat = scene.diffuse_material(Color::new(0.0, 0.3, 0.78));

    let offset = Point::new(1.0, -1.0, 0.0);
    let a = Point::new(0.0, 0.0, -1.0) + offset;
    let b = Point::new(0.0, 2.0, -1.0) + offset;
    let c = Point::new(-2.0, 0.0, -1.0) + offset;

    scene.triangle(c, b, a, &mat);

    scene.sphere(a, 0.1, &sphere_mat);
    scene.sphere(b, 0.1, &sphere_mat);
    scene.sphere(c, 0.1, &sphere_mat);

    let look_from = Point::new(0.0, 0.0, -10.0);
    let look_at = Point::new(0.0, 0.0, -1.0);
    let focus_dist = Vec3::from(look_at - look_from).len();

    scene.camera(
        look_from,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        30.0,
        aspect_ratio,
        0.1,
        focus_dist,
    );

    let mut image = Image::new(image_width, image_height, Color::new(0.0, 0.0, 0.0));

    scene.render(&mut image, samples_per_pixel, max_depth);

    image.save(&args.output)?;

    Ok(())
}
