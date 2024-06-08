#![allow(unused)]

fn main() {
    let f = |x: i64, y: i64| -> i64 {
        let nx = x / 4;
        let ny = y / 2;
        let mut ans = 8 * nx * ny;

        let col_areas = vec![0, 3, 6, 7];
        let row_areas = vec![0, 4];
        let rx = x - nx * 4;
        let ry = y - ny * 2;
        ans += col_areas[rx as usize] * ny;
        ans += row_areas[ry as usize] * nx;

        match (rx, ry) {
            (1, 1) => (ans + 2),
            (2, 1) => (ans + 3),
            (3, 1) => (ans + 3),
            _ => (ans + 0),
        }
    };

    let offset = 10i64.pow(9);
    let inp = readv::<i64>();
    let (a, b, c, d) = (inp[0], inp[1], inp[2], inp[3]);
    let (x1, x2) = (offset + a, offset + c);
    let (y1, y2) = (offset + b, offset + d);
    let ans = f(x2, y2) - f(x1, y2) - f(x2, y1) + f(x1, y1);
    println!("{}", ans);
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
