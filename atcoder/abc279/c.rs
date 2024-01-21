fn main() {
    let inp = readv::<usize>();
    let (h, _) = (inp[0], inp[1]);

    let mut a = vec![];
    let mut b = vec![];
    for _ in 0..h {
        a.push(reads());
    }
    for _ in 0..h {
        b.push(reads());
    }

    let mut ta = transpose(&a);
    let mut tb = transpose(&b);
    ta.sort();
    tb.sort();

    // println!("{:?}", ha);
    // println!("{:?}", hb);

    println!("{}", if ta == tb { "Yes" } else { "No" });
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

fn transpose<T: Copy>(arr: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(arr.len() >= 1);
    let mut res = vec![];
    for c in 0..arr[0].len() {
        res.push(arr.iter().map(|row| row[c]).collect::<Vec<T>>());
    }
    res
}
