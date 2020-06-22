use crate::graph::def::Graph;

use std::collections::VecDeque;

use cargo_snippet::snippet;

#[snippet("bfs")]
pub struct Bfs<'a, G: Graph<'a>> {
    visited: Vec<bool>,
    q: VecDeque<(G::NodeId, Option<G::NodeId>)>,
    g: &'a G,
}

#[snippet("bfs")]
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

/// `start`の頂点からの幅優先探索時の、
/// 頂点から頂点の接続をイテレートするIteratorを作る
///
/// ```ignore
/// let e = &[(0, 2), (0, 1), (1, 2), (2, 3), (3, 4), (3, 5), (1, 5)];
///
/// let g = make_undirected_graph(6, e);
///
/// for (f, t) in bfs(&g, 0) {
///     println!("{} -> {}", f, t);
/// }
/// ```
///
/// これは
///
/// `0 -> 2`
///
/// `0 -> 1`
///
/// `2 -> 3`
///
/// `1 -> 5`
///
/// `3 -> 4`
///
/// を表示する
#[snippet("bfs")]
pub fn bfs<'a, G>(g: &'a G, start: G::NodeId) -> Bfs<'a, G>
where
    G: Graph<'a, NodeId = usize>,
{
    let n = g.len();
    let mut visited = vec![false; n];
    let mut q = VecDeque::new();
    visited[start] = true;
    q.push_back((start, None));

    Bfs { visited, q, g }
}
