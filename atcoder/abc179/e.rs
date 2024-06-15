#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, r, m) = (inp[0], inp[1], inp[2]);

    let f = |x: usize| (x * x % m);

    let mut vis = vec![!0; m];
    let mut path = vec![];
    vis[r] = 0;
    path.push(r);

    let mut x = f(r);
    while vis[x] == !0 {
        vis[x] = path.len();
        path.push(x);
        x = f(x);
    }
    let cycle = path[vis[x]..].to_vec();
    let prefix = path[..vis[x]].to_vec();

    let cycle = mapv(&cycle, |&x| x as i64);
    let prefix = mapv(&prefix, |&x| x as i64);
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
