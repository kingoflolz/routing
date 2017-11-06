use petgraph::Graph;
use petgraph::graph::NodeIndex;
use petgraph::algo::dijkstra;

use spade::HasPosition;
use spade::rtree::RTree;

use nc::{NCNodeData, calc_update};

use rand::{thread_rng, Rng};
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

    let mut graph_other = graph.clone();
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
        for j in landmarks{
            let actual_metric = destinations[&j];
            landmarks_metric.push((j, actual_metric));
            // println!("{}, {}, {}", i.node_index.unwrap().index(), j.index(), actual_metric);
        }
        node_landmarks.insert(i.node_index.unwrap(), landmarks_metric);
    }

    // println!("Built landmarks/reference measurements");

    let mut n = 0;
    let mut s = 0.;
    let mut c = 0.;

    let mut p = 0.;
    let mut d = 0.;

    let mut m = Vec::new();

    let mut enabled = [false; 400];

    let mut n_enabled = 1;

    enabled[0] = true;

    for epochs in 1..1000 {
        for (i, landmarks) in &mut node_landmarks {
            if enabled[i.index()] {
                for &mut (j, actual) in landmarks {
                    if rng.next_f32() < 1./n_enabled as f32 {
                        enabled[j.index()] = true;
                        n_enabled += 1;
                    }
                    if *i != j && enabled[j.index()] {
                        let predicted = graph_other[j].nc.incoming_vec.dot(&graph_other[*i].nc.outgoing_vec);
                        let difference = actual - predicted;

                        d += predicted;

                        p += difference.abs() / actual;
                        m.push(difference.abs() / actual);
                        n += 1;
                        s += difference.abs() * difference.abs();

                        let (a_u, b_u) = calc_update(graph_other[j].nc.incoming_vec.clone(), graph_other[*i].nc.outgoing_vec.clone(), actual);

                        //println!("{}", update);

                        for i in a_u.clone().iter_mut() {
                            //*i *= 0.05;
                        }

                        graph[j].nc.incoming_vec = graph_other[j].nc.incoming_vec + a_u;

                        for i in graph[j].nc.incoming_vec.iter_mut() {
                            if *i < 0. {
                                *i = 0.
                            }
                        }

                        graph[*i].nc.outgoing_vec = graph_other[*i].nc.outgoing_vec + b_u;

                        for i in graph[*i].nc.outgoing_vec.iter_mut() {
                            if *i < 0. {
                                *i = 0.
                            }
                        }

                        let better = graph[j].nc.incoming_vec.dot(&graph[*i].nc.outgoing_vec);
                        let change = difference.abs() - (actual - better).abs();
                        c += change;
                        // println!("{}", change)
                    }
                }
                // rng.shuffle(&mut landmarks);
                // println!("{:?}, {}", landmarks, landmarks.len())
            };
        }
        graph_other = graph.clone();

        m.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let l = m.len();

        if epochs%1 == 0 {
            println!("!!! {}, {}, {}, {}, {}, {}", s/n as f32, c/n as f32, p/n as f32, d/n as f32, m[l/2], n_enabled);
        }

        n = 0;
        s = 0.;
        c = 0.;
        p = 0.;
        d = 0.;
        m = Vec::new();
    }

    for i in graph.node_weights_mut() {
        println!("{:?}", i.nc.outgoing_vec);
    }

    for (i, landmarks) in &mut node_landmarks {
        //println!("{:?}, {:?}", i, graph[*i].nc.incoming_vec);
        for &mut (j, actual) in landmarks {
            if *i != j {
                //let predicted = graph_other[j].nc.incoming_vec.dot(&graph_other[*i].nc.outgoing_vec);
                //let difference = actual - predicted;
                //println!("{}, {}, {}", predicted, difference, difference.abs() / actual);
            }
        }
    }

    println!("done");

    graph
}