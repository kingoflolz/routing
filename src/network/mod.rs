use petgraph::Graph;
use petgraph::graph::NodeIndex;
use petgraph::algo::astar;

use spade::HasPosition;

use nc::NCNodeData;

pub type Network = Graph<Node, Connection>;

pub mod nc;
pub mod load;
pub mod generate;
pub mod routing;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Connection {
    latency: f32,
    //in ms
    bandwidth: f32,
    //in kbps
    packet_loss: f32
    //in percent
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Node {
    node_index: Option<NodeIndex>,
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

pub fn best_route(graph: &Graph<Node, Connection>, start: NodeIndex, end: NodeIndex) -> (f32, Vec<NodeIndex>) {
    astar(&graph, start, |finish| finish == end, |e| e.weight().latency, |_| 0.).unwrap()
}
