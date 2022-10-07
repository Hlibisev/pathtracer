use crate::{phong::Material, utils::normalize};
use ndarray::Array1;

pub struct Sphere {
    pub position: Array1<f64>,
    pub radius: f64,
    pub material: Material,
    pub is_light: bool,
}

impl Sphere {
    pub fn ray_intersect(&self, orig: &Array1<f64>, dir: &Array1<f64>) -> (bool, f64) {
        let l = &self.position - orig;

        let a: f64 = l.dot(dir);
        let c2: f64 = l.dot(&l);
        let h2: f64 = c2 - a.powi(2);

        if h2 > self.radius.powi(2) {
            return (false, f64::INFINITY);
        }

        let half_a_inside_sphere = (self.radius.powi(2) - h2).sqrt();

        if a - half_a_inside_sphere > 0.0 {
            return (true, a - half_a_inside_sphere);
        }

        if a + half_a_inside_sphere > 0.0 {
            return (true, a + half_a_inside_sphere);
        }

        return (false, f64::INFINITY);
    }
}

impl Sphere {
    pub fn get_normal(&self, point: &Array1<f64>) -> Array1<f64> {
        let normal = point - &self.position;
        return normalize(normal);
    }
}
