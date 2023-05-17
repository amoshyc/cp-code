use std::io;

fn input() -> String {
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).unwrap();
    inp.trim().to_string()
}

fn main() {
    let n: usize = input().parse().unwrap();
    let l: Vec<i32> = input()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut ans = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            for k in (j + 1)..n {
                if l[i] == l[j] || l[j] == l[k] || l[i] == l[k] {
                    continue;
                }
                if l[i] + l[j] > l[k] && l[j] + l[k] > l[i] && l[i] + l[k] > l[j] {
                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans);
}
