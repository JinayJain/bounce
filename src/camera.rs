use crate::geometry::{Point, Ray, Vec3};

pub struct Camera {
    origin: Point<f64>,
    lower_left_corner: Point<f64>,
    vertical: Vec3<f64>,
    horizontal: Vec3<f64>,
}

impl Camera {
    pub fn new(viewport_height: f64, viewport_width: f64, focal_length: f64) -> Self {
        let origin = Point::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner = origin
            - (horizontal / 2.0).into()
            - (vertical / 2.0).into()
            - Vec3::new(0.0, 0.0, focal_length).into();

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn ray_at(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.horizontal * u + self.vertical * v + self.lower_left_corner.into(),
        )
    }
}

impl Default for Camera {
    fn default() -> Self {
        // Camera parameters
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        Camera::new(viewport_height, viewport_width, focal_length)
    }
}
