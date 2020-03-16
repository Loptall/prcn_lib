
fn dfs(g: &[Vec<usize>], now: usize, seen: &mut Vec<bool>) -> Vec<bool> {
    seen[now] = true;

    for i in &g[now] {
        if seen[*i] { continue; }
        dfs(g, *i, seen);
    }

    seen.to_vec()
}
