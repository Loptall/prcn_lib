use crate::graph::graph::Graph;
use std::cmp::{min, Reverse};
use std::collections::{BinaryHeap, HashMap};

/// 単純グラフじゃないと死ぬ
pub struct Dijkstra {
    // グラフの骨格部分
    g: Vec<Vec<usize>>,
    // 辺を(頂点, 頂点)の形で特定してその重みを持つ
    e: HashMap<(usize, usize), usize>,
}

impl<'a> Graph<'a> for Dijkstra {
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

pub fn make_graph_for_dijkstra(n: usize, edges: &[(usize, usize, usize)]) -> Dijkstra {
    let mut g = vec![Vec::new(); n];
    let mut e = HashMap::new();
    for (from, to, weight) in edges.iter() {
        g[*from].push(*to);
        g[*to].push(*from);
        e.insert((*from, *to), *weight);
        e.insert((*to, *from), *weight);
    }

    Dijkstra { g, e }
}

impl<'a> Dijkstra {
    pub fn weight(&self, from: usize, to: usize) -> usize {
        self.e[&(from, to)]
    }

    pub fn add_edge(&mut self, edge: (usize, usize, usize)) {
        self.g[edge.0].push(edge.1);
        self.g[edge.1].push(edge.0);
        self.e.insert((edge.0, edge.1), edge.2);
        self.e.insert((edge.1, edge.0), edge.2);
    }
}

pub fn dijkstra<'a>(g: &'a Dijkstra, start: usize, goal: usize) -> usize {
    // 初期化
    let mut d = Vec::with_capacity(g.len()); // `start`からの最短距離
                                             // 左にコスト、右にインデックス
    let mut q: BinaryHeap<Reverse<(usize, usize)>> = BinaryHeap::with_capacity(g.len()); // まだ確定していない頂点の集合
    for i in 0..g.len() {
        let j = if i == start { 0 } else { std::usize::MAX };
        d.push(j);
        q.push(Reverse((j, i)));
    }
    // let mut p: Vec<Option<usize>> = vec![None; g.len()]; // 各点までの最短距離の経路管理

    while !q.is_empty() {
        let u = q.pop().unwrap().0;
        for v in g.neighbors(u.1) {
            let w = g.weight(u.1, v);
            if d[v] > d[u.1] + w {
                d[v] = d[u.1] + w;
                // p[v] = Some(u.0);
                q.push(Reverse((w, v)));
            }
        }
    }

    d[goal]
}

pub fn dijkstra_with_path<'a>(g: &'a Dijkstra, start: usize, goal: usize) -> (Vec<usize>, usize) {
    // 初期化
    let mut d = Vec::with_capacity(g.len()); // `start`からの最短距離
                                             // 左にコスト、右にインデックス
    let mut q: BinaryHeap<Reverse<(usize, usize)>> = BinaryHeap::with_capacity(g.len()); // まだ確定していない頂点の集合
    for i in 0..g.len() {
        let j = if i == start { 0 } else { std::usize::MAX };
        d.push(j);
        q.push(Reverse((j, i)));
    }
    let mut p: Vec<Option<usize>> = vec![None; g.len()]; // 各点までの最短距離の経路管理

    while !q.is_empty() {
        let u = q.pop().unwrap().0;
        for v in g.neighbors(u.1) {
            let w = g.weight(u.1, v);
            if d[v] > d[u.1] + w {
                d[v] = d[u.1] + w;
                p[v] = Some(u.1);
                q.push(Reverse((w, v)));
            }
        }
    }

    let mut path = vec![goal];
    loop {
        match p[*path.last().unwrap()] {
            Some(prev) if prev == start => {
                path.push(prev);
                break;
            }
            Some(prev) => {
                path.push(prev);
            }
            None => break,
        }
    }

    path.reverse();

    (path, d[goal])
}

#[test]
fn dijkstra_test() {
    let mut d = make_graph_for_dijkstra(4, &[(0, 1, 2), (0, 2, 100), (1, 3, 10), (2, 3, 100)]);

    assert_eq!(dijkstra(&d, 0, 3), 12);

    d.add_edge((0, 3, 1));

    assert_eq!(dijkstra(&d, 0, 3), 1);

    let p = dijkstra_with_path(&d, 0, 3).0;
    assert_eq!(p, vec![0, 3]);
}
