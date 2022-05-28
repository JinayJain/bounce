use rand::{thread_rng, Rng};

use crate::geometry::{Point, Ray, Vec3};

pub struct Camera {
    origin: Point<f64>,
    lower_left_corner: Point<f64>,
    vertical: Vec3<f64>,
    horizontal: Vec3<f64>,

    // used for DoF calculation
    lens_radius: f64,
    vertical_axis: Vec3<f64>,
    horizontal_axis: Vec3<f64>,
}

impl Camera {
    // TODO: Convert new to a builder pattern
    pub fn new(
        look_from: Point<f64>,
        look_at: Point<f64>,
        view_up: Vec3<f64>,
        vertical_fov_deg: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let viewport_height = 2.0 * (vertical_fov_deg.to_radians() / 2.0).tan();
        let viewport_width = viewport_height * aspect_ratio;

        let reverse_view_dir = Vec3::from(look_from - look_at).unit();
        let unit_horizontal = view_up.cross(reverse_view_dir).unit();
        let unit_vertical = reverse_view_dir.cross(unit_horizontal);

        // assert!((unit_vertical.len_sq() - 1.0).abs() < 1e-8);

        let origin = look_from;
        let horizontal = viewport_width * focus_dist * unit_horizontal;
        let vertical = viewport_height * focus_dist * unit_vertical;
        let lower_left_corner = origin
            - (horizontal / 2.0).into()
            - (vertical / 2.0).into()
            - (reverse_view_dir * focus_dist).into();

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            lens_radius: aperture / 2.0,
            vertical_axis: unit_vertical,
            horizontal_axis: unit_horizontal,
        }
    }

    pub fn ray_at(&self, u: f64, v: f64) -> Ray {
        let random_on_lens = Camera::random_in_unit_disk() * self.lens_radius;
        let offset =
            self.horizontal_axis * random_on_lens.x() + self.vertical_axis * random_on_lens.y();

        Ray::new(
            self.origin + offset.into(),
            self.horizontal * u + self.vertical * v + self.lower_left_corner.into()
                - self.origin.into()
                - offset.into(),
        )
    }

    fn random_in_unit_disk() -> Vec3<f64> {
        let mut rng = thread_rng();

        loop {
            let cand = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);

            if cand.len_sq() < 1.0 {
                return cand;
            }
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(
            Point::new(0.0, 0.0, 0.0),
            Point::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 1.0, 0.0),
            90.0,
            1.0,
            0.0,
            1.0,
        )
    }
}
