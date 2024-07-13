// use ndarray::array;
mod brownian_motion;
use rand::prelude::*;
use rand_distr::StandardNormal;

struct BlackScholesModel {
    r: f64,
    sig: f64,
}
trait ItoProcess {
    fn mu(&self, t: f64, x: f64) -> f64;
    fn sigma(&self, t: f64, x: f64) -> f64;
    fn gaussian(&self) -> f64 {
        thread_rng().sample(StandardNormal)
    }
}

impl ItoProcess for BlackScholesModel {
    fn mu(&self, _: f64, x: f64) -> f64 {
        self.r * x
    }
    fn sigma(&self, _: f64, x: f64) -> f64 {
        self.sig * x
    }
}
#[derive(Debug)]
struct Point {
    t: f64,
    x: f64,
}

struct EulerScheme<T> {
    point: Point,
    dt: f64,
    model: T,
}
impl<T> EulerScheme<T>
where
    T: ItoProcess,
{
    fn new(initial: f64, dt: f64, model: T) -> EulerScheme<T> {
        let point = Point { t: 0.0, x: initial };
        Self { point, dt, model }
    }
}

impl<T> Iterator for EulerScheme<T>
where
    T: ItoProcess,
{
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        let (t, x) = (self.point.t, self.point.x);
        let to_return = Point { t, x };
        let dt = self.dt;
        let g = self.model.gaussian() * dt.sqrt();
        let result = x + self.model.mu(t, x) * dt + self.model.sigma(t, x) * g;
        self.point.t += dt;
        self.point.x = result;
        Some(to_return)
    }
}

pub fn main() {
    let val: f64 = thread_rng().sample(StandardNormal);
    let bs_model = BlackScholesModel { r: 0.5, sig: 0.25 };
    let scheme = EulerScheme::new(100.0, 0.01, bs_model);
    let results: Vec<Point> = scheme.take(10).collect();
    println!("{results:?}");
    println!("{val}");
}
