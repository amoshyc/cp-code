use std::collections::HashSet;
use std::collections::VecDeque;

type Set = HashSet<usize>;
type Que = VecDeque<(i32, i32)>;

fn main() {
    let inp = readv::<usize>();
    let (h, w) = (inp[0], inp[1]);
    let mut arr = vec![];
    for _ in 0..h {
        arr.push(reads());
    }

    let mut sr = 0;
    let mut sc = 0;
    for r in 0..h {
        for c in 0..w {
            if arr[r][c] == 'S' {
                sr = r as i32;
                sc = c as i32;
            }
        }
    }

    let edges = [(-1, 0), (1, 0), (0, 1), (0, -1)];
    let mut vis = vec![vec![Set::new(); w]; h];
    for (i, (dr, dc)) in edges.iter().enumerate() {
        let (root_r, root_c) = (sr + dr, sc + dc);

        if !(0..h).contains(&us(root_r)) || !(0..w).contains(&us(root_c)) {
            continue;
        }
        if arr[us(root_r)][us(root_c)] != '.' {
            continue;
        }

        let mut que = Que::new();
        que.push_back((root_r, root_c));
        vis[root_r as usize][root_c as usize].insert(i);

        while !que.is_empty() {
            let (r, c) = que.pop_front().unwrap();
            for (rr, cc) in edges.iter() {
                let (nr, nc) = (r + rr, c + cc);

                if !(0..h).contains(&us(nr)) || !(0..w).contains(&us(nc)) {
                    continue;
                }
                if vis[us(nr)][us(nc)].contains(&i) || arr[us(nr)][us(nc)] != '.' {
                    continue;
                }

                vis[us(nr)][us(nc)].insert(i);
                que.push_back((nr, nc));
            }
        }
    }

    let ans = vis.iter().any(|row| row.iter().any(|v| v.len() >= 2));
    println!("{}", if ans { "Yes" } else { "No" });
}

fn us(i: i32) -> usize {
    return i as usize;
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

// fn join<T: ToString>(v: &[T], sep: &str) -> String {
//     v.iter()
//         .map(|x| x.to_string())
//         .collect::<Vec<String>>()
//         .join(sep)
// }
