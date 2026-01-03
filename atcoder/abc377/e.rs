#![allow(unused)]

fn main() {
    let inp = readv::<u64>();
    let n = inp[0] as usize;
    let k = inp[1];
    let nxt = mapv(&readv::<usize>(), |&x| x - 1);

    let mut ans = vec![0; n];
    for cycle in find_cycles_in_permutation_graph(&nxt) {
        let rem = powmod(2, k, cycle.len() as u64) as usize;
        for i in 0..cycle.len() {
            ans[cycle[i]] = cycle[(i + rem) % cycle.len()];
        }
    }

    let ans = mapv(&ans, |&x| x + 1);
    println!("{}", join(&ans, " "));
}

fn find_cycles_in_permutation_graph(nxt: &Vec<usize>) -> Vec<Vec<usize>> {
    let n = nxt.len();
    let mut idx = vec![!0; n];
    let mut cycles = vec![];
    for r in 0..n {
        if idx[r] == !0 {
            idx[r] = 0;
            let mut path = vec![r];
            let mut u = nxt[r];
            while idx[u] == !0 {
                idx[u] = path.len();
                path.push(u);
                u = nxt[u];
            }
            cycles.push(path);
        }
    }
    cycles
}

fn powmod(a: u64, mut b: u64, m: u64) -> u64 {
    let mut base = a % m;
    let mut res = 1;
    while b != 0 {
        if b & 1 == 1 {
            res = res * base % m;
        }
        base = base * base % m;
        b >>= 1;
    }
    res
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
