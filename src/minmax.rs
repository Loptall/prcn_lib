use cargo_snippet::snippet;

#[snippet("minmax")]
#[macro_export]
macro_rules! minimum {
    ($($e:expr),+) => {
        {
            let mut res = None;
            $(
                match res {
                    Some(v) => res = Some(std::cmp::min(v, $e)),
                    None => res = Some($e),
                }
            )+
            res.unwrap()
        }
    };
}

// #[test]
// fn mi() {
//     let a = 3;
//     let b = 7;
//     let c = -3;
//     let d = 0;

//     let minimum = minimum!(a, b, c, d);
//     assert!(minimum == -3);
// }

#[snippet("minmax")]
#[macro_export]
macro_rules! maximum {
    ($($e:expr),+) => {
        {
            let mut res = None;
            $(
                match res {
                    Some(v) => res = Some(std::cmp::max(v, $e)),
                    None => res = Some($e),
                }
            )+
            res.unwrap()
        }
    };
}

// #[test]
// fn ma() {
//     let a = 3;
//     let b = 7;
//     let c = -3;
//     let d = 0;

//     let maximum = maximum!(a, b, c, d);
//     assert!(maximum == 7);
// }

