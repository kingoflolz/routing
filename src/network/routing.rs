use petgraph::graph::NodeIndex;

use nc::NCNodeData;
use network::{Node, Network, best_route};

use rand::thread_rng;
use rand::distributions::{Sample, Range};

use std::collections::HashMap;

pub fn test_routing(n: &Network) {
    let mut rng = thread_rng();
    let mut rand_node = Range::new(0, n.node_count());

    let source = NodeIndex::new(0);
    for i in 0..n.node_count() {
        if i != 0 {
            let des = NodeIndex::new(i);

            let (metric, path) = best_route(&n, source, des);
            println!("{}, {:?}", path[1].index(), n[des].nc);
        }
    }
}