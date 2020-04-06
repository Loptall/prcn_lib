//! Varified

use std::collections::VecDeque;

pub trait Graph<'a> {
    type NodeId: Copy;
    type Iter: Iterator<Item = Self::NodeId>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn index(&self, a: Self::NodeId) -> usize;
    fn neighbors(&'a self, a: Self::NodeId) -> Self::Iter;
}

/// 重みなしグラフ
pub type UnweightedGraph = Vec<Vec<usize>>;

impl<'a> Graph<'a> for UnweightedGraph {
    type NodeId = usize;
    type Iter = std::iter::Cloned<std::slice::Iter<'a, Self::NodeId>>;

    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn index(&self, a: Self::NodeId) -> usize {
        a
    }

    fn neighbors(&'a self, a: Self::NodeId) -> Self::Iter {
        self[a].iter().cloned()
    }
}

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

/// 重みありグラフ
pub type WeightedGraph<W> = Vec<Vec<(usize, W)>>;

impl<'a, W> Graph<'a> for WeightedGraph<W>
where
    W: std::marker::Copy + Clone + 'a,
{
    type NodeId = (usize, W);
    type Iter = std::iter::Cloned<std::slice::Iter<'a, Self::NodeId>>;

    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn index(&self, a: Self::NodeId) -> usize {
        a.0
    }

    fn neighbors(&'a self, a: Self::NodeId) -> Self::Iter {
        self[a.0].iter().cloned()
    }
}

/// 重み付き有向グラフ
pub fn make_weighted_directed_graph<W: Clone>(
    n: usize,
    edges: &[(usize, usize, W)],
) -> WeightedGraph<W> {
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
) -> WeightedGraph<W> {
    let mut g = vec![vec![]; n];
    for &(u, v, ref w) in edges.iter() {
        g[u].push((v, w.clone()));
        g[v].push((u, w.clone()));
    }
    g
}

pub struct Dfs<'a, G: Graph<'a>> {
    visited: Vec<bool>,
    s: Vec<(G::NodeId, Option<G::NodeId>)>,
    g: &'a G,
}

impl<'a, G: Graph<'a>> Iterator for Dfs<'a, G> {
    type Item = (G::NodeId, G::NodeId);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((u, prev)) = self.s.pop() {
            for v in self.g.neighbors(u) {
                if !self.visited[self.g.index(v)] {
                    self.visited[self.g.index(v)] = true;
                    self.s.push((v, Some(u)));
                }
            }

            if let Some(prev) = prev {
                Some((prev, u))
            } else {
                self.next()
            }
        } else {
            None
        }
    }
}

pub fn dfs<'a, G: Graph<'a, NodeId = usize>>(g: &'a G, start: G::NodeId) -> Dfs<'a, G> {
    let n = g.len();
    let mut visited = vec![false; n];
    let mut s = Vec::new();
    visited[start] = true;
    s.push((start, None));

    Dfs { visited, s, g }
}

pub struct Bfs<'a, G: Graph<'a>> {
    visited: Vec<bool>,
    q: VecDeque<(G::NodeId, Option<G::NodeId>)>,
    g: &'a G,
}

impl<'a, G: Graph<'a>> Iterator for Bfs<'a, G> {
    type Item = (G::NodeId, G::NodeId);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((u, prev)) = self.q.pop_front() {
            for v in self.g.neighbors(u) {
                if !self.visited[self.g.index(v)] {
                    self.visited[self.g.index(v)] = true;
                    self.q.push_back((v, Some(u)));
                }
            }

            if let Some(prev) = prev {
                Some((prev, u))
            } else {
                self.next()
            }
        } else {
            None
        }
    }
}

pub fn bfs<'a, G: Graph<'a, NodeId = usize>>(g: &'a G, start: G::NodeId) -> Bfs<'a, G> {
    let n = g.len();
    let mut visited = vec![false; n];
    let mut q = VecDeque::new();
    visited[start] = true;
    q.push_back((start, None));

    Bfs { visited, q, g }
}

/// Returns a vector which stores distances from `start`.
/// For unreachable node, `usize::MAX` is stored.
pub fn make_dist_table<'a, G: Graph<'a, NodeId = usize>>(g: &'a G, start: G::NodeId) -> Vec<usize> {
    let mut dist = vec![std::usize::MAX; g.len()];
    dist[start] = 0;
    for (u, v) in bfs(g, start) {
        dist[v] = dist[u] + 1;
    }
    dist
}
