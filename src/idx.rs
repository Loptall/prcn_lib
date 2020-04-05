//! Varified

use std::convert::TryInto;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Idx2D {
    pub x: usize,
    pub y: usize,
    h: usize,
    w: usize,
}

impl Idx2D {
    pub fn new<T: TryInto<isize>>(x: T, y: T, w: usize, h: usize) -> Self {
        let x = x.try_into().ok().unwrap();
        let y = y.try_into().ok().unwrap();

        assert!(x >= 0 && y >= 0);
        assert!(x < w as isize && y < h as isize);

        Idx2D {
            x: x as usize,
            y: y as usize,
            w,
            h,
        }
    }

    pub fn try_new<T: TryInto<isize>>(x: T, y: T, w: usize, h: usize) -> Option<Self> {
        let x = x.try_into().ok()?;
        let y = y.try_into().ok()?;

        if x >= 0 && y >= 0 && x < w as isize && y < h as isize {
            Some(Self {
                x: x as usize,
                y: y as usize,
                w,
                h,
            })
        } else {
            None
        }
    }

    pub fn try_add<T: TryInto<isize>>(&self, rhs: (T, T)) -> Option<Self> {
        let dx = rhs.0.try_into().ok().unwrap();
        let dy = rhs.1.try_into().ok().unwrap();
        let x = self.x as isize + dx;
        let y = self.y as isize + dy;

        Self::try_new(x, y, self.w, self.h)
    }

    pub fn try_sub<T: TryInto<isize>>(&self, rhs: (T, T)) -> Option<Self> {
        let dx = rhs.0.try_into().ok().unwrap();
        let dy = rhs.1.try_into().ok().unwrap();
        let x = self.x as isize - dx;
        let y = self.y as isize - dy;

        Self::try_new(x, y, self.w, self.h)
    }

    pub fn neighber4(&self) -> impl Iterator<Item = Idx2D> {
        const VECT: &[(isize, isize)] = &[(-1, 0), (0, -1), (1, 0), (0, 1)];
        let iter = *self;
        VECT.iter().filter_map(move |u| iter.try_add(u.clone()))
    }

    pub fn neighber8(&self) -> impl Iterator<Item = Idx2D> {
        const VECT: &[(isize, isize)] = &[
            (-1, 0),
            (0, -1),
            (1, 0),
            (0, 1),
            (-1, -1),
            (1, -1),
            (-1, 1),
            (1, 1),
        ];
        let iter = *self;
        VECT.iter().filter_map(move |u| iter.try_add(u.clone()))
    }
}
