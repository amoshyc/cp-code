fn main() {
    let inp = readv::<u32>();
    let (n, k) = (inp[0], inp[1]);
    let mut a = n;
    for _ in 0..k {
        let mut s = a.to_string().chars().collect::<Vec<char>>();
        s.sort();
        let g1 = s.iter().collect::<String>().parse::<u32>().unwrap();
        s.reverse();
        let g2 = s.iter().collect::<String>().parse::<u32>().unwrap();
        a = g2 - g1;
    }
    println!("{}", a);
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
