use image::RgbImage;
use ndarray::{array, Array1, Array2, Array3, ArrayBase, Dim, OwnedRepr};

pub fn normalize(vector: Array1<f64>) -> Array1<f64> {
    let norm = vector.dot(&vector).sqrt();
    return vector / norm;
}

pub fn reflect(norm: &Array1<f64>, light: &Array1<f64>) -> Array1<f64> {
    return light - 2.0 * light.dot(norm) * norm;
}

pub fn array_to_image(arr: Array3<u8>) -> RgbImage {
    assert!(arr.is_standard_layout());

    let (height, width, _) = arr.dim();
    let raw = arr.into_raw_vec();

    RgbImage::from_raw(width as u32, height as u32, raw)
        .expect("container should have the right size for the image dimensions")
}

pub fn rotation_matrix(degrees: f64) -> Array2<f64> {
    return array![
        [degrees.cos(), -degrees.sin(), 0.],
        [degrees.sin(), degrees.cos(), 0.0]
    ];
}
