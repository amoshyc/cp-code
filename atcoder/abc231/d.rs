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
    for r in 0..n {
        if !vis[r] && adj[r].len() == 1 {
            vis[r] = true;
            let mut p = r;
            let mut u = adj[r][0];
            loop {
                vis[u] = true;
                if adj[u].len() >= 3 {
                    println!("No");
                    return;
                }
                if adj[u].len() == 2 {
                    let new_p = u;
                    let new_u = adj[u][0] + adj[u][1] - p;
                    u = new_u;
                    p = new_p;
                } else {
                    break;
                }
            }
        }
    }

    let mut ok = true;
    for u in 0..n {
        if adj[u].len() > 0 && !vis[u] {
            ok = false;
        }
    }

    if ok {
        println!("Yes");
    } else {
        println!("No");
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
    read::<String>().chars().collect::<Vec<char>>()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
