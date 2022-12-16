#![allow(dead_code)]

mod algorithms;
mod graph;

use crate::graph::Graph;

fn print_graph(graph: &Graph) {
    println!("Graph:");

    for i in 0..graph.node_count() {
        for j in 0..graph.node_count() {
            print!("{} ", graph.has_edge(i, j) as usize);
        }
        
        println!();
    }
} 

fn main() {
    let graph = Graph::with_random_edges(10, 0.05);
    print_graph(&graph);

    println!();

    let solution = algorithms::find_hamiltonian_cycle(&graph, 8);
    println!("Solution:\n{:?}", solution);
}
