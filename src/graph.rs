use std::ops::{Add, AddAssign};

pub trait Graph<'a> {
    type NodeId: Copy;
    type Iter: Iterator<Item = Self::NodeId>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn index(&self, a: Self::NodeId) -> usize;
    fn neighbors(&'a self, a: Self::NodeId) -> Self::Iter;
}

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

pub fn make_directed_graph(n: usize, edges: &[(usize, usize)]) -> UnweightedGraph {
    let mut g = vec![vec![]; n];
    for &(u, v) in edges.iter() {
        g[u].push(v);
    }
    g
}

pub fn make_undirected_graph(n: usize, edges: &[(usize, usize)]) -> UnweightedGraph {
    let mut g = vec![vec![]; n];
    for &(u, v) in edges.iter() {
        g[u].push(v);
        g[v].push(u);
    }
    g
}

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
