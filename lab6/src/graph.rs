use rand::seq::SliceRandom;

#[derive(Clone, Debug)]
pub struct Graph {
    node_count: usize,
    edges: Vec<bool>,
}

impl Graph {
    pub fn new(node_count: usize) -> Self {
        Graph { node_count, edges: vec![false; node_count * node_count] }
    }
    
    pub fn with_random_edges(node_count: usize, fill_ratio: f32) -> Self {
        let edges = {
            let edges_len = node_count * node_count;
            let edge_count = (edges_len as f32 * fill_ratio) as usize;        
            let non_edge_count = edges_len - edge_count;
            
            let mut edges = std::iter::repeat(true)
                .take(edge_count)
                .chain(std::iter::repeat(false).take(non_edge_count))
                .collect::<Vec<_>>();

            edges.shuffle(&mut rand::thread_rng());
            edges
        };
        
        Self {
            node_count,
            edges,
        }
    }

    pub fn set_edge(&mut self, a: usize, b: usize, has_edge: bool) {
        debug_assert!(a < self.node_count);
        debug_assert!(b < self.node_count);

        self.edges[a * self.node_count + b] = has_edge;
    }

    pub fn set_double_edge(&mut self, a: usize, b: usize, has_edge: bool) {
        debug_assert!(a < self.node_count);
        debug_assert!(b < self.node_count);

        self.edges[a * self.node_count + b] = has_edge;
        self.edges[b * self.node_count + a] = has_edge;
    }

    pub fn has_edge(&self, a: usize, b: usize) -> bool {
        debug_assert!(a < self.node_count);
        debug_assert!(b < self.node_count);

        self.edges[a * self.node_count + b]
    }

    pub fn node_count(&self) -> usize {
        self.node_count
    }
}
