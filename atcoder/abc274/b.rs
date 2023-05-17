fn main() {
    let inp = readv::<usize>();
    let (h, w) = (inp[0], inp[1]);

    let mut a = vec![];
    for _ in 0..h {
        a.push(read::<String>().chars().collect::<Vec<char>>());
    }

    let mut ans = vec![0; w];
    for r in 0..h {
        for c in 0..w {
            if a[r][c] == '#' {
                ans[c] += 1;
            }
        }
    }

    println!("{}", join(&ans, " "));
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

fn join<T: ToString>(v: &Vec<T>, sep: &str) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
