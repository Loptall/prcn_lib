use std::collections::{HashSet, VecDeque};

type Idx2D = (usize, usize);

const N4: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
const N8: [(isize, isize); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

#[derive(Debug, Clone)]
pub struct Grid<'a> {
    h: usize,
    w: usize,
    board: &'a Vec<Vec<char>>,
    wall: HashSet<char>,
}

impl<'a> Grid<'a> {
    pub fn new(board: &'a Vec<Vec<char>>, wall: Vec<char>) -> Self {
        Self {
            h: board.len(),
            w: board[0].len(),
            board: board,
            wall: wall.into_iter().collect(),
        }
    }

    pub fn height(&self) -> usize {
        self.h
    }

    pub fn width(&self) -> usize {
        self.w
    }

    pub fn index(&self, idx: Idx2D) -> char {
        self.board[idx.0][idx.1]
    }

    pub fn is_passable(&self, idx: Idx2D) -> bool {
        !self.wall.contains(&self.index(idx))
    }

    pub fn neighbor4(&self, idx: Idx2D) -> impl Iterator<Item = Idx2D> {
        let mut res = Vec::new();
        for &(x, y) in N4.iter() {
            let idx = ((idx.0 as isize) + x, (idx.1 as isize) + y);
            if idx.0 < 0
                || idx.0 >= self.height() as isize
                || idx.1 < 0
                || idx.1 >= self.width() as isize
            {
                continue;
            }
            res.push((idx.0 as usize, idx.1 as usize));
        }
        res.into_iter()
    }

    pub fn neighbor8(&self, idx: Idx2D) -> impl Iterator<Item = Idx2D> {
        let mut res = Vec::new();
        for &(x, y) in N8.iter() {
            let idx = ((idx.0 as isize) + x, (idx.1 as isize) + y);
            if idx.0 < 0
                || idx.0 >= self.height() as isize
                || idx.1 < 0
                || idx.1 >= self.width() as isize
            {
                continue;
            }
            let idx = (idx.0 as usize, idx.1 as usize);
            if self.is_passable(idx) {
                res.push(idx);
            }
        }
        res.into_iter()
    }

    pub fn find(&self, c: char) -> Option<Idx2D> {
        for i in 0..self.height() {
            for j in 0..self.width() {
                if self.index((i, j)) == c {
                    return Some((i, j));
                }
            }
        }
        None
    }

    pub fn rfind(&self, c: char) -> Option<Idx2D> {
        for i in (0..self.height()).rev() {
            for j in (0..self.width()).rev() {
                if self.index((i, j)) == c {
                    return Some((i, j));
                }
            }
        }
        None
    }
}

pub struct Dfs<'a> {
    visited: Vec<Vec<bool>>,
    s: Vec<(Idx2D, Option<Idx2D>)>,
    g: &'a Grid<'a>,
}

impl<'a> Iterator for Dfs<'a> {
    type Item = (Idx2D, Idx2D);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((u, prev)) = self.s.pop() {
            if !self.g.is_passable(u) {
                return self.next();
            }

            for v in self.g.neighbor4(u) {
                if !self.visited[v.0][v.1] {
                    self.visited[v.0][v.1] = true;
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

pub fn dfs<'a>(g: &'a Grid, start: Idx2D) -> Dfs<'a> {
    let mut visited = vec![vec![false; g.height()]; g.width()];
    visited[start.0][start.1] = true;
    let mut s = Vec::new();
    s.push((start, None));
    Dfs { visited, s, g }
}

pub struct Bfs<'a> {
    visited: Vec<Vec<bool>>,
    q: VecDeque<(Idx2D, Option<Idx2D>)>,
    g: &'a Grid<'a>,
}

impl<'a> Iterator for Bfs<'a> {
    type Item = (Idx2D, Idx2D);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((u, prev)) = self.q.pop_front() {
            if !self.g.is_passable(u) {
                return self.next();
            }

            for v in self.g.neighbor4(u) {
                if !self.visited[v.0][v.1] {
                    self.visited[v.0][v.1] = true;
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

pub fn bfs<'a>(g: &'a Grid, start: Idx2D) -> Bfs<'a> {
    let mut visited = vec![vec![false; g.height()]; g.width()];
    visited[start.0][start.1] = true;
    let mut q = VecDeque::new();
    q.push_back((start, None));
    Bfs { visited, q, g }
}

#[test]
fn dfs_test() {
    let b = vec![
        vec!['.', '.', '#'],
        vec!['.', '.', '.'],
        vec!['#', '.', '.'],
    ];
    let g = Grid::new(&b, vec!['#']);
    for (a, b) in dfs(&g, (0, 0)) {
        dbg!(a, b);
    }
    // panic!()
}

#[test]
fn bfs_test() {
    let b = vec![
        vec!['.', '.', '#'],
        vec!['.', '.', '.'],
        vec!['#', '.', '.'],
    ];
    let g = Grid::new(&b, vec!['#']);
    for (a, b) in bfs(&g, (0, 0)) {
        dbg!(a, b);
    }
    // panic!()
}

pub fn is_joint(g: &Grid, from: Idx2D, to: Idx2D) -> bool {
    let mut used = vec![vec![false; g.height()]; g.width()];
    is_joint_inner(g, from, to, &mut used)
}

pub fn is_joint_inner(g: &Grid, from: Idx2D, to: Idx2D, used: &mut Vec<Vec<bool>>) -> bool {
    used[from.0][from.1] = true;
    dbg!(from);
    if from == to {
        return true;
    }
    let iter = g
        .neighbor4(from)
        .filter(|x| !used[x.0][x.1] && g.is_passable(*x))
        .collect::<Vec<Idx2D>>();
    for &(x, y) in iter.iter() {
        if is_joint_inner(g, (x, y), to, used) {
            return true;
        }
    }

    false
}

#[test]
fn grid_joint_test() {
    let b = vec![
        vec!['.', '.', '#'],
        vec!['#', '.', '#'],
        vec!['#', '.', '.'],
    ];
    let g = Grid::new(&b, vec!['#']);
    assert!(is_joint(&g, (0, 0), (2, 2)));

    let b = vec![
        vec!['.', '.', '#'],
        vec!['#', '#', '#'],
        vec!['#', '.', '.'],
    ];
    let g = Grid::new(&b, vec!['#']);
    assert!(!is_joint(&g, (0, 0), (2, 2)));
}

pub fn dist_table(g: &Grid, start: Idx2D) -> Vec<Vec<usize>> {
    let mut res = vec![vec![std::usize::MAX; g.height()]; g.width()];
    res[start.0][start.1] = 0;
    for (a, b) in bfs(&g, start) {
        res[b.0][b.1] = res[a.0][a.1] + 1;
    }

    res
}

#[test]
fn dist_test() {
    let b = vec![
        vec!['.', '.', '#'],
        vec!['.', '.', '.'],
        vec!['#', '.', '.'],
    ];
    let g = Grid::new(&b, vec!['#']);

    let d = dist_table(&g, (0, 0));
    assert_eq!(
        d,
        vec![
            vec![0, 1, std::usize::MAX],
            vec![1, 2, 3,],
            vec![std::usize::MAX, 3, 4]
        ]
    );
}
