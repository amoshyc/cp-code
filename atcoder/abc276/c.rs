fn main() {
    let _ = read::<usize>();
    let mut arr = readv::<usize>();
    prev_permutation(&mut arr);
    println!("{}", join(&arr, " "));
}

fn prev_permutation<T: Ord>(array: &mut [T]) -> Option<()> {
    let k = array.windows(2).rposition(|w| w[0] > w[1])?;
    let j = array.iter().rposition(|a| a < &array[k]).unwrap();
    array.swap(k, j);
    array[(k + 1)..].reverse();
    Some(())
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
