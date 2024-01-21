fn main() {
    let n = read::<usize>();
    let arr = readv::<i32>();
    let mut s = arr.clone();
    s.sort();
    s.dedup();

    let mut ans = vec![0; n + 1];
    for a in arr {
        // let p = s.partition_point(|&x| x <= a);
        let p = partition_point(&s, |&x| x <= a);
        let cnt = s.len() - p;
        ans[cnt] += 1;
    }

    println!("{}", join(&ans[..n], "\n"));
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

fn join<T: ToString>(v: &[T], sep: &str) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}

// arr.partition_point is added at 1.52.0
fn partition_point<T, P: FnMut(&T) -> bool>(arr: &[T], mut pred: P) -> usize {
    arr.binary_search_by(|x| {
        if pred(x) {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    })
    .unwrap_or_else(|i| i)
}
