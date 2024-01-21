fn input<T>() -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut buf = String::new();
    std::io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read");
    buf.split_ascii_whitespace()
        .map(|t| String::from(t).parse::<T>().unwrap())
        .collect::<Vec<T>>()
}

fn main() {
    let v = input::<i32>();
    let (_, k) = (v[0], v[1]);
    let mut a = input::<i32>();
    for _ in 0..k {
        a.remove(0);
        a.push(0);
    }
    let ans = a.iter().map(|x| x.to_string()).collect::<Vec<String>>();
    println!("{}", ans.join(" "));
}
