use num::{Bounded, Integer, One, Zero};
use std::ops::{Add, BitAnd, BitOr, BitXor, Mul, Sub};

/// 単位元が定義される `T -> T -> T`型の演算
pub trait Monoid: Sized {
    fn identity() -> Self;

    fn op(x: &Self, y: &Self) -> Self;

    fn fold(v: &[Self]) -> Self {
        v.iter().fold(Self::identity(), |a, b| Self::op(&a, b))
    }
}

/// 一行目にモノイド名、
/// 二行目に単位元
/// 三行目に`x`と`y`を引数に取って、同じ型の演算結果を返すクロージャを渡す
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
    };
}

/// 区間和
#[derive(Clone, Copy, Debug)]
pub struct Sum<T: Clone + Copy>(T);

impl<T: Clone + Copy + Zero + Add<Output = T>> Monoid for Sum<T> {
    fn identity() -> Self {
        Self(T::zero())
    }
    fn op(x: &Self, y: &Self) -> Self {
        Self(x.0.clone() + y.0.clone())
    }
}

impl<T: Copy> From<T> for Sum<T> {
    fn from(x: T) -> Self {
        Sum(x)
    }
}

/// 区間積
#[derive(Clone, Copy, Debug)]
pub struct Product<T>(T);

impl<T: Clone + Copy + One + Mul<Output = T>> Monoid for Product<T> {
    fn identity() -> Self {
        Self(T::one())
    }
    fn op(x: &Self, y: &Self) -> Self {
        Self(x.0.clone() * y.0.clone())
    }
}

impl<T> From<T> for Product<T> {
    fn from(x: T) -> Self {
        Product(x)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Max<T>(T);

impl<T: Copy + Clone + Bounded + Ord> Monoid for Max<T> {
    fn identity() -> Self {
        Max(T::min_value())
    }
    fn op(x: &Self, y: &Self) -> Self {
        Max(x.0.max(y.0))
    }
}

impl<T> From<T> for Max<T> {
    fn from(x: T) -> Self {
        Max(x)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Min<T>(T);

impl<T: Copy + Clone + Bounded + Ord> Monoid for Min<T> {
    fn identity() -> Self {
        Min(T::max_value())
    }
    fn op(x: &Self, y: &Self) -> Self {
        Min(x.0.min(y.0))
    }
}

impl<T> From<T> for Min<T> {
    fn from(x: T) -> Self {
        Min(x)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Xor<T>(T);

impl<T: Copy + Clone + BitXor<Output = T> + Zero> Monoid for Xor<T> {
    fn identity() -> Self {
        Xor(T::zero())
    }

    fn op(x: &Self, y: &Self) -> Self {
        Xor(x.0 ^ y.0)
    }
}

impl<T> From<T> for Xor<T> {
    fn from(x: T) -> Self {
        Xor(x)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct And<T>(T);

impl<T> Monoid for And<T>
where
    T: Copy + Clone + BitAnd<Output = T> + Bounded + Sub<Output = T> + One,
{
    fn identity() -> Self {
        And(T::max_value() - T::one())
    }

    fn op(x: &Self, y: &Self) -> Self {
        And(x.0 & y.0)
    }
}

impl<T> From<T> for And<T> {
    fn from(x: T) -> Self {
        And(x)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Or<T>(T);

impl<T: Copy + Clone + BitOr<Output = T> + Zero> Monoid for Or<T> {
    fn identity() -> Self {
        Or(T::zero())
    }

    fn op(x: &Self, y: &Self) -> Self {
        Or(x.0 | y.0)
    }
}

impl<T> From<T> for Or<T> {
    fn from(x: T) -> Self {
        Or(x)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Gcd<T: Integer>(T);

impl<T: Integer> Monoid for Gcd<T> {
    fn identity() -> Self {
        Gcd(T::zero())
    }

    fn op(x: &Self, y: &Self) -> Self {
        Gcd(x.0.gcd(&y.0))
    }
}

impl<T: Integer> From<T> for Gcd<T> {
    fn from(x: T) -> Self {
        Gcd(x)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Lcm<T: Integer>(T);

impl<T: Integer> Monoid for Lcm<T> {
    fn identity() -> Self {
        Lcm(T::one())
    }

    fn op(x: &Self, y: &Self) -> Self {
        Lcm(x.0.lcm(&y.0))
    }
}

impl<T: Integer> From<T> for Lcm<T> {
    fn from(x: T) -> Self {
        Lcm(x)
    }
}

monoid_def! {
    BoolAnd<bool>,
    true,
    |x, y| x && y
}

monoid_def! {
    BoolOr<bool>,
    false,
    |x, y| x || y
}

monoid_def! {
    BoolXor<bool>,
    false,
    |x, y| !(x && y) || !(!x && !y)
}
