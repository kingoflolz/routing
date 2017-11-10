use petgraph::Graph;
use petgraph::graph::NodeIndex;
use petgraph::algo::dijkstra;

use spade::rtree::RTree;

use nc::NCNodeData;

use rand::thread_rng;
use rand::distributions::{Weighted, WeightedChoice,Sample, Range};

use std::collections::HashMap;

use network::{MapNode, Node, Connection};

pub fn generate_flat_graph() -> Graph<Node, Connection> {
    let mut graph = Graph::<Node, Connection>::new();

    let c = Connection { latency: 10f32, bandwidth: 10f32, packet_loss: 1f32 };
    println!("Starting");

    let mut rtree = RTree::new();
    for i in 0..20 {
        for j in 0..20 {
            let p = [i as f32, j as f32];
            let index = graph.add_node(
                Node {
                    node_index: None,
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

fn latency(a: &[f32; 2], b: &[f32; 2]) -> f32 {
    ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2)).sqrt() / 3e5
}

pub fn generate_hier_graph() -> Graph<Node, Connection> {
    let mut graph = Graph::<Node, Connection>::new();

    let mut rng = thread_rng();

    let mut area = Range::new(-1e6, 1e6);
    let mut latency_jitter = Range::new(0., 10.);
    let mut items = vec!(Weighted { weight: 10, item: 2 },
                         Weighted { weight: 1, item: 2 });
    let mut wc = WeightedChoice::new(&mut items);

    println!("Starting graph generation...");

    let mut rtrees: Vec<RTree<MapNode>> = Vec::new();

    let neighbors = [10, 5, 5, 1]; // same level
    for level in 0..4 {
        let mut rtree = RTree::new();
        for _ in 0..10u32.pow(level + 1) {
            let p = [area.sample(&mut rng), area.sample(&mut rng)];
            let index = graph.add_node(
                Node {
                    node_index: None,
                    position: p,
                    nc: NCNodeData::new()
                }
            );
            graph[index].node_index = Some(index);
            rtree.insert(MapNode { position: p, node_index: index });

            //to higher level
            if level > 0 {
                for j in rtrees[(level-1) as usize].nearest_n_neighbors(&p, wc.sample(&mut rng)) {
                    let c = Connection { latency: latency(&j.position, &p) + latency_jitter.sample(&mut rng), bandwidth: 10f32, packet_loss: 1f32 };
                    graph.update_edge(index, j.node_index, c.clone());
                    graph.update_edge(j.node_index, index, c.clone());
                }
            }
        }

        //same level connection
        for i in rtree.iter() {
            if neighbors[level as usize] > 0 {
                for j in rtree.nearest_n_neighbors(&graph[i.node_index].position, neighbors[level as usize]) {
                    let c = Connection { latency: latency(&i.position, &j.position) + latency_jitter.sample(&mut rng), bandwidth: 10f32, packet_loss: 1f32 };
                    graph.update_edge(i.node_index, j.node_index, c.clone());
                    graph.update_edge(j.node_index, i.node_index, c.clone());
                }
            }
        }
        rtrees.push(rtree);
    }
    println!("Completed graph generation...");
    graph
}


pub fn calc_measurements(graph: &mut Graph<Node, Connection>) -> HashMap<NodeIndex<u32>, Vec<(NodeIndex<u32>, f32)>> {
    println!("Starting landmark measurements...");

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
    println!("Completed landmark measurements");
    node_landmarks
}