fn main() {
    let inp = readv::<u64>();
    let (mut x, k) = (inp[0], inp[1]);

    for i in 1..=k {
        let base = 10_u64.pow(i as u32);
        if x % base >= base / 2 {
            x = (x / base + 1) * base;
        } else {
            x = x / base * base;
        }
    }

    println!("{}", x);
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

// fn join<T: ToString>(v: &[T], sep: &str) -> String {
//     v.iter()
//         .map(|x| x.to_string())
//         .collect::<Vec<String>>()
//         .join(sep)
// }