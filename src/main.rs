extern crate petgraph;
extern crate spade;
extern crate rand;
extern crate nalgebra;
extern crate num_traits;
extern crate typenum;

mod network;
mod nc;

fn main() {
    // let mut g = network::load_king_nodes();
    // let k = network::load_king_measurements();
    // network::init_nc(&mut g, k);

    let mut g = network::generate_flat_graph();
    let landmarks = network::calc_measurements(&mut g);
    network::init_nc(&mut g, landmarks);


    println!("Nodes: {}", g.node_count());
    println!("Edges: {}", g.edge_count());
}
