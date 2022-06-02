use std::{ops::Range, path::PathBuf, sync::Arc};

use indicatif::{ProgressBar, ProgressStyle};

use crate::{
    camera::Camera,
    color::Color,
    geometry::{Point, Ray, Vec3},
    image::Image,
    material::{Dielectric, Lambertian, Material, Metal},
    object::{
        bvh::{BvhTree, Primitive},
        InfinitePlane, Object, Sphere, Tri, Visible, VisibleHit, VisibleList,
    },
    sky::{Sky, Uniform},
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

const HIT_TOLERANCE: f64 = 0.0001;

pub struct Scene {
    primitives: Vec<Arc<dyn Primitive>>,
    objects: VisibleList,
    camera: Camera,
    sky: Box<dyn Sky>,
    show_progress: bool,
}

type PrimArc = Arc<dyn Primitive>;
impl Scene {
    pub fn new() -> Self {
        Self {
            objects: VisibleList::new(),
            primitives: Vec::new(),
            camera: Camera::default(),
            sky: Box::new(Uniform::new(Color::white())),
            show_progress: true,
        }
    }

    // pub fn add(&mut self, object: impl Visible + 'static) {
    //     self.objects.add(Box::new(object));
    // }

    pub fn object(&mut self, path: impl Into<PathBuf>, material: &Arc<dyn Material>) {
        let obj = Object::new(path, Arc::clone(material)).expect("Unable to open object file");
        let mut prims = obj.to_primitives();

        dbg!(self.primitives.len());
        self.primitives.append(&mut prims);
        dbg!(self.primitives.len());
    }

    pub fn sphere(&mut self, center: Point<f64>, radius: f64, material: &Arc<dyn Material>) {
        let sphere: PrimArc = Arc::new(Sphere::new(center, radius, Arc::clone(material)));
        self.primitives.push(sphere);
    }

    pub fn plane(&mut self, origin: Point<f64>, normal: Vec3<f64>, material: &Arc<dyn Material>) {
        self.objects.add(Box::new(InfinitePlane::new(
            origin,
            normal,
            Arc::clone(material),
        )))
    }

    pub fn triangle(
        &mut self,
        a: Point<f64>,
        b: Point<f64>,
        c: Point<f64>,
        material: &Arc<dyn Material>,
    ) {
        let tri: PrimArc = Arc::new(Tri::new(a, b, c, Arc::clone(material)));
        self.primitives.push(tri);
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

    pub fn sky(&mut self, sky: impl Sky + 'static) {
        self.sky = Box::new(sky);
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

    pub fn progress(&mut self, show: bool) {
        self.show_progress = show;
    }

    pub fn render(&self, image: &mut Image, samples_per_pixel: u32, max_depth: u32) {
        // explicitly cloning the Arc references to the primitives
        let bvh = BvhTree::build(self.primitives.iter().map(|arc| Arc::clone(arc)).collect());

        let width = image.width();
        let height = image.height();

        let pb = if self.show_progress {
            let pb = ProgressBar::new((width * height) as u64);
            pb.set_style(ProgressStyle::default_bar().template(
                "[{elapsed_precise}] {wide_bar} ({percent}%) [{pos}px / {len}px ({per_sec})]",
            ));
            pb.set_draw_delta((width * height / 100) as u64);

            Some(pb)
        } else {
            None
        };

        image.apply_parallel(|x, y, pixel_color| {
            let mut color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (x as f64 + rand::random::<f64>()) / (width - 1) as f64;
                let v = (y as f64 + rand::random::<f64>()) / (height - 1) as f64;

                let r = self.camera.ray_at(u, v);

                color += self.ray_color(r, max_depth, &bvh);
            }

            let scale = 1.0 / (samples_per_pixel as f64);

            *pixel_color = Color::new(
                (color.r() * scale).sqrt(),
                (color.g() * scale).sqrt(),
                (color.b() * scale).sqrt(),
            );

            if let Some(pb) = &pb {
                pb.inc(1);
            }
        });
    }

    fn find_hit(&self, r: Ray, t_range: &Range<f64>, bvh: &BvhTree) -> Option<VisibleHit> {
        let closest_bvh = bvh.bounce(r, t_range);
        let closest_object = self.objects.bounce(r, t_range);

        // using comparison of options (shown in test below) to take the closest, non-None hit
        if closest_object.as_ref().and_then(|hit| Some(-hit.t))
            > closest_bvh.as_ref().and_then(|hit| Some(-hit.t))
        {
            closest_object
        } else {
            closest_bvh
        }
    }

    fn ray_color(&self, r: Ray, depth: u32, bvh: &BvhTree) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        if let Some(hit) = self.find_hit(r, &(HIT_TOLERANCE..f64::INFINITY), &bvh) {
            if let Some((scattered, attenuation)) = hit.material.scatter(r, &hit) {
                return attenuation * self.ray_color(scattered, depth - 1, bvh);
            }

            return Color::new(0.0, 0.0, 0.0);
        }

        let unit = r.direction().unit();
        self.sky.at(unit)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn visible_hit_comparison() {
        assert!(Some(5) > None);
        assert!(Some(-5) > None);
        assert!(None < Some(32));
        assert!(None::<f64> <= None);
        assert!(Some(23.453) > Some(-123.45));
    }
}
