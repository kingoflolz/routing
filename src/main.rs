extern crate petgraph;
extern crate spade;
extern crate rand;
extern crate nalgebra;
extern crate num_traits;
extern crate typenum;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

mod network;
mod nc;

use std::fs::File;
use std::io::prelude::*;

fn main() {
    // let mut g = network::load_king_nodes();
    // let k = network::load_king_measurements();
    // network::init_nc(&mut g, k);

    // generate new network and save
    let mut g = network::generate::generate_hier_graph();
    let landmarks = network::generate::calc_measurements(&mut g);
    network::nc::init_nc(&mut g, landmarks);
    let out = serde_json::to_string(&g).unwrap();
    let mut file = File::create("network.json").unwrap();
    file.write_all(out.as_bytes()).unwrap();

    // load network
    let file = File::open("network.json").expect("file not found");
    let g: network::Network = serde_json::from_reader(file).unwrap();

    println!("Nodes: {}", g.node_count());
    println!("Edges: {}", g.edge_count());
}
