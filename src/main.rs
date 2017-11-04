extern crate petgraph;
extern crate spade;
extern crate rand;
extern crate nalgebra;
extern crate num_traits;
extern crate typenum;

mod network;
mod nc;

fn main() {
    let mut g = network::generate();

    println!("Nodes: {}", g.node_count());
    println!("Edges: {}", g.edge_count());
}
