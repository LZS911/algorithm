use std::cmp::Ordering::*;
use std::collections::BinaryHeap;

struct FindPath<'a> {
    edges: &'a Vec<Vec<(usize, i32)>>,
    threshold: i32,
}

impl<'a> FindPath<'a> {
    fn find_lower_bound(&mut self, start: usize) -> i32 {
        // min-heap
        let mut binary_heap = BinaryHeap::new();
        binary_heap.push((0, start));
        // distance
        let mut distance = vec![i32::MAX; self.edges.len()];
        distance[start] = 0;
        // count
        let mut count = 0;
        // loop
        while let Some((mut _distance, _vertex)) = binary_heap.pop() {
            _distance = -_distance;
            for &(next_vertex, edge_length) in &self.edges[_vertex] {
                if _distance + edge_length < distance[next_vertex]
                    && _distance + edge_length <= self.threshold
                {
                    if distance[next_vertex] == i32::MAX {
                        count += 1;
                    }
                    distance[next_vertex] = _distance + edge_length;
                    binary_heap.push((-distance[next_vertex], next_vertex));
                }
            }
        }
        count
    }
}

impl Solution {
    pub fn find_the_city(n: i32, _edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
        let n = n as usize;
        let mut edges: Vec<Vec<(usize, i32)>> = vec![vec![]; n];
        for e in _edges {
            let f = e[0] as usize;
            let s = e[1] as usize;
            edges[f].push((s, e[2]));
            edges[s].push((f, e[2]));
        }
        let mut path_finder = FindPath {
            edges: &edges,
            threshold: distance_threshold,
        };
        let count: Vec<(i32, i32)> = (0..n)
            .map(|x| (path_finder.find_lower_bound(x), x as i32))
            .collect();
        count
            .iter()
            .copied()
            .min_by(
                |&(x_count, x_index), &(y_count, y_index)| match x_count.cmp(&y_count) {
                    Equal => x_index.cmp(&y_index).reverse(),
                    otherwise => otherwise,
                },
            )
            .unwrap()
            .1
    }
}
