/*
    graph
    This problem requires you to implement a basic graph functio
*/

use std::collections::{HashMap, HashSet};
use std::fmt;
#[derive(Debug, Clone)]
pub struct NodeNotInGraph;
impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}
pub struct UndirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}
impl Graph for UndirectedGraph {
    fn new() -> UndirectedGraph {
        UndirectedGraph {
            adjacency_table: HashMap::new(),
        }
    }
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        let (from, to, weight) = edge;
        let from_str = from.to_string();
        let to_str = to.to_string();

        // 1. 确保两个节点都在图中（不存在则插入空邻接表）
        self.adjacency_table_mutable()
            .entry(from_str.clone()) // 此处 clone()：避免后续移动 from_str 影响 entry
            .or_insert_with(Vec::new);
        self.adjacency_table_mutable()
            .entry(to_str.clone()) // 此处 clone()：同理，确保 to_str 可重复使用
            .or_insert_with(Vec::new);

        // 2. 给 from 的邻接表添加 to（用 clone() 复制 to_str，避免移动）
        let from_neighbours = self.adjacency_table_mutable().get_mut(&from_str).unwrap();
        if !from_neighbours.contains(&(to_str.clone(), weight)) {
            from_neighbours.push((to_str.clone(), weight)); // 再次 clone()：contains 中用的是临时复制，不影响原 to_str
        }

        // 3. 给 to 的邻接表添加 from（用 clone() 复制 from_str，避免移动后无法使用）
        let to_neighbours = self.adjacency_table_mutable().get_mut(&to_str).unwrap();
        if !to_neighbours.contains(&(from_str.clone(), weight)) {
            // 关键修复：from_str.clone() 避免移动
            to_neighbours.push((from_str, weight)); // 此时使用原 from_str，所有权转移到邻接表
        }
    }
}
pub trait Graph {
    fn new() -> Self;
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;
    fn add_node(&mut self, node: &str) -> bool {
        let node_str = node.to_string();
        // 先检查是否存在，不存在则插入
        self.adjacency_table_mutable()
            .entry(node_str)
            .or_insert_with(Vec::new)
            .is_empty() // 若插入了新节点，邻接表是空的（返回 true）；若已存在，邻接表非空（返回 false）
    }
    fn add_edge(&mut self, edge: (&str, &str, i32));

    fn contains(&self, node: &str) -> bool {
        self.adjacency_table().get(node).is_some()
    }
    fn nodes(&self) -> HashSet<&String> {
        self.adjacency_table().keys().collect()
    }
    fn edges(&self) -> Vec<(&String, &String, i32)> {
        let mut edges = Vec::new();
        for (from_node, from_node_neighbours) in self.adjacency_table() {
            for (to_node, weight) in from_node_neighbours {
                edges.push((from_node, to_node, *weight));
            }
        }
        edges
    }
}
#[cfg(test)]
mod test_undirected_graph {
    use super::Graph;
    use super::UndirectedGraph;
    #[test]
    fn test_add_edge() {
        let mut graph = UndirectedGraph::new();
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));
        let expected_edges = [
            (&String::from("a"), &String::from("b"), 5),
            (&String::from("b"), &String::from("a"), 5),
            (&String::from("c"), &String::from("a"), 7),
            (&String::from("a"), &String::from("c"), 7),
            (&String::from("b"), &String::from("c"), 10),
            (&String::from("c"), &String::from("b"), 10),
        ];
        for edge in expected_edges.iter() {
            assert_eq!(graph.edges().contains(edge), true);
        }
    }
}
