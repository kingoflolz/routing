use nalgebra::VectorN;
use nalgebra::U10;

use rand::distributions::{IndependentSample, Range};
use rand::thread_rng;

#[derive(Clone, Debug)]
pub struct NCNodeData {
    pub outgoing_vec: VectorN<f32, U10>,
    pub incoming_vec: VectorN<f32, U10>,
    pub learn_rate: f32,
}

impl NCNodeData {
    pub fn new() -> NCNodeData {
        let mut rng = thread_rng();
        let between = Range::new(0., 1.);
        NCNodeData {
            outgoing_vec: <VectorN<f32, U10>>::from_fn(|_, _| between.ind_sample(&mut rng)),
            incoming_vec: <VectorN<f32, U10>>::from_fn(|_, _| between.ind_sample(&mut rng)),
            learn_rate: 0.05
        }
    }
}

pub fn calc_update(mut a: VectorN<f32, U10>, mut b: VectorN<f32, U10>, actual: f32, learn_rate: f32) -> (VectorN<f32, U10>, VectorN<f32, U10>) {
    let diff = actual - a.dot(&b);

    let mut a_d = b.clone();

    let mut n = 0;

    let diff = (diff).min(2.).max(-2.);

    for i in a_d.iter_mut() {
        *i = *i * learn_rate * diff;
        n += 1;
    }

    let mut b_d = a.clone();

    n = 0;
    for i in b_d.iter_mut() {
        *i = *i * learn_rate * diff;
        n += 1;
    }

    (a_d, b_d)
}