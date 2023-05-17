fn solve() -> u64 {
    let inp = reads();
    let digits = inp
        .iter()
        .map(|&c| c.to_digit(10).unwrap() as u64)
        .collect::<Vec<u64>>();

    let m = read::<u64>();

    if digits.len() == 1 {
        if digits[0] <= m {
            return 1;
        } else {
            return 0;
        }
    }

    let check = |base: u64| -> bool {
        let val = digits.iter().fold(0 as u64, |acc, &d| {
            acc.saturating_mul(base).saturating_add(d)
        });
        val <= m
    };

    let max_digit = digits.iter().max().unwrap();
    let mut lb = max_digit + 1;
    let mut ub = m + 1;
    if !check(lb) {
        return 0;
    }
    while ub - lb > 1 {
        let m = (lb + ub) / 2;
        if check(m) {
            lb = m;
        } else {
            ub = m;
        }
    }

    lb - max_digit
}

fn main() {
    println!("{}", solve());
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
