use prcn_lib::graph::*;
use prcn_lib::prelude::*;

fn main() {
    input! {
        n: usize, m: usize, k: usize,
        friend: [(Usize1, Usize1); m],
        block: [(Usize1, Usize1); k],
    }

    // ans[i] = 人i の友達候補の数
    let mut ans = vec![0; n];

    let fg = make_undirected_graph(n, &friend);
    let bg = make_undirected_graph(n, &block);

    for i in 0..n {
        let distf = make_dist_table(&fg, i);
        let distb = make_dist_table(&bg, i);
        let ansi = (0..n)
            .filter(|x| distf[*x] >= 2 && distf[*x] != std::usize::MAX && distb[*x] != 1)
            .count();
        ans[i] = ansi;
    }

    for i in ans {
        print!("{} ", i);
    }
    println!();
}

// Didn't submitted
