use std::{io, path::PathBuf, sync::Arc};

use bounce::{
    color::Color,
    geometry::{Point, Vec3},
    image::Image,
    material::{Lambertian, Material},
    object::{
        bvh::{BvhTree, Primitive},
        Tri,
    },
    scene::Scene,
    sky::Day,
};
use clap::Parser;
use rand::{thread_rng, Rng};

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

    // make_triangles(120_000, &mut scene);

    let metal = scene.metal_material(Color::new(1.0, 1.0, 1.0), 0.3);
    let diffuse = scene.diffuse_material(Color::new(1.0, 0.0, 1.0));
    let glass = scene.dielectric_material(1.5);
    scene.object("files/teapot.obj", &diffuse);

    let plane_mat = scene.diffuse_material(Color::new(0.2, 0.2, 0.01));
    // scene.plane(
    //     Point::new(0.0, 0.0, 0.0),
    //     Vec3::new(0.0, 1.0, 0.0),
    //     &plane_mat,
    // );

    // scene.sphere(Point::new(3.0, 0.0, 2.0), 0.5, &tri_mat);
    // scene.triangle(
    //     Point::new(5.0, 0.0, 0.0),
    //     Point::new(0.0, 0.0, 0.0),
    //     Point::new(0.0, 3.0, 0.0),
    //     &teapot_mat,
    // );

    let look_from = Point::new(3.0, 3.0, 10.0);
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

fn make_triangles(num: usize, scene: &mut Scene) {
    let diffuse = scene.diffuse_material(Color::new(0.0, 0.3, 0.8));
    let metal = scene.metal_material(Color::new(0.3, 0.2, 0.0), 0.5);
    let glass = scene.dielectric_material(1.5);

    let mut rng = thread_rng();
    let a = Point::new(0.0, 0.0, 0.0);
    let b = Point::new(1.0, 0.0, 0.0);
    let c = Point::new(0.0, 1.0, 0.0);
    let coord_range = -10.0..10.0;
    for _ in 0..num {
        let offset = Point::new(
            rng.gen_range(coord_range.clone()),
            0.0,
            rng.gen_range(coord_range.clone()),
        );

        let mat = {
            let num: f64 = rng.gen();

            match num {
                num if num < 0.33 => &diffuse,
                num if num < 0.66 => &metal,
                _ => &glass,
            }
        };
        // scene.triangle(a + offset, b + offset, c + offset, &mat);
        scene.sphere(offset, 0.2, mat);
    }
}
