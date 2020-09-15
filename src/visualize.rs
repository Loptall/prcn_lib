use cargo_snippet::snippet;

#[snippet(name = "visualize", prefix = "pub use visualize::*;")]
mod visualize {
    use itertools::*;
    pub trait Visualize {
        fn visualize(&self, split: &str);
        fn continuous(&self) {
            self.visualize("");
        }
        fn spaces(&self) {
            self.visualize(" ");
        }
        fn lines(&self) {
            self.visualize("\n");
        }
    }

    macro_rules! impl_vis_for_sized {
        ($($t:ty),+) => {
            $(
                impl Visualize for $t {
                    fn visualize(&self, _split: &str) {
                        print!("{}", self);
                    }
                }
            )+
        };
    }

    impl_vis_for_sized! {
        usize, u8, u16, u32, u64, u128,
        isize, i8, i16, i32, i64, i128,
        String, &str, char
    }

    impl<T: std::fmt::Display> Visualize for [T] {
        fn visualize(&self, split: &str) {
            print!("{}", self.iter().join(split));
        }
    }

    #[test]
    fn fmt() {
        let a = 2;
        a.visualize("");

        let b = "str";
        b.visualize("");

        let c = vec![1, 2, 3];
        c.visualize(" ");
        c.visualize("\n");

        // panic!();
    }

    /// macro to output answer improved to println
    ///
    /// this macro will print '\n' at the last.
    ///
    /// # Patterns
    ///
    /// `,`, it means one space, it formats the elements, which is before, space-split.
    /// And insert a space after the prefix, unless it is at last.
    ///
    /// `=>`, it means empty string, it formats the element, witch is before, without any split.
    /// And insert nothing between two element.
    ///
    /// `;`, it means newline(\n), it formats the element, witch is before, newline-split.
    /// And insert a newline after the prefix, unless it is at last.
    ///
    /// # Eample
    ///
    /// ```
    /// #[test]
    /// fn vis_test() {
    ///     let a = 1;
    ///     let b = 2;
    ///     let c = 3;
    ///
    ///     let v = vec!['a', 'b', 'c'];
    ///
    ///     // single element test ...
    ///
    ///     vis!(a); // 1\n
    ///     vis!(a,); // 1\n
    ///     vis!(a =>); // 1\n
    ///     vis!(a;); // 1\n
    ///
    ///     println!();
    ///
    ///     vis!(a + b); // 3\n
    ///
    ///     println!();
    ///
    ///     vis!(v); // a b c\n
    ///     vis!(v,); // a b c\n
    ///     vis!(v =>); // abc\n
    ///     vis!(v;); // a\nb\nc\n
    ///
    ///     println!();
    ///
    ///     // multi elements connected by common operater test ...
    ///
    ///     println!();
    ///
    ///     vis!(a, b); // 1 2\n
    ///     vis!(a, b, c); // 1 2 3\n
    ///     vis!(a, b, c;); // 1 2 3\n
    ///
    ///     println!();
    ///
    ///     vis!(a => b); // 12\n
    ///     vis!(a => b => c); // 123\n
    ///     vis!(a => b => c ,); // 123\n
    ///
    ///     println!();
    ///
    ///     vis!(a; b); // 1\n2\n
    ///     vis!(a; b; c); // 1\n2\n3\n
    ///     vis!(a; b; c =>); // 1\n2\n3\n
    ///
    ///     println!();
    ///
    ///     // multi elements connected by different operater test ...
    ///
    ///     vis!(a, b; c); // 1 2\n3\n
    ///     vis!(a; b, c); // 1\n2 3\n
    ///     vis!(a => b, c); // 12 3\n
    ///     vis!(a; b => c); // 1\n23\n
    ///
    ///     println!();
    ///
    ///     vis!(a, v); // 1 a b c\n;
    ///     vis!(a, v;); // 1 a\nb\nc\n;
    ///     vis!(a => v =>); // 1abc\n;
    ///
    ///     println!("\\d");
    ///     // panic!()
    /// }
    /// ```
    #[macro_export]
    macro_rules! vis {
        // end
        () => {
            println!();
        };

        // last element + trailing pattern
        ($last:expr ;) => {
            $last.lines();
            vis!()
        };
        ($last:expr =>) => {
            $last.continuous();
            vis!();
        };
        ($last:expr $(,)?) => {
            $last.spaces();
            vis!();
        };

        // get first element and pass rest
        ($first:expr; $($rest:tt)*) => {
            $first.lines();
            println!();
            vis!($($rest)*);
        };
        ($first:expr => $($rest:tt)*) => {
            $first.continuous();
            vis!($($rest)*);
        };
        ($first:expr, $($rest:tt)*) => {
            $first.spaces();
            print!(" ");
            vis!($($rest)*);
        };
    }

    #[test]
    fn vis_test() {
        let a = 1;
        let b = 2;
        let c = 3;

        let v = vec!['a', 'b', 'c'];

        // empty

        vis!(); // \n

        // single element test ...

        vis!(a); // 1\n
        vis!(a,); // 1\n
        vis!(a =>); // 1\n
        vis!(a;); // 1\n

        println!();

        vis!(a + b); // 3\n

        println!();

        vis!(v); // a b c\n
        vis!(v,); // a b c\n
        vis!(v =>); // abc\n
        vis!(v;); // a\nb\nc\n

        println!();

        // multi elements connected by common operater test ...

        println!();

        vis!(a, b); // 1 2\n
        vis!(a, b, c); // 1 2 3\n
        vis!(a, b, c;); // 1 2 3\n

        println!();

        vis!(a => b); // 12\n
        vis!(a => b => c); // 123\n
        vis!(a => b => c ,); // 123\n

        println!();

        vis!(a; b); // 1\n2\n
        vis!(a; b; c); // 1\n2\n3\n
        vis!(a; b; c =>); // 1\n2\n3\n

        println!();

        // multi elements connected by different operater test ...

        vis!(a, b; c); // 1 2\n3\n
        vis!(a; b, c); // 1\n2 3\n
        vis!(a => b, c); // 12 3\n
        vis!(a; b => c); // 1\n23\n

        println!();

        vis!(a, v); // 1 a b c\n;
        vis!(a, v;); // 1 a\nb\nc\n;
        vis!(a => v =>); // 1abc\n;

        println!("\\d");
        // panic!()
    }
}
