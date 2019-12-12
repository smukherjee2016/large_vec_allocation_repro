use crate::sampler::sobol::Sobol;
use crate::sampler::Sampler;

mod sampler;

fn main() {
    println!("Hello, world");
    let x : Sobol = Sobol::init();

}
