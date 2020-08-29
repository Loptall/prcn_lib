use crate::graph::def::Graph;

pub struct Tree {
    graph: Vec<Vec<usize>>,
}

impl Tree {
    pub fn new(edges: &[(usize, usize)]) -> Self {
        let n = edges.len() + 1;
        let mut graph = vec![Vec::new(); n];
        for &(a, b) in edges {
            graph[a].push(b);
            graph[b].push(a);
        }

        Self { graph }
    }

    pub fn len(&self) -> usize {
        self.graph.len()
    }
}

impl<'a> Graph<'a> for Tree {
    type NodeId = usize;

    type Iter = std::iter::Cloned<std::slice::Iter<'a, Self::NodeId>>;

    fn len(&self) -> usize {
        self.graph.len()
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn index(&self, i: Self::NodeId) -> usize {
        i
    }

    fn neighbors(&'a self, i: Self::NodeId) -> Self::Iter {
        self.graph[i].iter().cloned()
    }
}

impl<'a> Tree {
    pub fn dfs(
        &'a self,
        cur: usize,
        in_order: &mut Vec<usize>,
        path: &mut Vec<usize>,
        depth: &mut Vec<usize>,
        d: usize,
        id: &mut usize,
        parent: usize,
    ) {
        in_order[cur] = *id;
        path.push(cur);
        depth.push(d);
        *id += 1;
        for &neighbor in self.graph[cur].iter() {
            if parent != neighbor {
                self.dfs(neighbor, in_order, path, depth, d + 1, id, cur);
                path.push(cur);
                depth.push(d);
                *id += 1;
            }
        }
    }

    // Vec<(index, depth)> in discovery time order
    pub fn euler_tour(&self, root: usize) -> (Vec<(usize, usize)>, Vec<usize>) {
        let mut in_order = vec![std::usize::MAX; self.len()];
        let mut path = Vec::new();
        let mut depth = Vec::new();
        let mut id = 0;

        self.dfs(
            root,
            &mut in_order,
            &mut path,
            &mut depth,
            0,
            &mut id,
            std::usize::MAX,
        );

        (
            depth.into_iter().enumerate().map(|(a, b)| (b, a)).collect(),
            in_order,
        )
    }
}
#[test]
fn euler_tour_test() {
    let tree = Tree::new(&[(0, 1), (0, 2), (1, 3), (3, 4), (2, 5), (5, 6), (5, 7)]);
    let dfs = tree.euler_tour(0);
    dbg!(dfs);
}
