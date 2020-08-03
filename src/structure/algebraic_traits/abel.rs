use cargo_snippet::snippet;

use super::monoid::Monoid;

/// 単位元が定義される `T -> T -> T`型の演算
///
/// Abel は Monoid である
#[snippet("abel")]
pub trait Abel: Sized + Monoid {
    /// 逆元
    fn inverse(x: &Self, y: &Self) -> Self;
}

/// Abelトレイトの自動実装マクロ
///
/// 一行目にアーベル名、
/// 二行目に単位元
/// 三行目に2引数を取って、`x + y` となる同型の演算結果を返すクロージャ、
/// 四行目に2引数を取って、`x - y` となる同型の演算結果を返すクロージャ、
///
/// ```example
/// abel_def! {
///     Add<usize>,
///     0,
///     |x: usize, y: usize| x + y,
///     |x: usize, y: usize| x - y
/// }
/// ```
///
#[snippet("abel")]
#[macro_export]
macro_rules! abel_def {
    {
        $M:ident<$t:ty>,
        $id:expr,
        $op:expr,
        $inv:expr
    } => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct $M($t);

        impl Monoid for $M {
            fn identity() -> Self {
                $M($id)
            }

            fn op(x: &Self, y: &Self) -> Self {
                let f = $op;
                $M(f(x.0, y.0))
            }
        }

        impl Abel for $M {
            fn inverse(x: &Self, y: &Self) -> Self {
                let f = $inv;
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

#[test]
fn able_test() {
    abel_def! {
        Add<usize>,
        0,
        |x, y| x + y,
        |x, y| x - y
    }

    let a = Add(5);
    let b = Add(3);

    assert_eq!(Add::op(&a, &b).0, 8);
    assert_eq!(Add::inverse(&a, &b).0, 2);
}
