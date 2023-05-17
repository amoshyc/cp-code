#![allow(unused)]

fn main() {
    let inp = readv::<i64>();
    let (n, p, q, r) = (inp[0] as usize, inp[1], inp[2], inp[3]);
    let arr = readv::<i64>();

    let mut pref = vec![0; n];
    pref[0] = arr[0];
    for i in 1..n {
        pref[i] = pref[i - 1] + arr[i];
    }

    for x in 0..n {
        let sub = if x == 0 { 0 } else { pref[x - 1] };
        let y = partition_point(&pref, |v| v - sub <= p);
        let z = partition_point(&pref, |v| v - sub <= p + q);
        let w = partition_point(&pref, |v| v - sub <= p + q + r);

        if y == 0 || z == 0 || w == 0 {
            continue;
        }

        let ok1 = pref[y - 1] - sub == p;
        let ok2 = pref[z - 1] - sub == p + q;
        let ok3 = pref[w - 1] - sub == p + q + r;

        if x < y && y < z && z < w && w <= n && ok1 && ok2 && ok3 {
            // println!("{} {} {} {}", x, y, z, w);
            println!("Yes");
            return;
        }
    }

    println!("No");
}

// arr.partition_point is added at 1.52.0
// 1 1 1 0 0 0
//       ^
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
