use crate::graph::def::Graph;

use cargo_snippet::snippet;

#[snippet("dfs")]
pub struct Dfs<'a, G: Graph<'a>> {
    visited: Vec<bool>,
    s: Vec<(G::NodeId, Option<G::NodeId>)>,
    g: &'a G,
}

#[snippet("dfs")]
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

/// `start`からの深さ優先探索時の、
/// 頂点から頂点の接続をイテレートするIteratorを作る
///
/// ```rust
/// use sfcpl::graph::{util::make_undirected_graph, dfs::dfs};
///
/// let e = &[(0, 2), (0, 1), (1, 2), (2, 3), (3, 4), (3, 5), (1, 5)];
///
/// let g = make_undirected_graph(6, e);
///
/// for (f, t) in dfs(&g, 0) {
///     println!("{} -> {}", f, t);
/// }
/// ```
///
/// これは
///
/// `0 -> 1`
///
/// `1 -> 5`
///
/// `5 -> 3`
///
/// `3 -> 4`
///
/// `0 -> 2`
///
/// を表示する
#[snippet("dfs")]
pub fn dfs<'a, G: Graph<'a, NodeId = usize>>(g: &'a G, start: G::NodeId) -> Dfs<'a, G> {
    let n = g.len();
    let mut visited = vec![false; n];
    let mut s = Vec::new();
    visited[start] = true;
    s.push((start, None));

    Dfs { visited, s, g }
}
