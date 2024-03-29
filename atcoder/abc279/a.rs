fn main() {
    let s = reads();
    let a = s.iter().fold(0, |acc, &x| {
        match x {
            'w' => acc + 2,
            'v' => acc + 1,
            _ => acc
        }
    });
    println!("{}", a);
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

// fn readv<T: std::str::FromStr>() -> Vec<T> {
//     read::<String>()
//         .split_ascii_whitespace()
//         .map(|t| t.parse().ok().unwrap())
//         .collect()
// }

fn reads() -> Vec<char> {
    read::<String>().chars().collect::<Vec<char>>()
}