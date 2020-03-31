use proconio::{input, marker::Usize1};
fn main() {
    input! {
        n: Usize1
    }

    let mut a = vec![1];
    let mut ruieskiwa = Vec::new();
    let mut ruieskiwanoruisekiwa = Vec::new();
    for _i in 0..n + 1 {
        ruieskiwa.push(a.iter().sum::<usize>());
        ruieskiwanoruisekiwa.push(ruieskiwa.iter().sum::<usize>());
        a.push(*ruieskiwanoruisekiwa.last().unwrap());
    }

    println!("{}", a[n]);
}
