// Functions to simulate brownian trajectories

#[derive(Debug)]
pub struct BrownianPath {
    t: Vec<f64>,
    x: Vec<f64>,
}

impl BrownianPath {
    pub fn new(t: Vec<f64>, x: Vec<f64>) -> BrownianPath {
        BrownianPath { t, x }
    }
}

pub fn main() {
    let t = vec![1., 2.];
    let x = vec![3., 1.];
    let _path = BrownianPath::new(t, x);
    println!("Hello World");
    println!("{_path:?}")
}

// fn brownian_bridge(T: f64, a: f64, b: f64) -> {}
