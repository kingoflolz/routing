use petgraph::Graph;
use petgraph::graph::NodeIndex;

use nc::{calc_update, NC};

use network::{Node, Connection};

use std::collections::HashMap;

fn clip_nc(a: &mut NC) {
    for i in a.iter_mut() {
        if *i < 0. {
            *i = 0.0001
        }

        if *i > 1000. {
            *i = 1000.
        }
    }
}

pub fn init_nc<'a>(graph: &'a mut Graph<Node, Connection>, node_landmarks: &HashMap<NodeIndex<u32>, Vec<(NodeIndex<u32>, f32)>>) -> &'a mut Graph<Node, Connection> {
    let mut m = Vec::new();

    for epochs in 1..200 {
        for (&i, landmarks) in node_landmarks {
            for &(j, actual) in landmarks {
                let predicted = graph[j].nc.incoming_vec.dot(&graph[i].nc.outgoing_vec);
                let difference = actual - predicted;

                m.push(difference.abs() / actual);

                let (a_u, b_u) = calc_update(graph[j].nc.incoming_vec, graph[i].nc.outgoing_vec, actual, graph[i].nc.learn_rate);

                graph[j].nc.incoming_vec += a_u;

                graph[i].nc.outgoing_vec += b_u;

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
            for i in 1..10 {
                percentile[i - 1] = m[(i * (l - 1)) / 10];
            }
            println!("!!! {:?}", percentile);
        }

        // println!("{:?}", m);
        m = Vec::new();
    }
    println!("done");

    graph
}