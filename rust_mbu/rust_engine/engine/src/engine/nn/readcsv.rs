extern crate ndarray_csv;
extern crate csv;

use ndarray::{Array1, Array2};
use std::fs::File;
use std::error::Error;

use csv::ReaderBuilder;
const CHOICES_NUMBER: usize = 101;


pub fn read_array2_from_csv(file_path: Option<&str>) -> Array2<f32> {
    let file = read_csv(file_path.unwrap());

    let mut rdr = csv::ReaderBuilder::new().has_headers(false).from_reader(file);
    let mut records = Vec::new();
    for result in rdr.records() {
        let record_result = result;

        let record = match record_result {
            Ok(record) => record,
            Err(_error) => panic!("Wrong record"),
        };

        let row: Vec<f32> = record.iter().map(|x| x.parse::<f32>().unwrap()).collect();
        records.push(row);
    }
    let rows = records.len();
    let cols = records[0].len();
    let flat_data: Vec<f32> = records.into_iter().flatten().collect();
    let array = Array2::from_shape_vec((rows, cols), flat_data);

    match array {
        Ok(array) => return array,
        Err(_error) => panic!("Problem")
    }
}


pub fn read_array1_from_csv(file_path: Option<&str>) -> Array1<f32> {
    let file = read_csv(file_path.unwrap());

    let mut rdr = csv::ReaderBuilder::new().has_headers(false).from_reader(file);
    let mut records = Vec::new();
    for result in rdr.records() {
        let record_result = result;

        let record = match record_result {
            Ok(record) => record,
            Err(_error) => panic!("Wrong record"),
        };
        for field in record.iter() {
            records.push(field.parse::<f32>().unwrap());
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

pub fn load_array_from_csv(filename: &str) -> Result<[i32; CHOICES_NUMBER], Box<dyn Error>> {
    // Open the CSV file
    let mut rdr = ReaderBuilder::new().has_headers(false).from_path(filename)?;

    // Initialize the static array with zeros
    let mut array = [0; CHOICES_NUMBER];

    // Populate the array with data from the CSV file
    for (i, result) in rdr.records().enumerate() {
        let record = result?;
        if i < CHOICES_NUMBER {
            // Parsing the first field of the record as i32
            array[i] = record[0].parse::<i32>()?;
        } else {
            break;
        }
    }

    Ok(array)
}