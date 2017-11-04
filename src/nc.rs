use nalgebra::VectorN;
use nalgebra::U10;

use rand::distributions::{IndependentSample, Range};
use rand::thread_rng;

#[derive(Clone, Debug)]
pub struct NCNodeData {
    outgoing_vec: VectorN<f32, U10>,
    incoming_vec: VectorN<f32, U10>
}

impl NCNodeData {
    pub fn new() -> NCNodeData {
        let mut rng = thread_rng();
        let between = Range::new(0., 2.);
        NCNodeData {
            outgoing_vec: <VectorN<f32, U10>>::from_fn(|_, _| between.ind_sample(&mut rng)),
            incoming_vec: <VectorN<f32, U10>>::from_fn(|_, _| between.ind_sample(&mut rng)),
        }
    }
}