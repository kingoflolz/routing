use nalgebra::{VectorN, MatrixN};
use nalgebra::{U10, U32};
use petgraph::Graph;
use petgraph::graph::NodeIndex;

use rand::distributions::{IndependentSample, Range};
use rand::{thread_rng, Rng};

use network::{Node, Connection, best_route};
use num_traits::identities::Zero;

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