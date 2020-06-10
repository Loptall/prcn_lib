use crate::graph::{bfs::bfs, dfs::dfs, graph::Graph};

/// 重みなしグラフにおいて、幅優先探索を用いて任意の頂点から、各頂点への最短距離を求める
///
/// dist[i] = (startからiまでの最短距離)
pub fn dist_table<'a, G>(g: &'a G, start: G::NodeId) -> Vec<usize>
where
    G: Graph<'a, NodeId = usize>,
{
    let mut dist = vec![std::usize::MAX; g.len()];
    dist[start] = 0;
    for (f, t) in bfs(g, start) {
        dist[t] = dist[f] + 1;
    }
    dist
}

/// 重みなしグラフでの2頂点の最短距離を幅優先探索で求める
///
/// goalまでの経路が見つかった時点で探索を打ち切るので、
/// グラフのサイズの大きさに直接の影響を受けないことが期待される
pub fn shortest_path<'a, G: Graph<'a, NodeId = usize>>(
    g: &'a G,
    start: usize,
    goal: usize,
) -> usize {
    let mut dist = vec![std::usize::MAX; g.len()];
    dist[start] = 0;
    for (f, t) in bfs(g, start) {
        if t == goal {
            return dist[f] + 1;
        }
        dist[t] = dist[f] + 1;
    }

    std::usize::MAX
}

/// グラフを連結成分ごとに分解する。
/// 返り値`r`は、`r[c][i] = 頂点iはグループcに属する`
pub fn classify_into_connected_group<'a, G: Graph<'a, NodeId = usize>>(
    g: &'a G,
) -> Vec<Vec<usize>> {
    let mut classified = vec![None; g.len()];
    let mut cur = 0usize;
    for u in 0..g.len() {
        match classified[u] {
            Some(_) => {}
            None => {
                for (i, _) in dist_table(g, u)
                    .iter()
                    .enumerate()
                    .filter(|x| *x.1 != std::usize::MAX)
                {
                    classified[i] = Some(cur);
                }
                cur += 1;
            }
        }
    }

    let mut res = vec![Vec::new(); cur];
    for (i, c) in classified.into_iter().map(|x| x.unwrap()).enumerate() {
        res[c].push(i);
    }

    res
}
