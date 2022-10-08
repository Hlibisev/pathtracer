use crate::objects::Sphere;
use crate::phong::{Light, Material};
use ndarray::array;

pub fn get_light_and_obj() -> (Vec<Sphere>, Vec<Light>) {
    let ruby: Material = Material {
        albedo_color: array![0.0, 0.8, 0.8],
        diffuse_color: array![0.18275, 0.17, 0.22525],
        specular_color: array![0.05375, 0.05, 0.06625],
        specular_exponent: 0.4,
        reflection: 0.,
        raphess: 0.2,
    };

    let gray: Material = Material {
        albedo_color: array![0.0, 0.8, 0.8],
        diffuse_color: array![0.18275, 0.17, 0.22525],
        specular_color: array![0.05375, 0.05, 0.06625],
        specular_exponent: 0.8,
        reflection: 0.,
        raphess: 0.5,
    };

    let emerlad: Material = Material {
        albedo_color: array![0.0, 0.8, 0.8],
        diffuse_color: array![0.18275, 0.17, 0.22525],
        specular_color: array![0.05375, 0.05, 0.06625],
        specular_exponent: 0.8,
        reflection: 0.,
        raphess: 0.1,
    };

    let mirror: Material = Material {
        albedo_color: array![0.0, 0.8, 0.8],
        diffuse_color: array![0.18275, 0.17, 0.22525],
        specular_color: array![0.05375, 0.05, 0.06625],
        specular_exponent: 0.8,
        reflection: 0.,
        raphess: 0.99,
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
        position: array![10., 10., 3.],
        intensity: 1.,
    };

    // let light2: Light = Light {
    //     position: array![-5., -10., 2.],
    //     intensity: 2.,
    // };

    let lights: Vec<Light> = vec![light];

    return (spheres, lights);
}


