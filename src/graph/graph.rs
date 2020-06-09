//! Varified
// use std::collections::{HashMap, VecDeque};

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

/// 重みありグラフ
pub type WeightedVertexGraph<W> = Vec<Vec<(usize, W)>>;

impl<'a, W> Graph<'a> for WeightedVertexGraph<W>
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

// /// Returns a vector which stores distances from `start`.
// /// For unreachable node, `usize::MAX` is stored.
// pub fn make_dist_table<'a, G: Graph<'a, NodeId = usize>>(g: &'a G, start: G::NodeId) -> Vec<usize> {
//     let mut dist = vec![std::usize::MAX; g.len()];
//     dist[start] = 0;
//     for (u, v) in bfs(g, start) {
//         dist[v] = dist[u] + 1;
//     }
//     dist
// }
