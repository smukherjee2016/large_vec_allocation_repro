pub mod sobol;

pub trait Sampler {
    fn init() -> Self;
    fn sample_a_point(&mut self, index: u64, dimension: u64, offset: f64) -> f64;
}
