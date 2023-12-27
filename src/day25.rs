use std::collections::HashMap;

use rustworkx_core::{
    connectivity::stoer_wagner_min_cut,
    petgraph::{graph::UnGraph, Graph, Undirected},
    Result,
};

use crate::read_lines;

pub fn day25() {
    let path = "data/day25.txt";
    let num = part1(path);
    println!("Day 25 Part 1 {}", num);
}

fn part1(path: &str) -> usize {
    let mut graph: Graph<&str, &str, Undirected> = UnGraph::new_undirected();

    let lines = read_lines(path);

    let mut nodes_added = vec![];

    let mut node_connections = vec![];
    let mut nodes = HashMap::new();

    for line in lines.iter() {
        let mut split = line.split(": ");

        let k = split.next().unwrap();

        let vv = split.next().unwrap();

        let mut split = vv.split(" ");

        while let Some(v) = split.next() {
            if !nodes_added.contains(&k) {
                let node = graph.add_node(k);
                nodes.insert(k, node);
                nodes_added.push(k);
            }

            if !nodes_added.contains(&v) {
                let node = graph.add_node(v);
                nodes.insert(v, node);
                nodes_added.push(v);
            }

            node_connections.push((k, v));
        }
    }

    let total_nodes = nodes_added.len();

    let mut cons = vec![];
    for connection in node_connections {
        cons.push((
            *nodes.get(connection.0).unwrap(),
            *nodes.get(connection.1).unwrap(),
        ));
    }

    graph.extend_with_edges(&cons);

    let min_cut_res: Result<Option<(usize, Vec<_>)>> = stoer_wagner_min_cut(&graph, |_| Ok(1));

    let (min_cut, partition) = min_cut_res.unwrap().unwrap();
    assert_eq!(min_cut, 3);

    let partition1 = partition.len();
    let partition2 = total_nodes - partition1;

    return partition1 * partition2;
}

#[test]
fn test_part1() {
    let path = "data_demo/day25_demo.txt";
    let count = part1(path);
    assert_eq!(count, 54);
}
