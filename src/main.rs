mod data;
mod objects;
mod phong;
mod utils;

use data::*;
use ndarray::{array, s, Array1, Array3, ArrayBase, Dim, OwnedRepr};
use objects::Sphere;
use phong::Light;
use utils::{array_to_image, normalize, reflect, shlick, geometric, distribution};

use std::f64::consts::PI;
use std::ops::{Add, Mul};

fn simple_raycast(
    orig: &Array1<f64>,
    dir: &Array1<f64>,
    objects: &Vec<Sphere>,
    lights: &Vec<Light>,
    depth: u8,
) -> Array1<f64> {
    if depth == 5 {
        return array![0.0, 0.0, 0.0];
    }

    let nearest_object_index_dist = objects
        .iter()
        .enumerate()
        .map(|(i, sphere)| (i, sphere.ray_intersect(&orig, &dir)))
        .max_by(|first, second| second.1 .1.total_cmp(&first.1 .1));

    let (nearest_index, (is_intersected, dist)) = match nearest_object_index_dist {
        Some(value) => value,
        None => return array![0.05, 0.05, 0.05], // if empty vec
    };

    let nearest_object = &objects[nearest_index];

    let mut intensity = 0.;
    let mut specular_intensity = 0.;

    let point = orig + dir * dist;
    let norm = nearest_object.get_normal(&point);

    if is_intersected {
        let mut color: Array1<f64> = array![0.0, 0.0, 0.0];

        for light in lights {
            let light_dir = normalize(&light.position - &point);

            // TODO: named break
            // let mut i = 0;
            // for (index, object) in objects.iter().enumerate() {
            //     if (index != nearest_index) && (object.ray_intersect(&point, &light_dir).1 < 200.0)
            //     {
            //         i = 1;
            //         break;
            //     }
            // }
            // if i > 0 {
            //     break;
            // }

            let inv_dir = -dir;
            let half = normalize(&inv_dir + &light_dir);

            let BDRF_div = 4.0 * inv_dir.dot(&norm) * light_dir.dot(&norm);
            //distribution(&norm, &half, nearest_object.material.specular_exponent)
            
            let BDRF = distribution(&norm, &half, nearest_object.material.specular_exponent)
                .mul(shlick(&nearest_object.material.albedo_color, &half, &inv_dir))
                .mul(shlick(&nearest_object.material.albedo_color, &half, &light_dir))
                .mul(geometric(&norm, &inv_dir,nearest_object.material.raphess))
                .mul(1.0 / BDRF_div)
                .add(&nearest_object.material.albedo_color);

            color = color + BDRF * light_dir.dot(&norm) * light.intensity;
        }

        let reflect_dir = reflect(&norm, dir);
        let reflect_orig: ArrayBase<OwnedRepr<f64>, Dim<[usize; 1]>>;

        if reflect_dir.dot(&norm) < 0.0 {
            reflect_orig = point - &norm * 1e-3;
        } else {
            reflect_orig = point + &norm * 1e-3;
        }

        color = simple_raycast(&reflect_orig, &reflect_dir, objects, lights, depth + 1_u8)
            .mul(nearest_object.material.reflection)
            .add(color);

        return color;
    }

    if depth > 0_u8 {
        return array![0.0, 0.0, 0.0];
    }

    return array![0.075, 0.7, 0.7];
}

fn main() {
    let (objects, lights) = get_light_and_obj();

    let fov = PI / 3.4;
    let n = 2000;
    let mut image = Array3::<f64>::zeros((n, n, 3));

    let widght_screen = (fov / 2.0).tan() * 2.0;
    let height_screen = (fov / 2.0).tan() * 2.0;

    let shape_1 = image.shape()[0] as f64;
    let shape_2 = image.shape()[1] as f64;

    let orig = Array1::<f64>::zeros(3);

    for i in 0..image.shape()[0] {
        for j in 0..image.shape()[1] {
            let i = i as f64;
            let j = j as f64;

            let x = (j / shape_1 - 0.5) * widght_screen;
            let y = -(i / shape_2 - 0.5) * height_screen;

            let dir = normalize(array![x, y, -1.]);

            let mut slice = image.slice_mut(s![i as usize, j as usize, ..]);
            slice += &(simple_raycast(&orig, &dir, &objects, &lights, 0_u8) * 255.);
        }
    }

    let image: Array3<u8> = image.mapv(|x| x as u8);
    let image = array_to_image(image);

    image.save("out.png").expect("File wasn't save");
}
