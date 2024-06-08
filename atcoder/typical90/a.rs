#![allow(unused)]

// [Problem]
// There is a piece of yokan with a length of L [cm].
// It has N cuts, and the i-th cut from the left is at position Ai [cm].
// You want to choose K cuts out of N and divide the yokan into K+1 pieces.
// Therefore, define the following value as the score:
//     The length of the shortest piece among the K+1 pieces (in cm)
// Please find the score obtained when dividing it to maximize the score.

// [Solution]
// Binary search the length of the shortest piece.
// To check a specific length of the shortest piece m,
// We can cut the yokan greedily to maximum the number of piece
// And check if the number >= k + 1

fn main() {
    let inp = readv::<usize>();
    let (n, l) = (inp[0], inp[1]);
    let k = read::<usize>();
    let mut arr = readv::<usize>();
    arr.insert(0, 0);
    arr.push(l);

    let ok = |m: usize| -> bool {
        let mut cnt = 0;
        let mut prv = arr[0];
        for i in 1..arr.len() {
            if arr[i] - prv >= m {
                cnt += 1;
                prv = arr[i];
            }
        }
        cnt >= k + 1
    };

    // 1 1 1 0 0 0
    let mut lb = 0;
    let mut ub = l + 1;
    while ub - lb > 1 {
        let m = (lb + ub) / 2;
        if ok(m) {
            lb = m;
        } else {
            ub = m;
        }
    }

    println!("{}", lb);
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

fn mapv<T, S, F: Fn(&T) -> S>(arr: &Vec<T>, f: F) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(v: &[T], sep: &str) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
