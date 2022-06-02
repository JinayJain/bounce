use criterion::{criterion_group, criterion_main, Criterion};

use bounce::{color::Color, geometry::Point, image::Image, scene::Scene};

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

criterion_group!(benches, bench_sphere);
criterion_main!(benches);
