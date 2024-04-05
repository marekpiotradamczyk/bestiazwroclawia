use std::fs::File;

use candle_core::{NdArray, Tensor};
use itertools::Itertools;



pub mod parser;

fn main() {
    let file = File::open("data_collector/dataset.csv").unwrap();
    let data = parser::parse_data(file);
    let tensor = Tensor::new(data, device)

    dbg!(data.take(3).collect_vec());
}
