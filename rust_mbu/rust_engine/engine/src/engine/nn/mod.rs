extern crate ndarray;

use ndarray::{Array1, Array2};
use std::fs;
pub mod readcsv;
use sdk::position::Position;

#[derive(Clone)]
pub struct DenseNetwork {
    weights: Vec<Array2<f32>>,
    biases: Vec<Array1<f32>>,
    layers: usize,
    acc: Vec<Array1<f32>>,
    last_pos: Option<Position>,

}

impl Default for DenseNetwork {
    fn default() -> Self {
        Self {
            weights: Vec::new(),
            biases: Vec::new(),
            layers: 0,
            acc: vec![Array1::zeros(0)],
            last_pos: None,
        }
    }
}

impl DenseNetwork {
    pub fn new(path: &str) -> Self {
        let mut w = Vec::new();
        let mut b = Vec::new();

        let mut paths: Vec<_> = fs::read_dir(path).unwrap().map(|r| r.unwrap()).collect();

        paths.sort_by_key(|dir| dir.path());

        let l = paths.len() / 2;

        for i in 0..paths.len() / 2 {
            b.push(readcsv::read_array1_from_csv(paths[2 * i].path().to_str()));
            w.push(readcsv::read_array2_from_csv(paths[2 * i + 1].path().to_str()));
        }
        
        let a = vec![Array1::zeros(w[0].shape()[0])];
        Self {
            weights: w,
            biases: b,
            layers: l,
            acc: a,
            last_pos: None,
        }
    }

    fn step_with_relu(&self, x: &Array2<f32>, w: &Array2<f32>, b: &Array1<f32>) -> Array2<f32> {
        let z = x.dot(&w.t()) + b;
        z.mapv(|v| v.max(0.0))
    }

    fn step_with_sigmoid(&self, x: &Array2<f32>, w: &Array2<f32>, b: &Array1<f32>) -> Array2<f32> {
        let z = x.dot(&w.t()) + b;
        z.mapv(|v| 1.0 / (1.0 + (-v).exp()))
    }

    pub fn forward(&mut self, x: &Position) -> Array1<f32> {
        self.last_pos = Some(x.clone());
        let mut a = x.to_nn_input();
        
        for i in 0..self.layers - 1 {
            a = self.step_with_relu(&a, &self.weights[i], &self.biases[i]);
        }
        let result = self.step_with_sigmoid(
            &a,
            &self.weights[self.layers - 1],
            &self.biases[self.layers - 1],
        );
        result.column(0).to_owned()
    }

    pub fn update(&mut self, move_idx: [Vec<usize>; 2]) -> f32 {

        let last_idx = self.acc.len()-1;

        for idx in 0..move_idx[0].len() {
            self.acc[last_idx] -= &self.weights[0].column(move_idx[0][idx]); 
        }
        for idx in 0..move_idx[1].len() {
            self.acc[last_idx] += &self.weights[0].column(move_idx[1][idx]);
        }

        1.0 / (1.0 + (-self.acc[last_idx][0]).exp())
    }

    pub fn init_acc(&mut self, x: &Position) {
        self.acc = vec![Array1::zeros(self.weights[0].shape()[0])];
        self.last_pos = Some(x.clone());
        let a = x.to_nn_input();
        self.acc[0] += &self.biases[0];

        for i in 0..a.shape()[1] {
            if a.column(i)[0] == 1.0 {
                self.acc[0] += &self.weights[0].column(i);
            }
        }
    }

    pub fn save_acc(&mut self) {
        self.acc.push(self.acc.last().clone().unwrap().to_owned());
    }

    pub fn forget_acc(&mut self) {
        self.acc.truncate(self.acc.len() - 1);
    }

}
