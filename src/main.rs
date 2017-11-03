extern crate petgraph;
extern crate spade;
extern crate rand;
extern crate cgmath;

mod network;

fn main() {
    let g = network::generate_graph();

    println!("Nodes: {}", g.node_count());
    println!("Edges: {}", g.edge_count());
}
