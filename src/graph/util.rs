use super::def::{UnweightedGraph, WeightedNodeGraph};
use cargo_snippet::snippet;

/// 重みなし有向グラフ
#[snippet("graph")]
pub fn make_directed_graph(n: usize, edges: &[(usize, usize)]) -> UnweightedGraph {
    let mut g = vec![vec![]; n];
    for &(u, v) in edges.iter() {
        g[u].push(v);
    }
    UnweightedGraph::new(g)
}

/// 重みなし無向グラフ
#[snippet("graph")]
pub fn make_undirected_graph(n: usize, edges: &[(usize, usize)]) -> UnweightedGraph {
    let mut g = vec![vec![]; n];
    for &(u, v) in edges.iter() {
        g[u].push(v);
        g[v].push(u);
    }
    UnweightedGraph::new(g)
}

/// 重み付き有向グラフ
#[snippet("graph")]
pub fn make_weighted_directed_graph<W: Clone>(
    n: usize,
    edges: &[(usize, usize, W)],
) -> WeightedNodeGraph<W> {
    let mut g = vec![vec![]; n];
    for &(u, v, ref w) in edges.iter() {
        g[u].push((v, w.clone()));
    }
    WeightedNodeGraph::new(g)
}

/// 重み付き無向グラフ
#[snippet("graph")]
pub fn make_weighted_undirected_graph<W: Clone>(
    n: usize,
    edges: &[(usize, usize, W)],
) -> WeightedNodeGraph<W> {
    let mut g = vec![vec![]; n];
    for &(u, v, ref w) in edges.iter() {
        g[u].push((v, w.clone()));
        g[v].push((u, w.clone()));
    }
    WeightedNodeGraph::new(g)
}
