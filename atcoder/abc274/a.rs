fn main() {
    let inp = input::<f64>();
    let (a, b) = (inp[0], inp[1]);
    println!("{:1.3}", b / a);
}

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

// fn join<T>(v: &Vec<T>, sep: &str) -> String
// where
//     T: ToString,
// {
//     v.iter()
//         .map(|x| x.to_string())
//         .collect::<Vec<String>>()
//         .join(sep)
// }
