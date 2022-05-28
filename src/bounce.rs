use std::sync::Arc;

use crate::{
    camera::Camera,
    color::Color,
    geometry::{Point, Ray, Vec3},
    image::Image,
    material::{Dielectric, Lambertian, Material, Metal},
    object::{Hit, HittableList, Sphere},
};

/*
Should have:
- a global material register
- an adaptable representation of a scene
- a way to render a scene with parameters
*/

/*
let scene = Scene::new();
let diffuse = scene.diffuse_material(Color::new(1.0, 0.0, 0.0));
let sphere = scene.sphere(Point::new(0.0, 0.0, -1.0), 0.5, diffuse);

*/

const HIT_TOLERANCE: f64 = 0.001;

pub struct Scene {
    objects: HittableList,
    camera: Camera,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            objects: HittableList::new(),
            camera: Camera::default(),
        }
    }

    pub fn sphere(&mut self, center: Point<f64>, radius: f64, material: &Arc<dyn Material>) {
        self.objects
            .add(Box::new(Sphere::new(center, radius, material.clone())));
    }

    pub fn diffuse_material(&mut self, color: Color) -> Arc<dyn Material> {
        let material = Lambertian::new(color);
        let material_arc = Arc::new(material);
        material_arc
    }

    pub fn metal_material(&mut self, color: Color, fuzz: f64) -> Arc<dyn Material> {
        let material = Metal::new(color, fuzz);
        let material_arc = Arc::new(material);
        material_arc
    }

    pub fn dielectric_material(&mut self, ref_idx: f64) -> Arc<dyn Material> {
        let material = Dielectric::new(ref_idx);
        let material_arc = Arc::new(material);
        material_arc
    }

    pub fn camera(
        &mut self,
        look_from: Point<f64>,
        look_at: Point<f64>,
        up: Vec3<f64>,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) {
        self.camera = Camera::new(
            look_from,
            look_at,
            up,
            vfov,
            aspect_ratio,
            aperture,
            focus_dist,
        );
    }

    pub fn render(&self, image: &mut Image, samples_per_pixel: u32, max_depth: u32) {
        let width = image.width();
        let height = image.height();

        image.apply_parallel(|x, y, pixel_color| {
            let mut color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (x as f64 + rand::random::<f64>()) / (width - 1) as f64;
                let v = (y as f64 + rand::random::<f64>()) / (height - 1) as f64;

                let r = self.camera.ray_at(u, v);

                color += self.ray_color(r, max_depth);
            }

            let scale = 1.0 / (samples_per_pixel as f64);

            *pixel_color = Color::new(
                (color.r() * scale).sqrt(),
                (color.g() * scale).sqrt(),
                (color.b() * scale).sqrt(),
            );
        });
    }

    fn ray_color(&self, r: Ray, depth: u32) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        if let Some(hit) = self.objects.hit(r, HIT_TOLERANCE..f64::INFINITY) {
            if let Some((scattered, attenuation)) = hit.material.scatter(r, &hit) {
                return attenuation * self.ray_color(scattered, depth - 1);
            }

            return Color::new(0.0, 0.0, 0.0);
        }

        let unit = r.direction().unit();
        let t = 0.5 * (unit.y() + 1.0);

        Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
    }
}
