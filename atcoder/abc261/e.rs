#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, c) = (inp[0], inp[1]);
    // pref[i, j, 0] = result of 0 after performing 0..=i operations for bit j
    // pref[i, j, 1] = result of 1 after performing 0..=i operations for bit j
    // pref[i, j, 0] = apply(op[i], pref[i - 1, j, 0])
    // pref[i, j, 1] = apply(op[i], pref[i - 1, j, 1])
    let mut pref = vec![vec![vec![0, 0]; 30]; n];
    for i in 0..n {
        let inp = readv::<usize>();
        let (cmd, a) = (inp[0], inp[1]);        
        for j in 0..30 {
            let apply_op = |x| {
                match cmd {
                    1 => x & ((a >> j) & 1),
                    2 => x | ((a >> j) & 1),
                    3 => x ^ ((a >> j) & 1),
                    _ => 0
                }
            };
            let prev0 = if i == 0 { 0 } else { pref[i - 1][j][0] };
            let prev1 = if i == 0 { 1 } else { pref[i - 1][j][1] };
            pref[i][j][0] = apply_op(prev0);
            pref[i][j][1] = apply_op(prev1);
        }
    }

    let mut ans = vec![0; n];
    for i in 0..n {
        let prev = if i == 0 { c } else { ans[i - 1] };
        for j in 0..30 {
            let bit = pref[i][j][(prev >> j) & 1];
            ans[i] |= (bit << j);
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
