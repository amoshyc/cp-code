#![allow(unused)]

// 2 1 3
// 1 10
// 2 3
// 1 2
// Note the pivot

// 1 1 1
// 1 5
// can make both sets empty

fn main() {
    let inp = readv::<usize>();
    let (n, k, q) = (inp[0], inp[1], inp[2]);

    let mut sum = 0 as i64;
    let mut larger = std::collections::BTreeSet::new();
    let mut smaller = std::collections::BTreeSet::new();
    let mut arr = vec![0; n];
    for i in 0..n {
        if i < k {
            larger.insert((arr[i], i));
        } else {
            smaller.insert((arr[i], i));
        }
    }

    let mut ans = vec![];
    for _ in 0..q {
        let inp = readv::<i64>();
        let x = inp[0] as usize - 1;
        let y = inp[1];

        if larger.contains(&(arr[x], x)) {
            larger.remove(&(arr[x], x));
            sum -= arr[x];
        } else {
            smaller.remove(&(arr[x], x));
        }

        arr[x] = y;
        let pivot = if larger.is_empty() {
            smaller.iter().last().unwrap_or(&(0, 0)).0
        } else {
            larger.iter().next().unwrap_or(&(0, 0)).0
        };
        if y >= pivot {
            larger.insert((y, x));
            sum += y;
        } else {
            smaller.insert((y, x));
        }

        if larger.len() > k {
            let &(a, b) = larger.iter().next().unwrap();
            smaller.insert((a, b));
            larger.remove(&(a, b));
            sum -= a;
        }
        if larger.len() < k {
            let &(a, b) = smaller.iter().last().unwrap();
            larger.insert((a, b));
            smaller.remove(&(a, b));
            sum += a;
        }
        ans.push(sum);
    }

    println!("{}", join(&ans, "\n"));
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

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
