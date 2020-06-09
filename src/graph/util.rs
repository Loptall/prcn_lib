use crate::graph::graph::{UnweightedGraph, WeightedVertexGraph};

/// 重みなし有向グラフ
pub fn make_directed_graph(n: usize, edges: &[(usize, usize)]) -> UnweightedGraph {
    let mut g = vec![vec![]; n];
    for &(u, v) in edges.iter() {
        g[u].push(v);
    }
    g
}

/// 重みなし無向グラフ
pub fn make_undirected_graph(n: usize, edges: &[(usize, usize)]) -> UnweightedGraph {
    let mut g = vec![vec![]; n];
    for &(u, v) in edges.iter() {
        g[u].push(v);
        g[v].push(u);
    }
    g
}

/// 重み付き有向グラフ
pub fn make_weighted_directed_graph<W: Clone>(
    n: usize,
    edges: &[(usize, usize, W)],
) -> WeightedVertexGraph<W> {
    let mut g = vec![vec![]; n];
    for &(u, v, ref w) in edges.iter() {
        g[u].push((v, w.clone()));
    }
    g
}

/// 重み付き無向グラフ
pub fn make_weighted_undirected_graph<W: Clone>(
    n: usize,
    edges: &[(usize, usize, W)],
) -> WeightedVertexGraph<W> {
    let mut g = vec![vec![]; n];
    for &(u, v, ref w) in edges.iter() {
        g[u].push((v, w.clone()));
        g[v].push((u, w.clone()));
    }
    g
}
