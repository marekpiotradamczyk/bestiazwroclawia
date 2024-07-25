extern crate ndarray;

use ndarray::{Array2, Array1};
use std::fs;
pub mod readcsv;
pub struct DenseNetwork {
    weights: Vec<Array2<f64>>,
    biases: Vec<Array1<f64>>,
    layers: usize,
}

impl Default for DenseNetwork {
    fn default() -> Self {
        Self {
            weights: Vec::new(),
            biases: Vec::new(),
            layers: 0
        }
    }
}

impl DenseNetwork {
    pub fn new(path: &str) -> Self {
        let mut w = Vec::new();
        let mut b =  Vec::new();

        let mut paths: Vec<_> = fs::read_dir(path).unwrap()
            .map(|r| r.unwrap())
            .collect();

        paths.sort_by_key(|dir| dir.path());

        let l = paths.len()/2;
        
        for i in 0..paths.len()/2 {
            b.push(readcsv::read_array1_from_csv(paths[2*i].path().to_str()));
            w.push(readcsv::read_array2_from_csv(paths[2*i+1].path().to_str()));
        }
        
        Self {
            weights: w,
            biases: b,
            layers: l
        }
    }

    fn step_with_relu(&self, x: &Array2<f64>, w: &Array2<f64>, b: &Array1<f64>) -> Array2<f64> {
        let z = x.dot(&w.t()) + b;
        z.mapv(|v| v.max(0.0))
    }

    fn step_with_sigmoid(&self, x: &Array2<f64>, w: &Array2<f64>, b: &Array1<f64>) -> Array2<f64> {
        let z = x.dot(&w.t()) + b;
        z.mapv(|v| 1.0 / (1.0 + (-v).exp()))
    }

    pub fn forward(&self, x: &Array2<f64>) -> Array1<f64> {
        let mut a = x.clone();

        for i in 0..self.layers-1 {
            a = self.step_with_relu(&a, &self.weights[i], &self.biases[i]);
        }
        let result = self.step_with_sigmoid(&a, &self.weights[self.layers-1], &self.biases[self.layers-1]);
        result.column(0).to_owned()
    }
}
