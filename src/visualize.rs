use itertools::*;

pub trait Visualize {
    fn visualize(&self) -> String;
}

macro_rules! impl_vis_sized {
    ($t:ty) => {
        impl Visualize for $t {
            fn visualize(&self) -> String {
                self.to_string()
            }
        }
    };
}

impl_vis_sized!(usize);
impl_vis_sized!(u8);
impl_vis_sized!(u16);
impl_vis_sized!(u32);
impl_vis_sized!(u64);
impl_vis_sized!(u128);
impl_vis_sized!(isize);
impl_vis_sized!(i8);
impl_vis_sized!(i16);
impl_vis_sized!(i32);
impl_vis_sized!(i64);
impl_vis_sized!(i128);
impl_vis_sized!(char);
impl_vis_sized!(bool);
impl_vis_sized!(&str);

impl<T: Visualize> Visualize for [T] {
    fn visualize(&self) -> String {
        self.iter().map(|x| x.visualize()).join(" ")
    }
}

impl Visualize for Vec<char> {
    fn visualize(&self) -> String {
        self.iter().join("")
    }
}

impl Visualize for Vec<bool> {
    fn visualize(&self) -> String {
        self.iter().map(|x| if *x { "1" } else { "0" }).join("")
    }
}

impl Visualize for String {
    fn visualize(&self) -> String {
        self.clone()
    }
}

impl Visualize for Vec<Vec<char>> {
    fn visualize(&self) -> String {
        self.iter().map(|x| x.visualize()).join("\n")
    }
}

#[macro_export]
macro_rules! vis {
    ($e:expr) => {
        let mut res = format!("[{}:{}] {} =\n{}", file!(), line!(), stringify!($e), $e.visualize());
        eprintln!("{}", res);
    };
}
