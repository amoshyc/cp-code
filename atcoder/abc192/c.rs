fn main() {
    let inp = readv::<u32>();
    let (n, k) = (inp[0], inp[1]);
    let mut a = n;
    for _ in 0..k {
        let mut digits = vec![];
        while a != 0 {
            digits.push(a % 10);
            a = a / 10;
        }
        digits.sort();

        let g1 = digits.iter().fold(0, |acc, &x| acc * 10 + x);
        let g2 = digits.iter().rev().fold(0, |acc, &x| acc * 10 + x);
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
