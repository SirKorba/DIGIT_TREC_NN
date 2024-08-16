use std::fs;



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
}