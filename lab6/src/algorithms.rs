use crate::graph::Graph;
use rand::seq::SliceRandom;
use std::sync::atomic::{AtomicBool, Ordering};

pub fn find_hamiltonian_cycle(graph: &Graph, thread_count: usize) -> Vec<usize> {
    let found = AtomicBool::new(false);

    std::thread::scope(|scope| {
        let threads =
            std::iter::repeat_with(|| scope.spawn(|| try_find_hamiltonian_cycle(graph, &found)))
                .take(thread_count)
                .collect::<Vec<_>>();

        threads
            .into_iter()
            .flat_map(|thread| thread.join().expect("Failed to join thread"))
            .next()
            .expect("No solution found")
    })
}

pub fn try_find_hamiltonian_cycle(graph: &Graph, found: &AtomicBool) -> Option<Vec<usize>> {
    let mut rng = rand::thread_rng();
    let mut path = (0..graph.node_count()).collect::<Vec<_>>();

    while !found.load(Ordering::Relaxed) {
        path.shuffle(&mut rng);

        let all_connected_inside = path.windows(2).all(|nodes| graph.has_edge(nodes[0], nodes[1]));

        if let (Some(&first), Some(&last)) = (path.first(), path.last()) {
            if all_connected_inside && graph.has_edge(last, first) {
                return Some(path);
            }
        }
    }

    None
}
