#![allow(unused)]

use proconio::{input, marker::Chars};

fn main() {
    input! {
        tc: usize,
    }

    let mut ans = vec![];
    for _ in 0..tc {
        input! {
            a: Chars,
            b: Chars,
        }

        if normalize(a) == normalize(b) {
            ans.push("Yes");
        } else {
            ans.push("No");
        }
    }

    println!("{}", join(&ans, "\n"));
}

fn normalize(s: Vec<char>) -> Vec<char> {
    let mut stack = vec![];

    for &c in &s {
        if c == '(' {
            stack.push(c);
        } else if c == 'x' {
            stack.push(c);
        } else if c == ')' {
            if stack.len() >= 3 {
                if stack[stack.len() - 3..] == ['(', 'x', 'x'] {
                    stack.pop();
                    stack.pop();
                    stack.pop();
                    stack.push('x');
                    stack.push('x');
                } else {
                    stack.push(c);
                }
            } else {
                stack.push(c);
            }
        }
    }

    stack
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
