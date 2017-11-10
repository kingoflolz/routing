use petgraph::Graph;
use petgraph::graph::NodeIndex;

use nc::NCNodeData;
use network::{Node, Connection};

use rand::thread_rng;
use rand::distributions::{Sample, Range};

use std::collections::HashMap;

use std::fs::File;
use std::io::prelude::*;

pub fn load_king_nodes() -> Graph<Node, Connection> {
    let mut graph = Graph::<Node, Connection>::new();

    let mut f = File::open("datasets/king/matrix").expect("King matrix dataset not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let split = contents.split("\n");

    for _ in split {
        graph.add_node(Node {
            node_index: None,
            position: [0., 0.],
            nc: NCNodeData::new()
        });
    };

    graph
}

pub fn load_king_measurements() -> HashMap<NodeIndex<u32>, Vec<(NodeIndex<u32>, f32)>> {
    let mut f = File::open("datasets/king/matrix").expect("King matrix dataset not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let split = contents.split("\n");

    let mut rng = thread_rng();
    let mut node_landmarks: HashMap<NodeIndex<u32>, Vec<(NodeIndex<u32>, f32)>> = HashMap::new();

    let mut k = 0;
    for mut i in split {
        let mut node_measurements = Vec::new();
        let mut landmarks_metric: Vec<(NodeIndex<u32>, f32)> = Vec::new();

        for j in i.split(" ") {
            if j.len() > 0 {
                node_measurements.push(j.parse::<i32>().unwrap());
            }
        }

        if node_measurements.len() > 0 {
            let mut random_node = Range::new(0, node_measurements.len());

            for _ in 0..32 {
                let n = random_node.sample(&mut rng);
                if node_measurements[n] as f32 >= 0. {
                    landmarks_metric.push((NodeIndex::new(n), node_measurements[n] as f32 / 1000.));
                }
            }

            node_landmarks.insert(NodeIndex::new(k), landmarks_metric);
            k += 1;
        }
    };

    node_landmarks
}