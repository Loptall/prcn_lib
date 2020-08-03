use cargo_snippet::snippet;

/// 単位元が定義される `T -> T -> T`型の演算
#[snippet("monoid")]
pub trait Monoid: Sized {
    fn identity() -> Self;

    fn op(x: &Self, y: &Self) -> Self;

    fn fold(v: &[Self]) -> Self {
        v.iter().fold(Self::identity(), |a, b| Self::op(&a, b))
    }
}

/// Monoidトレイトの自動実装マクロ
///
/// 一行目にモノイド名、
/// 二行目に単位元
/// 三行目に`2引数を取って、同型の演算結果を返すクロージャを渡す
///
/// ```example
/// monoid_def! {
///     Max<usize>,
///     std::usize::MIN,
///     |x: usize, y: usize| x.max(y)
/// }
/// ```
///
#[snippet("monoid")]
#[macro_export]
macro_rules! monoid_def {
    {
        $M:ident<$t:ty>,
        $id:expr,
        $me:expr
    } => {
        #[derive(Debug, Clone, Copy)]
        pub struct $M($t);

        impl Monoid for $M {
            fn identity() -> Self {
                $M($id)
            }

            fn op(x: &Self, y: &Self) -> Self {
                let f = $me;
                $M(f(x.0, y.0))
            }
        }

        impl Into<$M> for $t {
            fn into(self) -> $M {
                $M(self)
            }
        }
    };
}

// monoid_def! {
//     Max<usize>,
//     std::usize::MIN,
//     |a: usize, b: usize| a.max(b)
// }
