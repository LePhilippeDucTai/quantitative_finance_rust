// use ndarray::array;
mod brownian_motion;
use rand::prelude::*;
use rand_distr::StandardNormal;

#[derive(Debug)]
struct BlackScholesModel {
    s0: f32,
    r: f32,
    sigma: f32,
}

trait ItoProcess {
    fn mu(&self, t: f32, x: f32) -> f32;
    fn sigma(&self, t: f32, x: f32) -> f32;
    fn gaussian(&self) -> f32 {
        thread_rng().sample(StandardNormal)
    }
}

impl ItoProcess for BlackScholesModel {
    fn mu(&self, _: f32, x: f32) -> f32 {
        self.r * x
    }
    fn sigma(&self, _: f32, x: f32) -> f32 {
        self.sigma * x
    }
}

struct EulerScheme<T>
where
    T: ItoProcess,
{
    current_x: f32,
    current_t: f32,
    dt: f32,
    model: T,
}

impl<T> Iterator for EulerScheme<T>
where
    T: ItoProcess,
{
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        let (t, x) = (self.current_t, self.current_x);
        let dt = self.dt;
        let g = self.model.gaussian() * dt.sqrt();
        let result = x + self.model.mu(t, x) * dt + self.model.sigma(t, x) * g;
        self.current_t += dt;
        self.current_x = result;
        Some(result)
    }
}

pub fn main() {
    let val: f64 = thread_rng().sample(StandardNormal);
    let bs_params = BlackScholesModel {
        s0: 100.0,
        r: 0.5,
        sigma: 0.25,
    };
    println!("{val}");
    println!("{bs_params:?}")
}
