#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, r, m) = (inp[0], inp[1], inp[2]);

    let mut nxt = vec![0; m];
    for x in 0..m {
        nxt[x] = x * x % m;
    }

    let (prefix, cycle) = walk_on_functional_graph(&nxt, r);
    let prefix = mapv(&prefix, |&x| x as i64);
    let cycle = mapv(&cycle, |&x| x as i64);
    if n <= prefix.len() {
        println!("{}", prefix[..n].iter().sum::<i64>());
    } else {
        let num_cycle = (n - prefix.len()) / cycle.len();
        let remaining = (n - prefix.len()) % cycle.len();
        let mut ans = 0;
        ans += prefix.iter().sum::<i64>();
        ans += (num_cycle as i64) * cycle.iter().sum::<i64>();
        ans += cycle[..remaining].iter().sum::<i64>();
        println!("{}", ans);
    }
}

fn walk_on_functional_graph(nxt: &Vec<usize>, src: usize) -> (Vec<usize>, Vec<usize>) {
    let mut idx = vec![!0; nxt.len()];

    idx[src] = 0;
    let mut path = vec![src];
    let mut u = nxt[src];
    while idx[u] == !0 {
        idx[u] = path.len();
        path.push(u);
        u = nxt[u];
    }

    let prefix = path[..idx[u]].to_vec(); // will be empty in permutation graph
    let cycle = path[idx[u]..].to_vec();
    (prefix, cycle)
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
    read::<String>().chars().collect()
}

fn mapv<T, S, F: Fn(&T) -> S>(arr: &Vec<T>, f: F) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
