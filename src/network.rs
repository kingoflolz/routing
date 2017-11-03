use petgraph::Graph;
use petgraph::graph::NodeIndex;
use spade::HasPosition;
use spade::rtree::{RTree};
use cgmath::Point2;

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
    position: Point2<f32>,

}

#[derive(Clone, Debug)]
pub struct MapNode {
    position: Point2<f32>,
    node_index: NodeIndex,
}

impl HasPosition for MapNode {
    type Point = Point2<f32>;
    fn position(&self) -> Point2<f32> {
        self.position
    }
}

pub fn generate_graph() -> Graph<Node, Connection> {
    let mut graph = Graph::<Node, Connection>::new();

    let c = Connection{latency: 10f32, bandwidth: 10f32, packet_loss: 1f32 };

    let mut rtree = RTree::new();
    for i in 0..20{
        for j in 0..20{
            let p = Point2::new(i as f32, j as f32);
            let index = graph.add_node(Node{node_index:None, level: 0, position:p});
            graph[index].node_index = Some(index);
            rtree.insert(MapNode{position:p, node_index: index});
        }
    }

    for i in graph.node_indices() {
        for j in rtree.lookup_in_circle(&graph[i].position, &1.1f32){
            graph.add_edge(i, j.node_index, c.clone());
        }
    }

    graph
}