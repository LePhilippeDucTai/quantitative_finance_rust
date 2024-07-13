use qlib::brownian_motion::brownian;

fn main() {
    let t = vec![1., 2.];
    let x = vec![3., 1.];
    let _path = brownian::BrownianPath::new(t, x);
    println!("Hello World");
    println!("{_path:?}")
}
