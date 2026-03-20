#![allow(unused)]

use proconio::input;

fn main() {
    input! {
        q: usize,
        a: [usize; q],
    }

    let mut playing = false;
    let mut volume = 0 as usize;
    let mut ans = vec![];

    for &a in &a {
        if a == 1 {
            volume += 1;
        } else if a == 2 {
            volume = volume.saturating_sub(1);
        } else {
            playing = !playing;
        }

        if playing && volume >= 3 {
            ans.push("Yes");
        } else {
            ans.push("No");
        }
    }

    println!("{}", join(&ans, "\n"));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
