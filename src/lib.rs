use std::fs;
use std::iter::zip;
use image::ImageReader;
use serde_json;
use serde_json::Value;
use rand::prelude::*;


pub struct Network {
    n_neur:Vec<u32>,                                                        // Example: [784, 30, 10]
    weights:Vec<Vec<Vec<f64>>>,
    disps:Vec<Vec<f64>>,
}

impl Network {

    pub fn new(n_neur:Vec<u32>) -> Network {                                // Create NeuroNetwork
        
        let mut weights:Vec<Vec<Vec<f64>>> = Vec::new();
        let mut disps:Vec<Vec<f64>> = Vec::new();
        let mut rng = rand::thread_rng();

        for (i, n) in n_neur.iter().enumerate() {
            
            if i == 0 {continue;}

            let mut _d_layer:Vec<f64> = Vec::new();
            let mut _w_layer:Vec<Vec<f64>> = Vec::new();

            disps.push(_d_layer.clone());                                   // Push layers of neuronetwork to disps and weights
            weights.push(_w_layer.clone());

            for j in 0..*n {
                disps[i-1].push(rng.gen());                                 // Push disp to disps
                weights[i-1].push(Vec::new());                              // Push edge massive to weights

                for _ in 0..n_neur[i-1] {
                    weights[i-1][j as usize].push(rng.gen());               //Push weight to edge
                }
            }
        }

        Network {                                                           // Return instance of neuro network
            n_neur,
            weights,
            disps,
        }
    }

    pub fn feed_forward(self, input_data:Vec<Value>) -> f64 {                                             // Start NeuroNetwork
        
        let output:f64 = 0.0;

        output
    }

    pub fn SGD(self) {                                                      // Educate NeuroNetwork

    }
} 

fn sigmoid(x: f64) -> f64 {                                                 // Activate func
    1.0/(1.0 + (-x).exp())
}

pub fn start_app() {

    match fs::create_dir("data") {                                   // Create directories
        Ok(_) => (),
        Err(e) => eprintln!("Error: {e}"),
    }

    match fs::create_dir("data/dataset") {
        Ok(_) => (),
        Err(e) => eprintln!("Error: {e}"),
    }

    match fs::create_dir("data/edResults") {
        Ok(_) => (),
        Err(e) => eprintln!("Error: {e}"),
    }

    parse_dataset("dataset/training", "data/dataset/dataset.json");
}

fn parse_dataset(input_dataset:&str, output_dataset:&str) {

    let mut numbers_paths:Vec<String> = Vec::new();
    let mut num_images:Vec<(String, u32)> = Vec::new();
    let mut dataset:Vec<(Vec<u8>, u32)> = Vec::new();

    match fs::read_dir(input_dataset) {
        Err(e) => eprintln!("Error: {e}"),
        Ok(paths) => {
            for path in paths {
                let p = path.unwrap().path();
                numbers_paths.append(&mut vec!(String::from(p.to_str().unwrap())));
            }
        },
    }

    for path in numbers_paths {

        let n:u32 = path[input_dataset.len()+1..].parse().unwrap();

        match fs::read_dir(path) {
            Err(e) => eprintln!("Error: {e}"),
            Ok(paths) => {
                for ph in paths {
                    let p = ph.unwrap().path();
                    num_images.push((String::from(p.to_str().unwrap()), n));
                }
            }
        }
    }

    for (path, n) in num_images {
        
        let data = ImageReader::open(path).unwrap().decode().unwrap().grayscale().into_bytes();

        dataset.push((data, n));
    }

    fs::write(output_dataset, serde_json::to_string_pretty(&dataset).unwrap())
        .expect("Can't write to file");
}

pub fn get_dataset(dataset_file:&str) -> Vec<Value> {

    let s = fs::read_to_string(dataset_file).unwrap();

    let json_data:serde_json::Value = serde_json::from_str(&s)
        .expect("Can't parse json");

    let result = json_data.as_array().unwrap().clone();

    result
}

// fn dot(a: &Vec<f64>, b: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {

// }

fn mul_matr(a: &Vec<f64>, b: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dot_test() {
        
        let a:Vec<f64> = vec![1.0, 2.0, 3.0];
        let b:Vec<Vec<f64>> = vec![vec![5.0, 4.0, 2.0], vec![1.0, 4.0, 6.0], vec![3.0, 8.0, 3.0]];
        println!("{:?}", dot(&a, &b));
        assert_eq!(dot(&a, &b), vec![vec![16.0, 36.0, 23.0]])

    }
}