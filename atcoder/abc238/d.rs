#![allow(unused)]

fn main() {
    let t = read::<usize>();
    let mut ans = vec![];

    for _ in 0..t {
        let inp = readv::<u64>();
        let (a, s) = (inp[0], inp[1]);
        // x + y = x ^ y + 2 * (x & y)
        // x ^ y = s - 2 * a
        // Given x & y and x ^ y, x and y can be determined (not uniquely).
        // Therefore we check whether the combination of each bit in x & y and x ^ y is possible.
        let x = s - 2 * a;
        let whiltelist = vec![(0, 0), (0, 1), (0, 1), (1, 0)];
        if s >= 2 * a && (0..64).all(|i| whiltelist.contains(&((a >> i) & 1, (x >> i) & 1))) {
            ans.push("Yes");
        } else {
            ans.push("No");
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
