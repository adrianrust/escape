use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Map {
    pub x: usize,
    pub y: usize,
    pub data: Vec<Vec<usize>>,
}

pub fn read_data() -> Map {
    let mut x: usize = 7;
    let mut y: usize = 7;
    let mut lines: Vec<String> = Vec::new();
    let file = File::open("data.txt").unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        if index == 0 {
            let values: Vec<&str> = line.split(",").collect();
            x = values[0].parse().unwrap();
            y = values[1].parse().unwrap();
        } else {
            lines.push(line);
        }
    }

    return Map { x, y, data: create_data_vector(x, y, lines) };
}

fn create_data_vector(x: usize, y: usize, data: Vec<String>) -> Vec<Vec<usize>> {
    let mut data_vec: Vec<Vec<usize>> = Vec::with_capacity(x * y);

    for single_line in data {
        let numbers: Vec<&str> = single_line.split("").collect();
        let mut row: Vec<usize> = Vec::new();
        for single_number in numbers.iter() {
            if single_number.len() == 1 {
                row.push(single_number.parse().unwrap());
            }
        }
        data_vec.push(row);
    }

    return data_vec;
}