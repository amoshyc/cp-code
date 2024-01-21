#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, q) = (inp[0], inp[1]);
    let val = readv::<usize>();
    let mut adj = vec![vec![]; n];
    let mut ask = vec![vec![]; n];
    for _ in 0..(n - 1) {
        let inp = readv::<usize>();
        let u = inp[0] - 1;
        let v = inp[1] - 1;
        adj[u].push(v);
        adj[v].push(u);
    }
    for qid in 0..q {
        let inp = readv::<usize>();
        let v = inp[0] - 1;
        let k = inp[1] - 1;
        ask[v].push((qid, k));
    }

    let mut ans = vec![0; q];
    dfs(0, n, &val, &mut ans, &mut adj, &mut ask);

    println!("{}", join(&ans, "\n"));
}

fn dfs(
    u: usize,
    p: usize,
    val: &Vec<usize>,
    ans: &mut Vec<usize>,
    adj: &mut Vec<Vec<usize>>,
    ask: &Vec<Vec<(usize, usize)>>,
) -> Vec<usize> {
    let mut cands: Vec<usize> = vec![val[u]];
    let neighbors = adj[u].clone();
    for v in neighbors {
        if v != p {
            cands.extend(dfs(v, u, val, ans, adj, ask).iter());
        }
    }
    cands.sort();
    cands.reverse();
    cands.truncate(20);

    for &(qid, k) in ask[u].iter() {
        ans[qid] = cands[k];
    }

    cands
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn readv<T: std::str::FromStr>() -> Vec<T> {
    read::<String>()
        .split_ascii_whitespace()
        .map(|t| t.parse().ok().unwrap())
        .collect()
}

fn reads() -> Vec<char> {
    read::<String>().chars().collect::<Vec<char>>()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
