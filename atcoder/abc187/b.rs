#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut x = vec![];
    let mut y = vec![];
    for _ in 0..n {
        let inp = readv::<i32>();
        x.push(inp[0]);
        y.push(inp[1]);
    }

    let mut cnt = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            let mut dx = x[j] - x[i];
            let mut dy = y[j] - y[i];
            // -1 <= dy/dx <= +1
            if dx < 0 {
                dx = -dx;
                dy = -dy;
            }
            if dy <= dx && dy >= -dx {
                cnt += 1;
            }
        }
    }

    println!("{}", cnt);
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

fn mapv<T, S>(arr: &Vec<T>, f: fn(&T) -> S) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(v: &[T], sep: &str) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
