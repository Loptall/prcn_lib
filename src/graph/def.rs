use cargo_snippet::snippet;

use super::{
    bfs::{bfs, Bfs},
    dfs::{dfs, Dfs},
};

#[snippet("graph")]
pub trait Graph<'a> {
    type NodeId: Copy;
    type Iter: Iterator<Item = Self::NodeId>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn index(&self, i: Self::NodeId) -> usize;
    fn neighbors(&'a self, i: Self::NodeId) -> Self::Iter;
}

/// 重みなしグラフ
#[snippet("graph")]
pub struct UnweightedGraph(Vec<Vec<usize>>);

#[snippet("graph")]
impl UnweightedGraph {
    pub fn new(graph: Vec<Vec<usize>>) -> Self {
        Self(graph)
    }
}

#[snippet("graph")]
impl<'a> Graph<'a> for UnweightedGraph {
    type NodeId = usize;
    type Iter = std::iter::Cloned<std::slice::Iter<'a, Self::NodeId>>;

    fn len(&self) -> usize {
        self.0.len()
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn index(&self, a: Self::NodeId) -> usize {
        a
    }

    fn neighbors(&'a self, a: Self::NodeId) -> Self::Iter {
        self.0[a].iter().cloned()
    }
}

#[snippet("graph")]
impl<'a> UnweightedGraph {
    pub fn dfs(&'a self, start: usize) -> Dfs<'a, UnweightedGraph> {
        dfs(self, start)
    }

    pub fn bfs(&'a self, start: usize) -> Bfs<'a, UnweightedGraph> {
        bfs(self, start)
    }
}

/// 重みありグラフ
#[snippet("graph")]
pub struct WeightedNodeGraph<W>(Vec<Vec<(usize, W)>>);

#[snippet("graph")]
impl<W> WeightedNodeGraph<W> {
    pub fn new(graph: Vec<Vec<(usize, W)>>) -> Self {
        Self(graph)
    }
}

#[snippet("graph")]
impl<'a, W> Graph<'a> for WeightedNodeGraph<W>
where
    W: std::marker::Copy + Clone + 'a,
{
    type NodeId = (usize, W);
    type Iter = std::iter::Cloned<std::slice::Iter<'a, Self::NodeId>>;

    fn len(&self) -> usize {
        self.0.len()
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn index(&self, a: Self::NodeId) -> usize {
        a.0
    }

    fn neighbors(&'a self, a: Self::NodeId) -> Self::Iter {
        self.0[a.0].iter().cloned()
    }
}
