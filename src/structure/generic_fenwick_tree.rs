use super::algebraic_traits::abel::Abel;

#[derive(Debug, Clone)]
pub struct FenwickTree<T: Abel> {
    len: usize,
    segment: Vec<T>,
}

impl<T: Abel + Clone> FenwickTree<T> {
    pub fn new(n: usize) -> Self {
        Self {
            len: n,
            segment: vec![T::identity(); n + 1],
        }
    }

    pub fn from<I: Into<T> + Copy>(a: &[I]) -> Self {
        let n = a.len();
        let mut f = Self::new(n);
        for (i, &v) in a.iter().enumerate() {
            f.merge(i, v.into());
        }
        f
    }

    pub fn len(&self) -> usize {
        self.len
    }

    /// クエリ: 要素`i` に `v` をマージする
    pub fn merge(&mut self, i: usize, v: T) {
        let mut i = i + 1;
        while i <= self.len() {
            self.segment[i] = T::op(&self.segment[i].clone(), &v.clone());
            i += (i as i64 & -(i as i64)) as usize;
        }
    }

    pub fn update(&mut self, i: usize, v: T) {
        let inv = T::inverse(&v, &self.segment[i].clone());
        self.merge(i, inv);
    }

    pub fn accumulate(&self, mut i: usize) -> T {
        let mut s = T::identity();
        while i > 0 {
            s = T::op(&s.clone(), &self.segment[i].clone());
            i -= (i as i64 & -(i as i64)) as usize;
        }
        s
    }

    pub fn range(&self, from: usize, to: usize) -> T {
        T::inverse(&self.accumulate(to), &self.accumulate(from))
    }
}

#[test]
fn generic_fenwick() {
    use super::algebraic_traits::monoid::Monoid;
    #[macro_export]
    macro_rules ! abe_def {{$ M : ident <$ t : ty >, $ id : expr , $ op : expr , $ inv : expr } => {# [derive (Debug , Clone , Copy , PartialEq ) ] pub struct $ M ($ t ) ; impl Monoid for $ M {fn identity () -> Self {$ M ($ id ) } fn op (x : & Self , y : & Self ) -> Self {let f = $ op ; $ M (f (x . 0 , y . 0 ) ) } } impl Abel for $ M {fn inverse (x : & Self , y : & Self ) -> Self {let f = $ inv ; $ M (f (x . 0 , y . 0 ) ) } } impl Into <$ M > for $ t {fn into (self ) -> $ M {$ M (self ) } } } ; }

    abe_def! {
        Add<i64>,
        0,
        |x, y| x + y,
        |x, y| x - y
    }

    let v = vec![1, 2, 3, 4];
    let mut ft = FenwickTree::<Add>::from(&v);

    dbg!(&ft);

    assert_eq!(ft.range(0, 2).0, 3);
    assert_eq!(ft.range(1, 4).0, 9);

    ft.update(2, Add(0));

    assert_eq!(ft.range(0, 4).0, 7);
}
