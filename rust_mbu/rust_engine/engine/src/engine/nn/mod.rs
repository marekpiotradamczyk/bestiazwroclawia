extern crate ndarray;

use ndarray::{Array2, Array1};
pub mod readcsv;
pub struct DenseNetwork {
    w1: Array2<f64>,
    b1: Array1<f64>,
    w2: Array2<f64>,
    b2: Array1<f64>,
}

impl Default for DenseNetwork {
    fn default() -> Self {
        Self {
            w1: readcsv::read_array2_from_csv("layer1_weights.csv"),
            b1: readcsv::read_array1_from_csv("layer1_biases.csv"),
            w2: readcsv::read_array2_from_csv("layer2_weights.csv"),
            b2: readcsv::read_array1_from_csv("layer2_biases.csv"),
        }
    }
}

impl DenseNetwork {
    pub fn new(weight1: Array2<f64>, biases1: Array1<f64>, weight2: Array2<f64>, biases2: Array1<f64>) -> Self {
        Self {
            w1: weight1,
            b1: biases1,
            w2: weight2,
            b2: biases2,
        }
    }

    pub fn forward(&self, x: &Array2<f64>) -> Array1<f64> {
        let z1 = x.dot(&self.w1.t()) + &self.b1;
        let a1 = z1.mapv(|v| v.max(0.0));
        let z2 = a1.dot(&self.w2.t()) + &self.b2;
        let a2 = z2.mapv(|v| 1.0 / (1.0 + (-v).exp()));
        a2.column(0).to_owned()
    }
}

