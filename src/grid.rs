//! Varified

use crate::idx::Idx2D;
use std::convert::TryInto;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
pub struct Grid<T>(pub Vec<Vec<T>>);

impl<T> Grid<T> {
    pub fn new(b: Vec<Vec<T>>) -> Self {
        Self(b)
    }

    pub fn width(&self) -> usize {
        self.0[0].len()
    }

    pub fn height(&self) -> usize {
        self.0.len()
    }

    pub fn idx(&self, x: usize, y: usize) -> Idx2D {
        Idx2D::new(x, y, self.width(), self.height())
    }

    pub fn get<Ix: TryInto<isize>>(&self, x: Ix, y: Ix) -> Option<&T> {
        let x = x.try_into().ok()?;
        let y = y.try_into().ok()?;
        let w = self.width() as isize;
        let h = self.height() as isize;
        if x >= 0 && y >= 0 && x < w && y < h {
            Some(&self.0[y as usize][x as usize])
        } else {
            None
        }
    }

    pub fn get_mut<Ix: TryInto<isize>>(&mut self, x: Ix, y: Ix) -> Option<&mut T> {
        let x = x.try_into().ok()?;
        let y = y.try_into().ok()?;
        let w = self.width() as isize;
        let h = self.height() as isize;
        if x >= 0 && y >= 0 && x < w && y < h {
            Some(&mut self.0[y as usize][x as usize])
        } else {
            None
        }
    }
}

impl<T: PartialEq> Grid<T> {
    pub fn find(&self, v: T) -> Option<Idx2D> {
        for i in 0..self.height() {
            for j in 0..self.width() {
                if *self.get(j, i).unwrap() == v {
                    return Some(self.idx(i, j));
                }
            }
        }
        None
    }
}

impl<T> Index<Idx2D> for Grid<T> {
    type Output = T;

    fn index(&self, ix: Idx2D) -> &Self::Output {
        &self.0[ix.y][ix.x]
    }
}

impl<T> IndexMut<Idx2D> for Grid<T> {
    fn index_mut(&mut self, ix: Idx2D) -> &mut Self::Output {
        &mut self.0[ix.y][ix.x]
    }
}
