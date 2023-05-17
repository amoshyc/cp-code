fn main() {
    let _ = read::<usize>();
    let arr = readv::<u64>();

    let g = arr.iter().fold(0, |acc, &x| gcd(acc, x));
    let mut ans = 0;
    for a in arr {
        let mut q = a / g;
        while q % 2 == 0 {
            q /= 2;
            ans += 1;
        }
        while q % 3 == 0 {
            q /= 3;
            ans += 1;
        }
        if q != 1 {
            ans = -1;
            break;
        }
    }
    println!("{}", ans);
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
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

// fn join<T: ToString>(v: &[T], sep: &str) -> String {
//     v.iter()
//         .map(|x| x.to_string())
//         .collect::<Vec<String>>()
//         .join(sep)
// }
