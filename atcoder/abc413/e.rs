#![allow(unused)]

fn main() {
    //     8 3 4 2 1 5 7 6
    // A:  -------
    // B:          -------
    // C:  ---------------

    // Let f(C, <) return the C after operations to make it small to large.
    // Since all operation except the last largest one cannot affect both A and B,
    // there are only 2 results:
    // (1) Don't apply the op: f(A, <) + f(B, <)
    // (2) Apply the op: rev(f(A, >) + f(B, >))
    // Next we consider which is better:
    // (1) is the result if min(A) < min(B)
    // (2) is the result otherwise.
    // Note that rev(f(A, >) + f(B, >)) = f(B, <) + f(A, <)

    // In summary, f(C, <) = {
    //     f(A, <) + f(B, <) if min(A) < min(B),
    //     f(B, <) + f(A, <) otherwise,
    // }

    let tc = read::<usize>();
    let mut res = vec![];
    for _ in 0..tc {
        let n = read::<usize>();
        let arr = readv::<usize>();
        let ans = f(&arr);
        res.push(join(&ans, " "));
    }
    println!("{}", join(&res, "\n"));
}

fn f(arr: &[usize]) -> Vec<usize> {
    if arr.len() == 1 {
        return arr.to_vec();
    }

    let m = arr.len() / 2;
    let a = f(&arr[..m]);
    let b = f(&arr[m..]);

    let mut res = vec![];
    if a[0] < b[0] {
        res.extend(a);
        res.extend(b);
    } else {
        res.extend(b);
        res.extend(a);
    }
    res
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s);
    s.trim().parse().ok().unwrap()
}

fn readv<T: std::str::FromStr>() -> Vec<T> {
    read::<String>()
        .split_ascii_whitespace()
        .map(|t| t.parse().ok().unwrap())
        .collect()
}

fn reads() -> Vec<char> {
    read::<String>().chars().collect()
}

fn mapv<T, S, F: Fn(&T) -> S>(arr: &Vec<T>, f: F) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
