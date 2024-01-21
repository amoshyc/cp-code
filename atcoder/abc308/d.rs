#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let mut arr = vec![];
    for _ in 0..n {
        arr.push(reads());
    }

    if arr[0][0] != 's' {
        println!("No");
        return;
    }

    let snuke = vec!['s', 'n', 'u', 'k', 'e'];
    let mut que = std::collections::VecDeque::new();
    let mut dis = std::collections::HashMap::new();

    que.push_back((0, 0));
    dis.insert((0, 0), 0);
    while let Some((r, c)) = que.pop_front() {
        if (r, c) == (n - 1, m - 1) {
            println!("Yes");
            return;
        }

        for &(dr, dc) in [(0, 1), (0, !0), (1, 0), (!0, 0)].iter() {
            let nr = r.wrapping_add(dr);
            let nc = c.wrapping_add(dc);
            let nd = dis[&(r, c)] + 1;
            if !dis.contains_key(&(nr, nc)) && nr < n && nc < m && arr[nr][nc] == snuke[nd % 5] {
                que.push_back((nr, nc));
                dis.insert((nr, nc), nd);
            }
        }
    }

    println!("No");
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
