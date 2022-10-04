use crate::objects::Sphere;
use crate::phong::{Light, Material};
use ndarray::array;

pub fn get_light_and_obj() -> (Vec<Sphere>, Vec<Light>) {
    let ruby: Material = Material {
        albedo_color: array![0.1745, 0.01175, 0.01175],
        diffuse_color: array![0.61424, 0.04136, 0.04136],
        specular_color: array![0.727811, 0.296648, 0.296648],
        specular_exponent: 120.,
        reflection: 0.6,
    };

    let gray: Material = Material {
        albedo_color: array![0.05375, 0.05, 0.06625],
        diffuse_color: array![0.18275, 0.17, 0.22525],
        specular_color: array![0.05375, 0.05, 0.06625],
        specular_exponent: 2.,
        reflection: 0.01,
    };

    let emerlad: Material = Material {
        albedo_color: array![0.0215, 0.1745, 0.0215],
        diffuse_color: array![0.07568, 0.61424, 0.07568],
        specular_color: array![0.2, 1., 0.2],
        specular_exponent: 10.,
        reflection: 0.2,
    };

    let mirror: Material = Material {
        albedo_color: array![0.3, 0.3, 0.3],
        diffuse_color: array![0.0, 0.0, 0.0],
        specular_color: array![1., 1., 1.],
        specular_exponent: 1500.,
        reflection: 1.,
    };

    let shpere: Sphere = Sphere {
        position: array![-4.5, 1., -19.],
        radius: 1.6,
        material: ruby,
    };

    let sphere2: Sphere = Sphere {
        position: array![-3., -1., -17.],
        radius: 1.6,
        material: gray,
    };

    let sphere3: Sphere = Sphere {
        position: array![5.6, 3., -21.0],
        radius: 4.,
        material: mirror,
    };

    let sphere4: Sphere = Sphere {
        position: array![-0., -0.5, -21.0],
        radius: 2.2,
        material: emerlad,
    };

    let spheres: Vec<Sphere> = vec![shpere, sphere2, sphere3, sphere4];

    let light: Light = Light {
        position: array![5., 10., 3.],
        intensity: 2.,
    };

    let light2: Light = Light {
        position: array![-5., -10., 2.],
        intensity: 2.,
    };

    let lights: Vec<Light> = vec![light, light2];

    return (spheres, lights);
}
