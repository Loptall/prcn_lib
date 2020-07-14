use super::graph::Graph;
use std::cmp::{min, Reverse};
use std::collections::{BinaryHeap, HashMap};

/// 単純グラフじゃないと死ぬ
pub struct BellmanFord {
    // グラフの骨格部分
    g: Vec<Vec<usize>>,
    // 辺を(頂点, 頂点)の形で特定してその重みを持つ
    e: HashMap<(usize, usize), i64>,
}

impl<'a> Graph<'a> for BellmanFord {
    type NodeId = usize;
    type Iter = std::iter::Cloned<std::slice::Iter<'a, Self::NodeId>>;

    fn len(&self) -> usize {
        self.g.len()
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn index(&self, a: Self::NodeId) -> usize {
        a
    }

    fn neighbors(&'a self, a: Self::NodeId) -> Self::Iter {
        self.g[a].iter().cloned()
    }
}

pub fn make_graph_for_dijkstra(n: usize, edges: &[(usize, usize, i64)]) -> BellmanFord {
    let mut g = vec![Vec::new(); n];
    let mut e = HashMap::new();
    for (from, to, weight) in edges.iter() {
        g[*from].push(*to);
        g[*to].push(*from);
        e.insert((*from, *to), *weight);
        e.insert((*to, *from), *weight);
    }

    BellmanFord { g, e }
}

impl<'a> BellmanFord {
    pub fn weight(&self, from: usize, to: usize) -> i64 {
        self.e[&(from, to)]
    }

    pub fn add_edge(&mut self, edge: (usize, usize, i64)) {
        self.g[edge.0].push(edge.1);
        self.g[edge.1].push(edge.0);
        self.e.insert((edge.0, edge.1), edge.2);
        self.e.insert((edge.1, edge.0), edge.2);
    }
}

pub fn bellman_ford<'a>(g: &'a BellmanFord, start: usize, goal: usize) -> i64 {
    todo!()
}
