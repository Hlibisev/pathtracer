use ndarray::Array1;

pub struct Material {
    pub albedo_color: Array1<f64>,
    pub diffuse_color: Array1<f64>,
    pub specular_color: Array1<f64>,
    pub specular_exponent: f64,
    pub reflection: f64,
}
