use petgraph::Graph;
use petgraph::graph::NodeIndex;
use petgraph::algo::dijkstra;

use spade::HasPosition;
use spade::rtree::RTree;

use nc::NCNodeData;

use rand::thread_rng;
use rand::distributions::{Sample, Range};

use std::collections::HashMap;

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

pub fn generate() -> Graph<Node, Connection> {
    let mut graph = Graph::<Node, Connection>::new();

    let c = Connection { latency: 10f32, bandwidth: 10f32, packet_loss: 1f32 };

    let mut rtree = RTree::new();
    for i in 0..20 {
        for j in 0..20 {
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

    for i in graph.node_indices() {
        for j in rtree.lookup_in_circle(&graph[i].position, &1.1f32) {
            graph.add_edge(i, j.node_index, c.clone());
            // println!("{:?}", &graph[j.node_index].nc);
        }
    }

    let graph_other = &graph.clone();
    let mut rng = thread_rng();

    let mut random_node = Range::new(0, graph.node_count());

    let mut node_landmarks: HashMap<NodeIndex<u32>, Vec<(NodeIndex<u32>, f32)>> = HashMap::new();

    for i in graph.node_weights_mut() {
        let mut landmarks: Vec<NodeIndex<u32>> = Vec::new();

        for _ in 0..20 {
            landmarks.push(NodeIndex::new(random_node.sample(&mut rng)));
        }

        for j in graph_other.neighbors(i.node_index.unwrap()) {
            landmarks.push(j);
        }

        let mut landmarks_metric: Vec<(NodeIndex<u32>, f32)> = Vec::new();
        let destinations = dijkstra(&graph_other, i.node_index.unwrap(), None, |e| e.weight().latency);
        for j in landmarks{
            let actual_metric = destinations[&j];
            landmarks_metric.push((j, actual_metric));
        }
        node_landmarks.insert(i.node_index.unwrap(), landmarks_metric);
    }

    println!("Building landmarks/reference measurements");

    for epochs in 0..100 {
        for (i, landmarks) in &node_landmarks {
            for &(j, actual) in landmarks {
                let predicted = graph_other[j].nc.incoming_vec.dot(&graph_other[*i].nc.outgoing_vec);

            }
            // println!("{:?}, {}", landmarks, landmarks.len());
        }
    }

    graph
}