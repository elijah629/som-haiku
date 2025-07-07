use crate::numbers::format_human;
use std::io::{self, BufRead};
use std::iter::repeat_n;

mod est;
mod numbers;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        match line {
            Ok(line_content) => {
                let words = format(&line_content);
                let words = words.split(' ').map(est::SyllableEstimator::estimate);

                //                let padding = compute_haiku_padding(&words);
                // sum of the “core” words

                if compute_haiku_padding(&words.collect::<Vec<_>>()) {
                    println!("{line_content}");
                }
            }
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                break;
            }
        }
    }
    /*    let words = format("U092FMXURQV");
    let words = words.split(' ').collect::<Vec<_>>();

    println!("{words:?}");

    let words = words
        .into_iter()
        .map(est::SyllableEstimator::estimate)
        .collect::<Vec<_>>();

    println!("{words:?}");

    println!("{:?}", compute_haiku_padding(&words));*/
}

/*fn format(text: &str) -> String {
    let text = text.to_ascii_lowercase();

    let mut out = String::new(); //with_capacity(text.len());
    let mut chars = text.chars().peekable();

    while let Some(&ch) = chars.peek() {
        if ch.is_ascii_digit() {
            // parse the full digit run
            let mut n = 0u64;
            while let Some(&d) = chars.peek() {
                if d.is_ascii_digit() {
                    // safe because d is '0'..'9'
                    n = n * 10 + (d as u8 - b'0') as u64;
                    chars.next();
                } else {
                    break;
                }
            }
            out.push_str(&format_human(n));
        } else {
            // copy any non‑digit
            out.push(ch);
            chars.next();
        }
    }

    out
}*/

fn format(text: &str) -> String {
    let text = text.to_ascii_lowercase();

    let mut out = Vec::with_capacity(text.len()); // Start with capacity to avoid reallocations
    let mut chars = text.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch.is_ascii_digit() {
            // Parse the full digit run efficiently
            let mut n = 0u64;
            while let Some(&d) = chars.peek() {
                if d.is_ascii_digit() {
                    n = n * 10 + (d as u8 - b'0') as u64;
                    chars.next(); // consume the digit
                } else {
                    break;
                }
            }
            out.extend(format_human(n).chars()); // Collect the formatted number directly
        } else {
            // Copy non-digit characters directly
            out.push(ch);
        }
    }

    // Convert the collected characters into a String once
    String::from_iter(out)
}

pub fn compute_haiku_padding(id_sylls: &[usize]) -> bool {
    // sum of the “core” words
    let total: usize = id_sylls.iter().sum();
    // how many extra 1‐syllables we can distribute
    let max_pad = 17usize.saturating_sub(total);

    //let mut sol = Vec::new();

    // try every possible total pad count
    for pad in 0..=max_pad {
        // split that pad between left and right
        for left in 0..=pad {
            let right = pad - left;

            // build the full stream: `left` ones, then core syllables, then `right` ones
            let mut stream = Vec::with_capacity(left + id_sylls.len() + right);
            stream.extend(repeat_n(1, left));
            stream.extend(id_sylls.iter().cloned());
            stream.extend(repeat_n(1, right));

            // try to carve out 5, then 7, then 5
            let mut idx = 0;
            let mut ok = true;
            for &target in &[5, 7, 5] {
                let mut sum = 0;
                while sum < target {
                    if idx >= stream.len() {
                        ok = false;
                        break;
                    }
                    sum += stream[idx];
                    idx += 1;
                }
                if !ok || sum != target {
                    ok = false;
                    break;
                }
            }

            // ensure we've consumed the entire stream
            if ok && idx == stream.len() {
                return left == 0 && right == 0;
                //sol.push((left, right));
            }
        }
    }

    false
    //sol
}

/*
#[derive(Debug, Clone)]
enum Syl {
    N(u8),
    B,
    L(String),
}

fn format(text: &str) -> Vec<u8> {
    let text = text.to_ascii_lowercase();

    let mut out = vec![]; //String::with_capacity(text.len());
    let mut chars = text.chars().peekable();

    let mut built = String::new();

    while let Some(&ch) = chars.peek() {
        if ch.is_ascii_digit() {
            if !built.is_empty() {
                out.push(Syl::L(built.clone()));
                built.clear();
            }

            // parse the full digit run
            let mut n = 0u64;
            while let Some(&d) = chars.peek() {
                if d.is_ascii_digit() {
                    // safe because d is '0'..'9'
                    n = n * 10 + (d as u8 - b'0') as u64;
                    chars.next();
                } else {
                    break;
                }
            }
            // replace with your spelled‑out version
            //out.push_str(format_human(n));
            out.extend(format_human(n));
        } else {
            // copy any non‑digit
            built.push(ch);
            chars.next();
        }
    }

    if !built.is_empty() {
        out.push(Syl::L(built.clone()));
    }

    let mut real_out = vec![];
    let mut current = 0;

    println!("{out:?}");
    for s in out {
        match s {
            Syl::N(x) => current += x,
            Syl::B => {
                real_out.push(current);
                current = 0;
            }
            Syl::L(x) => current += SyllableEstimator::estimate(&x) as u8,
        };
    }

    if current != 0 {
        real_out.push(current);
    }

    real_out
}*/
