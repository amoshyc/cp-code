fn main() {
    let n: usize = read();
    let inp: Vec<i32> = readv();
    let mut dep = vec![0; 2 * n + 2];
    for (i, &a) in inp.iter().enumerate() {
        let a = a as usize;
        dep[2 * (i + 1) + 0] = dep[a] + 1;
        dep[2 * (i + 1) + 1] = dep[a] + 1;
    }
    println!("{}", join(&dep[1..dep.len()], "\n"));
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

fn join<T: ToString>(v: &[T], sep: &str) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}