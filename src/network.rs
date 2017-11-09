use petgraph::Graph;
use petgraph::graph::NodeIndex;
use petgraph::algo::dijkstra;

use spade::HasPosition;
use spade::rtree::RTree;

use nc::{NCNodeData, calc_update, NC};

use rand::{thread_rng};
use rand::distributions::{Sample, Range};

use std::collections::HashMap;

use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Debug)]
pub struct Connection {
    latency: f32,
    //in ms
    bandwidth: f32,
    //in kbps
    packet_loss: f32
    //in percent
}

#[derive(Clone, Debug)]
pub struct Node {
    node_index: Option<NodeIndex>,
    level: u8,
    pub added: usize,
    position: [f32; 2],
    nc: NCNodeData,
}

#[derive(Clone, Debug)]
pub struct MapNode {
    position: [f32; 2],
    node_index: NodeIndex,
}

impl HasPosition for MapNode {
    type Point = [f32; 2];
    fn position(&self) -> [f32; 2] {
        self.position
    }
}

pub fn best_route(graph: &Graph<Node, Connection>, start: NodeIndex, end: NodeIndex) -> f32 {
    let m = dijkstra(&graph, start, Some(end), |e| e.weight().latency);

    assert!(m.contains_key(&end));
    return m[&end];
}

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
            level: 0,
            added: 0,
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

pub fn generate_flat_graph() -> Graph<Node, Connection> {
    let mut graph = Graph::<Node, Connection>::new();

    let c = Connection { latency: 10f32, bandwidth: 10f32, packet_loss: 1f32 };
    println!("Starting");

    let mut rtree = RTree::new();
    for i in 0..10 {
        for j in 0..10 {
            let p = [i as f32, j as f32];
            let index = graph.add_node(
                Node {
                    node_index: None,
                    level: 0,
                    added: 0,
                    position: p,
                    nc: NCNodeData::new()
                }
            );
            graph[index].node_index = Some(index);
            rtree.insert(MapNode { position: p, node_index: index });
        }
    }

    println!("Added to rtree");

    for i in graph.node_indices() {
        for j in rtree.lookup_in_circle(&graph[i].position, &1.1f32) {
            if i != j.node_index {
                graph.add_edge(i, j.node_index, c.clone());
                // println!("{:?}", &graph[j.node_index].nc);
            }
        }
    }

    println!("Added edges");

    graph
}

pub fn calc_measurements(graph: &mut Graph<Node, Connection>) -> HashMap<NodeIndex<u32>, Vec<(NodeIndex<u32>, f32)>> {
    let graph_other = graph.clone();
    let mut rng = thread_rng();

    let mut random_node = Range::new(0, graph.node_count());

    let mut node_landmarks: HashMap<NodeIndex<u32>, Vec<(NodeIndex<u32>, f32)>> = HashMap::new();

    for i in graph.node_weights_mut() {
        let mut landmarks: Vec<NodeIndex<u32>> = Vec::new();

        for _ in 0..32 {
            landmarks.push(NodeIndex::new(random_node.sample(&mut rng)));
        }

        for j in graph_other.neighbors(i.node_index.unwrap()) {
            landmarks.push(j);
        }

        let mut landmarks_metric: Vec<(NodeIndex<u32>, f32)> = Vec::new();
        let destinations = dijkstra(&graph_other, i.node_index.unwrap(), None, |e| e.weight().latency);
        for j in landmarks {
            let actual_metric = destinations[&j];
            landmarks_metric.push((j, actual_metric));
            // println!("{}, {}, {}", i.node_index.unwrap().index(), j.index(), actual_metric);
        }
        node_landmarks.insert(i.node_index.unwrap(), landmarks_metric);
    }
    node_landmarks
}

fn clip_nc(a: &mut NC){
    for i in a.iter_mut() {
        if *i < 0. {
            *i = 0.0001
        }

        if *i > 1000. {
            *i = 1000.
        }
    }
}

pub fn init_nc(graph: &mut Graph<Node, Connection>, node_landmarks: HashMap<NodeIndex<u32>, Vec<(NodeIndex<u32>, f32)>>) -> &mut Graph<Node, Connection> {
    let mut n = 0;

    let mut m = Vec::new();

    for epochs in 1..200 {
        n = 0;
        for (&i, landmarks) in &node_landmarks {
            for &(j, actual) in landmarks {
                let predicted = graph[j].nc.incoming_vec.dot(&graph[i].nc.outgoing_vec);
                let difference = actual - predicted;

                m.push(difference.abs() / actual);
                n += 1;

                let (a_u, b_u) = calc_update(graph[j].nc.incoming_vec.clone(), graph[i].nc.outgoing_vec.clone(), actual, graph[i].nc.learn_rate);

                graph[j].nc.incoming_vec = graph[j].nc.incoming_vec + a_u;

                graph[i].nc.outgoing_vec = graph[i].nc.outgoing_vec + b_u;

                let better = graph[j].nc.incoming_vec.dot(&graph[i].nc.outgoing_vec);
                let change = difference.abs() - (actual - better).abs();
                if change > 1. {
                    graph[i].nc.learn_rate /= 2.;
                    // println!("{}", graph[*i].nc.learn_rate);
                }
            }
            clip_nc(&mut graph[i].nc.incoming_vec);
            clip_nc(&mut graph[i].nc.outgoing_vec);

        }

        if epochs % 10 == 0 {
            m.sort_by(|a, b| a.partial_cmp(b).unwrap());

            let mut percentile = [0.0f32; 9];
            let l = m.len();
            for i in 1..10{
                percentile[i-1] = m[(i * (l-1)) / 10];
            }
            println!("!!! {:?}", percentile);
        }

        // println!("{:?}", m);
        m = Vec::new();
    }
    println!("!!! Total connections {}", n);
    println!("done");

    graph
}