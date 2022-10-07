mod data;
mod objects;
mod phong;
mod utils;

use data::*;
use ndarray::{array, s, Array, Array1, Array3, ArrayBase, Dim, OwnedRepr};
use objects::Sphere;

use ndarray_rand::rand;
use ndarray_rand::rand_distr::{Distribution, Uniform, UnitSphere};
use ndarray_rand::RandomExt;
use utils::{array_to_image, normalize, reflect};

use ndarray_rand::rand::Rng;
use std::f64::consts::PI;
use std::ops::{Add, Mul};

fn simple_raycast(
    orig: &Array1<f64>,
    dir: &Array1<f64>,
    objects: &Vec<Sphere>,
    depth: u8,
) -> Array1<f64> {
    if depth == 5 {
        return array![0.1, 0.1, 0.1];
    }

    let nearest_object_index_dist = objects
        .iter()
        .enumerate()
        .map(|(i, sphere)| (i, sphere.ray_intersect(&orig, &dir)))
        .max_by(|first, second| second.1 .1.total_cmp(&first.1 .1));

    let (nearest_index, (is_intersected, dist)) = match nearest_object_index_dist {
        Some(value) => value,
        None => return array![0.1, 0.1, 0.1], // if empty vec
    };

    let nearest_object = &objects[nearest_index];

    if is_intersected {
        if nearest_object.is_light {
            return nearest_object.material.albedo_color.to_owned();
        }

        let point = orig + dir * dist;
        let norm = nearest_object.get_normal(&point);

        let unit_sphere_vector: [f64; 3] = UnitSphere.sample(&mut rand::thread_rng());
        let unit_sphere_vector = Array::from_vec(unit_sphere_vector.to_vec());

        let half_unit_sphere_vector;

        if norm.dot(&unit_sphere_vector) < 0.0 {
            half_unit_sphere_vector = -unit_sphere_vector
        } else {
            half_unit_sphere_vector = unit_sphere_vector
        }

        // let reflect_dir = half_unit_sphere_vector + &point;
        let reflect_orig = point + &norm * 1e-3;

        let light = simple_raycast(
            &reflect_orig,
            &half_unit_sphere_vector,
            objects,
            depth + 1_u8,
        );
        let color = (&nearest_object.material.diffuse_color + 2.0)
            * light
            * half_unit_sphere_vector.dot(&norm);
        // println!("{}", half_unit_sphere_vector.dot(&norm));
        return color;
    }

    if depth > 0_u8 {
        return array![0.1, 0.1, 0.1];
    }

    return array![0.1, 0.1, 0.1];
}

fn main() {
    let objects = get_light_and_obj();

    let fov = PI / 3.4;
    let n = 500;
    let mut image = Array3::<f64>::zeros((n, n, 3));

    let widght_screen = (fov / 2.0).tan() * 2.0;
    let height_screen = (fov / 2.0).tan() * 2.0;

    let shape_1 = image.shape()[0] as f64;
    let shape_2 = image.shape()[1] as f64;

    let orig = Array1::<f64>::zeros(3);
    let mut rng = rand::thread_rng();

    for i in 0..image.shape()[0] {
        for j in 0..image.shape()[1] {
            let i = i as f64;
            let j = j as f64;

            let x = (j / shape_1 - 0.5) * widght_screen;
            let y = -(i / shape_2 - 0.5) * height_screen;

            let mut slice = image.slice_mut(s![i as usize, j as usize, ..]);

            for _ in 1..64 {
                // println!("{}", rng.gen::<f64>());
                let x_shift = (rng.gen::<f64>() - 0.5) / shape_1 * widght_screen;
                let y_shift = (rng.gen::<f64>() - 0.5) / shape_2 * height_screen;
                let dir = normalize(array![x + x_shift, y + y_shift, -1.]);

                slice += &(simple_raycast(&orig, &dir, &objects, 0_u8) * 255.);
            }
            slice /= 64.0;
        }
    }

    let image: Array3<u8> = image.mapv(|x| x as u8);
    let image = array_to_image(image);

    image.save("out.png").expect("File wasn't save");
}
