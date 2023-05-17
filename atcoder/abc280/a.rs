fn main() {
    let inp = readv::<usize>();
    let n = inp[0];
    let mut cnt = 0;
    for _ in 0..n {
        for &c in reads().iter() {
            if c == '#' {
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

// fn join<T: ToString>(v: &[T], sep: &str) -> String {
//     v.iter()
//         .map(|x| x.to_string())
//         .collect::<Vec<String>>()
//         .join(sep)
// }
