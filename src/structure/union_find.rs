use std::collections::HashSet;

use cargo_snippet::snippet;

#[snippet("union_find")]
#[derive(Debug, Clone)]
pub struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

#[snippet("snippet")]
impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect::<Vec<usize>>(),
            size: vec![1; n],
        }
    }

    pub fn len(&self) -> usize {
        self.parent.len()
    }

    /// i が属する集合の親のインデックス
    pub fn find(&mut self, mut i: usize) -> usize {
        while self.parent[i] != i {
            self.parent[i] = self.find(self.parent[i]);
            i = self.parent[self.parent[i]];
        }
        i
    }

    /// a と b を繋ぐ
    pub fn unite(&mut self, a: usize, b: usize) -> bool {
        let mut a = self.find(a);
        let mut b = self.find(b);
        if a == b {
            return false;
        }
        if self.size[a] < self.size[b] {
            std::mem::swap(&mut a, &mut b);
        }
        self.size[a] += self.size[b];
        self.parent[b] = a;
        true
    }

    /// i が属する集合の要素数
    pub fn count(&mut self, i: usize) -> usize {
        let p = self.find(i);
        self.size[p]
    }

    /// a と b が同一集合に属するか
    pub fn joint(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }

    /// i が属する集合の要素を列挙する
    pub fn get_group(&mut self, i: usize) -> HashSet<usize> {
        let p = self.find(i);
        (0..self.len()).filter(|x| self.find(*x) == p).collect()
    }
}

#[test]
fn find_test() {
    let mut uf = UnionFind::new(10);

    assert_eq!(uf.find(2), 2);

    uf.unite(1, 2);
    assert_eq!(uf.find(2), 1);

    uf.unite(2, 5);
    assert_eq!(uf.find(5), 1);
    assert_eq!(uf.count(5), 3);
}

#[test]
fn group_test() {
    let mut uf = UnionFind::new(5);
    uf.unite(0, 2);
    uf.unite(2, 3);
    uf.unite(1, 4);

    assert_eq!(uf.get_group(0), vec![0, 2, 3].into_iter().collect::<HashSet<_>>());
    assert_eq!(uf.get_group(4), vec![1, 4].into_iter().collect::<HashSet<_>>());
}
