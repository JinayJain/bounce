use std::sync::Arc;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, PlotConfiguration};

use bounce::{
    camera::Camera,
    color::Color,
    geometry::{Point, Ray, Vec3},
    image::Image,
    material::{Lambertian, Material},
    object::{
        bvh::{BvhTree, Primitive},
        Tri, Visible, VisibleList,
    },
    scene::Scene,
};
use rand::{prelude::StdRng, Rng, SeedableRng};

fn bench_sphere(c: &mut Criterion) {
    let mut scene = Scene::new();

    let ground_material = scene.diffuse_material(Color::new(0.8, 0.2, 0.1));
    let sphere_material = scene.metal_material(Color::new(0.5, 0.5, 0.1), 0.3);

    scene.sphere(Point::new(1.0, 0.0, -1.0), 0.5, &sphere_material);
    scene.sphere(Point::new(0.0, -100.5, -1.0), 100.0, &ground_material);

    scene.progress(false);

    let mut image = Image::new(100, 100, Color::black());
    let samples_per_pixel = 100;
    let max_depth = 50;

    c.bench_function("simple sphere", |b| {
        b.iter(|| {
            scene.render(&mut image, samples_per_pixel, max_depth);
        })
    });
}

fn bench_bvh_vs_list(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default().summary_scale(criterion::AxisScale::Logarithmic);

    let mut group = c.benchmark_group("intersection methods");
    group.plot_config(plot_config);

    let seed = 12391234;

    let cam = make_camera();

    for num_tris in [1, 10, 100, 1_000, 10_000, 100_000, 1_000_000].into_iter() {
        let bvh_tris = populate_tris(num_tris, seed);
        let bvh_tris: Vec<Arc<dyn Primitive>> = bvh_tris
            .into_iter()
            .map(|p| Arc::new(p) as Arc<dyn Primitive>)
            .collect();

        let list_tris = populate_tris(num_tris, seed);
        let list_tris: Vec<_> = list_tris
            .into_iter()
            .map(|p| Box::new(p) as Box<dyn Visible>)
            .collect();

        let bvh = BvhTree::build(bvh_tris);
        let list = VisibleList::from_list(list_tris);

        group.bench_with_input(BenchmarkId::new("BVH", num_tris), &num_tris, |b, i| {
            let mut rng = StdRng::seed_from_u64(seed);

            b.iter(|| {
                let r = gen_ray(&cam, &mut rng);

                bvh.bounce(r, &(0.0..f64::INFINITY));
            })
        });

        group.bench_with_input(BenchmarkId::new("list", num_tris), &num_tris, |b, i| {
            let mut rng = StdRng::seed_from_u64(seed);

            b.iter(|| {
                let r = gen_ray(&cam, &mut rng);

                list.bounce(r, &(0.0..f64::INFINITY));
            })
        });
    }

    group.finish();
}

fn make_camera() -> Camera {
    let look_from = Point::new(0.0, 0.0, -20.0);
    let look_at = Point::new(0.0, 0.0, 0.0);

    Camera::new(
        look_from,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        30.0,
        16.0 / 9.0,
        0.3,
        Vec3::from(look_at - look_from).len(),
    )
}

fn gen_ray(cam: &Camera, rng: &mut StdRng) -> Ray {
    let u = rng.gen_range(0.0..1.0);
    let v = rng.gen_range(0.0..1.0);

    cam.ray_at(u, v)
}

fn populate_tris(amount: u32, seed: u64) -> Vec<Tri> {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut prims = Vec::new();

    let point_range = 0.0..1.0;
    let offset_range = -10.0..10.0;

    let mat: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(1.0, 0.0, 0.0)));

    for _ in 0..amount {
        let a = Point::new(
            rng.gen_range(point_range.clone()),
            rng.gen_range(point_range.clone()),
            rng.gen_range(point_range.clone()),
        );
        let b = Point::new(
            rng.gen_range(point_range.clone()),
            rng.gen_range(point_range.clone()),
            rng.gen_range(point_range.clone()),
        );
        let c = Point::new(
            rng.gen_range(point_range.clone()),
            rng.gen_range(point_range.clone()),
            rng.gen_range(point_range.clone()),
        );

        let offset = Point::new(
            rng.gen_range(offset_range.clone()),
            rng.gen_range(offset_range.clone()),
            rng.gen_range(offset_range.clone()),
        );

        let tri = Tri::new(a + offset, b + offset, c + offset, Arc::clone(&mat));

        prims.push(tri);
    }

    prims
}

criterion_group!(benches, bench_bvh_vs_list, bench_sphere);
criterion_main!(benches);
