use cargo_snippet;
use nalgebra::{Dynamic, MatrixMN};
use num::Num;

type Mat<N: Num> = nalgebra::MatrixMN<N, Dynamic, Dynamic>;
type Vector<N: Num> = nalgebra::VectorN<N, Dynamic>;


pub fn solve_linear_equations<N>(row: usize, colum: usize, cs: &[N]) -> Option<Vector<N>> {
    

    None
}
