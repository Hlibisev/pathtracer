use crate::objects::{Object, Plane, Sphere};
use crate::phong::Material;
use ndarray::array;

pub fn get_light_and_obj() -> Vec<Box<dyn Object>> {
    let ruby = Material {
        albedo_color: array![0.1745, 0.01175, 0.01175],
        diffuse_color: array![0.61424, 0.04136, 0.04136],
        specular_color: array![0.727811, 0.296648, 0.296648],
        specular_exponent: 120.,
        reflection: 0.6,
    };

    let gray = Material {
        albedo_color: array![0.05375, 0.05, 0.06625],
        diffuse_color: array![0.18275, 0.17, 0.22525],
        specular_color: array![0.05375, 0.05, 0.06625],
        specular_exponent: 2.,
        reflection: 0.01,
    };

    let emerlad = Material {
        albedo_color: array![1.0, 1.0, 1.0],
        diffuse_color: array![1.0, 1.0, 1.0],
        specular_color: array![1.0, 1.0, 1.0],
        specular_exponent: 10.,
        reflection: 0.2,
    };

    let emerlad2 = Material {
        albedo_color: array![1.0, 1.0, 1.0],
        diffuse_color: array![1.0, 1.0, 1.0],
        specular_color: array![1.0, 1.0, 1.0],
        specular_exponent: 10.,
        reflection: 0.2,
    };

    let emerlad3 = Material {
        albedo_color: array![1.0, 1.0, 1.0],
        diffuse_color: array![1.0, 1.0, 1.0],
        specular_color: array![1.0, 1.0, 1.0],
        specular_exponent: 10.,
        reflection: 0.2,
    };

    let mirror = Material {
        albedo_color: array![0.3, 0.3, 0.3],
        diffuse_color: array![0.0, 0.0, 0.0],
        specular_color: array![1., 1., 1.],
        specular_exponent: 1500.,
        reflection: 1.,
    };

    let sphere = Sphere::new(array![-4.5, 1., -19.], 1.6, ruby, false);
    let sphere2 = Sphere::new(array![-3., -1., -17.], 1.6, gray, false);
    let sphere3 = Sphere::new(array![5.6, 3., -21.0], 4., mirror, false);
    // let sphere4 = Sphere::new(array![1., -4., -15.0], 4., emerlad, true);
    let sphere5 = Sphere::new(array![-6.5, -1.5, -17.], 1.6, emerlad2, true);
    
    let plane = Plane::new(
        [
            array![0.0, -3.5, 0.0],
            array![1.0, -3.5, -1.0],
            array![0.0, -3.5, -1.0],
        ],
        emerlad3,
        // array![[-9.2, 9.2], [-5.0, 5.0], [-20.0, -7.0]],
        array![[-f64::INFINITY, f64::INFINITY], [-f64::INFINITY, f64::INFINITY], [-f64::INFINITY, f64::INFINITY]],
        false,
    );
    let objects: Vec<Box<dyn Object>> = vec![
        Box::new(sphere),
        Box::new(sphere2),
        Box::new(sphere3),
        // Box::new(sphere4),
        Box::new(sphere5),
        Box::new(plane),
    ];

    return objects;
}
