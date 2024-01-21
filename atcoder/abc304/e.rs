#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);

    let mut adj = vec![vec![]; n];
    for _ in 0..m {
        let inp = readv::<usize>();
        let (u, v) = (inp[0] - 1, inp[1] - 1);
        adj[u].push(v);
        adj[v].push(u);
    }

    let mut vis = vec![false; n];
    let mut gid = vec![!0; n];
    let mut cnt = 0;
    for root in 0..n {
        if vis[root] {
            continue;
        }
        let mut que = std::collections::VecDeque::new();
        vis[root] = true;
        gid[root] = cnt;
        que.push_back(root);
        while let Some(u) = que.pop_front() {
            for &v in adj[u].iter() {
                if !vis[v] {
                    vis[v] = true;
                    gid[v] = cnt;
                    que.push_back(v);
                }
            }
        }
        cnt += 1;
    }

    let mut blacklist = std::collections::HashSet::new();
    let k = read::<usize>();
    for _ in 0..k {
        let inp = readv::<usize>();
        let (x, y) = (inp[0] - 1, inp[1] - 1);
        let (gx, gy) = (gid[x], gid[y]);
        let (gx, gy) = if gx < gy { (gx, gy) } else { (gy, gx) };
        blacklist.insert((gx, gy));
    }

    // println!("{:?}", gid);
    // println!("{:?}", blacklist);

    let q = read::<usize>();
    let mut ans = vec![];
    for _ in 0..q {
        let inp = readv::<usize>();
        let (p, q) = (inp[0] - 1, inp[1] - 1);
        let (gp, gq) = (gid[p], gid[q]);
        let (gp, gq) = if gp < gq { (gp, gq) } else { (gq, gp) };
        if blacklist.contains(&(gp, gq)) {
            ans.push("No");
        } else {
            ans.push("Yes");
        }
    }

    println!("{}", join(&ans, "\n"));
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
