/*
	bfs
	This problem requires you to implement a basic BFS algorithm
*/

use std::collections::VecDeque;

// Define a graph
struct Graph {
    adj: Vec<Vec<usize>>, 
}

impl Graph {
    // Create a new graph with n vertices
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    // Add an edge to the graph
    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest); 
        self.adj[dest].push(src); 
    }

    // Perform a breadth-first search on the graph, return the order of visited nodes
    fn bfs_with_return(&self, start: usize) -> Vec<usize> {
        let len = self.adj.len();
        if len == 0 {
            return Vec::new();
        }

        let mut seen = vec![false; len];
        let mut queue = VecDeque::new();
        let mut visit_order = vec![];

        seen[start] = true;
        queue.push_back(start);

        while let Some(node) = queue.pop_front() {
            visit_order.push(node);

            for &next in &self.adj[node] {
                if !seen[next] {
                    seen[next] = true;
                    queue.push_back(next);
                }
            }
        }

        visit_order
    }

    // fn bfs_with_return(&self, start: usize) -> Vec<usize> {
    //     let mut seen = HashSet::new();
    //     let mut visit_order = vec![];
    //     visit_order.push(start);
    //     seen.insert(start);

    //     let result = self.bfs_inner(start, &mut seen);
    //     visit_order.extend_from_slice(&result);

    //     visit_order
    // }

    // fn bfs_inner(&self, start: usize, seen: &mut HashSet<usize>) -> Vec<usize> {
    //     let mut visit_order = vec![];
    //     let mut nodes = vec![];
    //     for &n in &self.adj[start] {
    //         if seen.insert(n) {
    //             nodes.push(n);
    //         }
    //     }
    //     visit_order.extend_from_slice(&nodes);

    //     for node in nodes {
    //         let r = self.bfs_inner(node, seen);
    //         visit_order.extend_from_slice(&r);
    //     }

    //     visit_order
    // }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_all_nodes_visited() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 4);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(1, 4);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 4, 2, 3]);
    }

    #[test]
    fn test_bfs_different_start() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visited_order = graph.bfs_with_return(2);
        assert_eq!(visited_order, vec![2, 1, 0]);
    }

    #[test]
    fn test_bfs_with_cycle() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_bfs_single_node() {
        let mut graph = Graph::new(1);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0]);
    }
}

