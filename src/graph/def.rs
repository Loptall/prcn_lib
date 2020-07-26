use cargo_snippet::snippet;

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
pub type UnweightedGraph = Vec<Vec<usize>>;

#[snippet("graph")]
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
#[snippet("graph")]
pub type WeightedNodeGraph<W> = Vec<Vec<(usize, W)>>;

#[snippet("graph")]
impl<'a, W> Graph<'a> for WeightedNodeGraph<W>
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
