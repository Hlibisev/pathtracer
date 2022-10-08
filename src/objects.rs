use std::array;

use crate::{phong::Material, utils::normalize};
use ndarray::array;
use ndarray::{s, Array1, Array2};
use ndarray_linalg::Scalar;

pub struct Sphere {
    position: Array1<f64>,
    radius: f64,
    material: Material,
    is_light: bool,
}

impl Sphere {
    pub fn new(position: Array1<f64>, radius: f64, material: Material, is_light: bool) -> Sphere {
        return Sphere {
            position,
            radius,
            material,
            is_light,
        };
    }
}

pub trait Object {
    fn is_light(&self) -> bool;
    fn material(&self) -> &Material;
    fn get_normal(&self, point: &Array1<f64>) -> Array1<f64>;
    fn ray_intersect(&self, orig: &Array1<f64>, dir: &Array1<f64>) -> (bool, f64);
}

impl Object for Sphere {
    fn is_light(&self) -> bool {
        return self.is_light;
    }

    fn material(&self) -> &Material {
        return &self.material;
    }

    fn ray_intersect(&self, orig: &Array1<f64>, dir: &Array1<f64>) -> (bool, f64) {
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

    fn get_normal(&self, point: &Array1<f64>) -> Array1<f64> {
        let normal = point - &self.position;
        return normalize(normal);
    }
}

pub struct Plane {
    material: Material,
    bounds: Array2<f64>,
    is_light: bool,
    lin_coeff: Array1<f64>,
    constant: f64,
}

impl Plane {
    pub fn new(
        points: [Array1<f64>; 3],
        material: Material,
        bounds: Array2<f64>,
        is_light: bool,
    ) -> Plane {
        let (lin_coeff, constant) = Plane::get_coeffitient(&points);

        return Plane {
            material,
            bounds,
            is_light,
            lin_coeff,
            constant,
        };
    }

    fn get_coeffitient(points: &[Array1<f64>; 3]) -> (Array1<f64>, f64) {
        let (x1, y1, z1) = (points[0][0], points[0][1], points[0][2]);
        let (x2, y2, z2) = (points[1][0], points[1][1], points[1][2]);
        let (x3, y3, z3) = (points[2][0], points[2][1], points[2][2]);

        let a1 = x2 - x1;
        let b1 = y2 - y1;
        let c1 = z2 - z1;
        let a2 = x3 - x1;
        let b2 = y3 - y1;
        let c2 = z3 - z1;

        let a = b1 * c2 - b2 * c1;
        let b = a2 * c1 - a1 * c2;
        let c = a1 * b2 - b1 * a2;
        let d = -a * x1 - b * y1 - c * z1;

        let lin_coeff = array![a, b, c];
        let norm = lin_coeff.dot(&lin_coeff).sqrt();

        let constant = d / norm;
        let lin_coeff = lin_coeff / norm;

        return (lin_coeff, constant);
    }
}

impl Object for Plane {
    fn is_light(&self) -> bool {
        return self.is_light;
    }

    fn material(&self) -> &Material {
        return &self.material;
    }

    fn ray_intersect(&self, orig: &Array1<f64>, dir: &Array1<f64>) -> (bool, f64) {
        let coef_t = self.lin_coeff.dot(dir);
        let constant_t = -self.constant - self.lin_coeff.dot(orig);

        if coef_t.abs() < 1e-7 && constant_t.abs() < 1e-7 {
            return (true, 0.0);
        } else if coef_t.abs() > 1e-7 && constant_t.abs() < 1e-7 {
            return (false, f64::INFINITY);
        }

        let t = constant_t / coef_t;

        if t < 0.0 {
            return (false, f64::INFINITY);
        }

        let point = orig + t * dir;

        let low_bound = self.bounds.slice(s![.., 0]);
        let higher_low_bound = point.iter().zip(low_bound.iter()).all(|(x, y)| x > y);
        // println!("{}", low_bound);
        // println!("{}", point);
        // println!("{}", higher_low_bound);
        // println!("  ");

        let hight_bound = self.bounds.slice(s![.., 1]);
        let lower_hight_bound = point.iter().zip(hight_bound.iter()).all(|(x, y)| x < y);

        if higher_low_bound && lower_hight_bound {
            return (true, t);
        }

        return (false, f64::INFINITY);
    }

    fn get_normal(&self, point: &Array1<f64>) -> Array1<f64> {
        return self.lin_coeff.to_owned();
    }
}
