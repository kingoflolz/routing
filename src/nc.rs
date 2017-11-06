use nalgebra::VectorN;
use nalgebra::U10;

use rand::distributions::{IndependentSample, Range};
use rand::thread_rng;

#[derive(Clone, Debug)]
pub struct NCNodeData {
    pub outgoing_vec: VectorN<f32, U10>,
    pub incoming_vec: VectorN<f32, U10>
}

impl NCNodeData {
    pub fn new() -> NCNodeData {
        let mut rng = thread_rng();
        let between = Range::new(0., 1.);
        NCNodeData {
            outgoing_vec: <VectorN<f32, U10>>::from_fn(|_, _| between.ind_sample(&mut rng)),
            incoming_vec: <VectorN<f32, U10>>::from_fn(|_, _| between.ind_sample(&mut rng)),
        }
    }
}

pub fn calc_update(mut a: VectorN<f32, U10>, mut b: VectorN<f32, U10>, actual: f32) -> (VectorN<f32, U10>, VectorN<f32, U10>) {
    let diff = actual - a.dot(&b);

    let mut a_d = b.clone();

    let mut n = 0;

    for i in a_d.iter_mut() {
        *i *= 0.005 * (diff).min(10.).max(-10.) + 0.001; // - a[n]*a[n] * 0.0001;
        n += 1;
    }

    let mut b_d = a.clone();

    n = 0;
    for i in b_d.iter_mut() {
        *i *= 0.005 * (diff).min(10.).max(-10.) + 0.001; // - b[n]*b[n] * 0.0001;
        n += 1;
    }

    // if((actual - (a + a_d).dot(&(b + b_d))).abs() > (actual - a.dot(&b)).abs()){
    //     println!("!!! {}, {}, {}, {}, {}, {}, {}, {}", a, b, actual, a.dot(&b), diff, a_d, b_d, ((a + a_d).dot(&(b + b_d))));
    //     panic!("RIP");
    // }

    (a_d, b_d)
}