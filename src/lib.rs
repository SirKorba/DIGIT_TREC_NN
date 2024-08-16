use std::fs;
use image::ImageReader;
use serde_json;



pub struct Network {
    n_neur:[i32;3],                                         // [784, 30, 10]
}

impl Network {

    pub fn new(self, n_neur:[i32;3]) -> Network {           // Create NeuroWide
        Network{n_neur}
    }

    pub fn feed_forward(self) {                             // Start NeuroWide

    }

    pub fn SGD() {                                          // Educate NeuroWide

    }
} 

fn sygmoid(x: f64) -> f64 {                                 // Activate func
    1.0/(1.0 + x.exp())
}

pub fn start_app() {

    match fs::create_dir("data") {
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

    parse_dataset();
}

fn parse_dataset() {

    let mut numbers_paths:Vec<String> = Vec::new();
    let mut num_images:Vec<(String, u32)> = Vec::new();
    let mut dataset:Vec<(Vec<u8>, u32)> = Vec::new();

    match fs::read_dir("dataset/training") {
        Err(e) => eprintln!("Error: {e}"),
        Ok(paths) => {
            for path in paths {
                let p = path.unwrap().path();
                numbers_paths.append(&mut vec!(String::from(p.to_str().unwrap())));
            }
        },
    }

    for path in numbers_paths {

        let n:u32 = path["dataset/training".len()+1..].parse().unwrap();

        match fs::read_dir(path) {
            Err(e) => eprintln!("Error: {e}"),
            Ok(paths) => {
                for ph in paths {
                    let p = ph.unwrap().path();
                    num_images.append(&mut vec!((String::from(p.to_str().unwrap()), n)));
                }
            }
        }
    }

    for (path, n) in num_images {
        
        let data = ImageReader::open(path).unwrap().decode().unwrap().grayscale().into_bytes();

        dataset.append(&mut vec!((data, n)));
    }

    fs::write("data/dataset/dataset.json", serde_json::to_string_pretty(&dataset).unwrap())
        .expect("Can't write to file");

    // let s = fs::read_to_string("data/dataset/dataset.json").unwrap();

    // let mut json_data:serde_json::Value = serde_json::from_str(&s)
    //     .expect("Can't parse json");

    // println!("{}", json_data.as_array().unwrap()[0]);

}