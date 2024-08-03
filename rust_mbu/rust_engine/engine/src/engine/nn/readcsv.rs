extern crate csv;
extern crate ndarray_csv;

use ndarray::{Array1, Array2};
use std::fs::File;

pub fn read_array2_from_csv(file_path: Option<&str>) -> Array2<f64> {
    let file = read_csv(file_path.unwrap());

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);
    let mut records = Vec::new();
    for result in rdr.records() {
        let record_result = result;

        let record = match record_result {
            Ok(record) => record,
            Err(_error) => panic!("Wrong record"),
        };

        let row: Vec<f64> = record.iter().map(|x| x.parse::<f64>().unwrap()).collect();
        records.push(row);
    }
    let rows = records.len();
    let cols = records[0].len();
    let flat_data: Vec<f64> = records.into_iter().flatten().collect();
    let array = Array2::from_shape_vec((rows, cols), flat_data);

    match array {
        Ok(array) => return array,
        Err(_error) => panic!("Problem"),
    }
}

pub fn read_array1_from_csv(file_path: Option<&str>) -> Array1<f64> {
    let file = read_csv(file_path.unwrap());

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);
    let mut records = Vec::new();
    for result in rdr.records() {
        let record_result = result;

        let record = match record_result {
            Ok(record) => record,
            Err(_error) => panic!("Wrong record"),
        };
        for field in record.iter() {
            records.push(field.parse::<f64>().unwrap());
        }
    }
    return Array1::from(records);
}

fn read_csv(file_path: &str) -> File {
    let file_result = File::open(file_path);

    match file_result {
        Ok(file) => return file,
        Err(_error) => panic!("File problem"),
    };
}
