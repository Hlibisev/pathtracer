use crate::objects::Sphere;
use crate::phong::Material;
use ndarray::array;

pub fn get_light_and_obj() -> Vec<Sphere> {
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

    let mirror = Material {
        albedo_color: array![0.3, 0.3, 0.3],
        diffuse_color: array![0.0, 0.0, 0.0],
        specular_color: array![1., 1., 1.],
        specular_exponent: 1500.,
        reflection: 1.,
    };

    let shpere = Sphere {
        position: array![-4.5, 1., -19.],
        radius: 1.6,
        material: ruby,
        is_light: false,
    };

    let sphere2 = Sphere {
        position: array![-3., -1., -17.],
        radius: 1.6,
        material: gray,
        is_light: false,
    };

    let sphere3 = Sphere {
        position: array![5.6, 3., -21.0],
        radius: 4.,
        material: mirror,
        is_light: false,
    };

    let sphere4 = Sphere {
        position: array![1., -4., -15.0],
        radius: 4.,
        material: emerlad,
        is_light: true,
    };

    let sphere5 = Sphere {
        position: array![-6.5, -1.5, -17.],
        radius: 1.6,
        material: emerlad2,
        is_light: true,
    };

    let spheres = vec![shpere, sphere2, sphere3, sphere4, sphere5];

    return spheres;
}
