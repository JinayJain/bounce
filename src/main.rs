use std::{io, path::PathBuf};

use bounce::{
    color::Color,
    geometry::{Point, Vec3},
    image::Image,
    object::Object,
    scene::Scene,
    sky::Day,
};
use clap::Parser;

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

    let diffuse = scene.diffuse_material(Color::new(1.0, 0.3, 0.3));

    let model = Object::new("files/teapot.obj", diffuse).expect("Unable to create OBJ object");
    scene.add(model);

    let look_from = Point::new(8.0, 5.0, -5.0);
    let look_at = Point::new(0.0, 1.0, 0.0);
    let focus_dist = Vec3::from(look_at - look_from).len();

    scene.camera(
        look_from,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        30.0,
        aspect_ratio,
        0.0,
        focus_dist,
    );
    scene.sky(Day::new());

    let mut image = Image::new(image_width, image_height, Color::black());

    scene.render(&mut image, samples_per_pixel, max_depth);

    image.save(args.output)?;

    Ok(())
}
