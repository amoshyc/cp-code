
fn main() {
    let n = read::<usize>();
    let arr = readv::<usize>();

    let mut cnt = vec![0; n];
    for (i, &a) in arr.iter().enumerate() {
        let offset = (n + a - i) % n;
        cnt[offset] += 1;
        cnt[(offset + 1) % n] += 1;
        cnt[(offset + n - 1) % n] += 1;
    }

    println!("{}", cnt.iter().max().unwrap());
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

// fn reads() -> Vec<char> {
//     read::<String>().chars().collect::<Vec<char>>()
// }
