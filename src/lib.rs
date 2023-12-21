mod utils;

use core::f64;

use friedrich::{gaussian_process::GaussianProcess, kernel::Matern1, prior::ConstantPrior};
use js_sys::Float64Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Sampler {
    gp: GaussianProcess<Matern1, ConstantPrior>,
    inputs: Vec<Vec<f64>>,
    output: Vec<f64>,
}

impl Sampler {
    fn grid() -> Vec<Vec<f64>> {
        let grid: Vec<Vec<f64>> = (0..100).map(|i| vec![(i as f64) / 20.0]).collect();
        grid
    }
}

#[wasm_bindgen]
impl Sampler {
    pub fn new(inputs: &[f64], output: &[f64]) -> Self {
        utils::set_panic_hook();

        let inputs = Vec::from(inputs)
            .into_iter()
            .map(|x| vec![x])
            .collect::<Vec<Vec<f64>>>();
        let output = Vec::from(output);
        // let gp = GaussianProcess::default(inputs.clone(), output.clone());
        let gp = GaussianProcess::new(
            ConstantPrior::new(0.0),
            Matern1::default(),
            0.1,
            None,
            inputs.clone(),
            output.clone(),
        );

        Sampler { gp, inputs, output }
    }

    pub fn get_grid(&self) -> Float64Array {
        Float64Array::from(&Sampler::grid().into_iter().flatten().collect::<Vec<f64>>()[..])
    }

    pub fn get_samples(&self) -> Float64Array {
        // samples from the distribution
        let new_inputs: Vec<Vec<f64>> = Sampler::grid();
        let sampler = self.gp.sample_at(&new_inputs);
        let mut rng = rand::thread_rng();
        let sample = sampler.sample(&mut rng);
        Float64Array::from(&sample[..])
    }

    pub fn mean(&self) -> Float64Array {
        let new_inputs: Vec<Vec<f64>> = Sampler::grid();
        let mean = self.gp.predict(&new_inputs);
        Float64Array::from(&mean[..])
    }

    pub fn variance(&self) -> Float64Array {
        let new_inputs: Vec<Vec<f64>> = Sampler::grid();
        let var = self.gp.predict_variance(&new_inputs);
        Float64Array::from(&var[..])
    }

    pub fn get_inputs(&self) -> Float64Array {
        Float64Array::from(
            &self
                .inputs
                .clone()
                .into_iter()
                .flatten()
                .collect::<Vec<f64>>()[..],
        )
    }

    pub fn get_outputs(&self) -> Float64Array {
        Float64Array::from(&self.output[..])
    }

    pub fn add_samples(&mut self, inputs: &[f64], output: &[f64]) {
        let mut inputs = Vec::from(inputs)
            .into_iter()
            .map(|x| vec![x])
            .collect::<Vec<Vec<f64>>>();
        let mut output = Vec::from(output);
        self.gp.add_samples(&inputs, &output);
        self.inputs.append(&mut inputs);
        self.output.append(&mut output);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let sampler = Sampler::new(&vec![0.7, 1.2, 3.8, 4.2], &vec![3.0, 3.0, -3.0, -2.0]);
        let new_inputs = Sampler::grid();
        let sampler = sampler.gp.sample_at(&new_inputs);
        let mut rng = rand::thread_rng();
        let _sample = sampler.sample(&mut rng);
    }
}
