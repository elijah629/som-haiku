use crate::numbers::format_human;
use std::io::{self, Read, Write};

mod est;
mod numbers;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn filter_is_haiku(input: &[u8]) -> Vec<u8> {
    use std::io::Cursor;

    let mut output = vec![];
    let reader = Cursor::new(input);

    process(reader, &mut output).unwrap();

    output
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn into_haiku(input: &str) -> String {
    let strwords = format(input);
    let words = strwords
        .split_ascii_whitespace()
        .map(est::estimate)
        .collect::<Vec<_>>();

    let mut output = String::new();

    let sum = words.iter().sum();

    for (left, right) in compute_haiku_padding(sum, &words) {
        for _ in 0..left {
            output.push_str("a ");
        }

        // Push main content
        output.push_str(input);

        for _ in 0..right {
            output.push_str(" a");
        }

        output.push('\n');
    }

    output
}

pub fn process<R: Read + std::io::BufRead, W: Write>(reader: R, mut writer: W) -> io::Result<()> {
    for line in reader.lines() {
        let line_content = line?;

        let words = format(&line_content);
        let words = words.split_ascii_whitespace().map(est::estimate);

        if is_haiku(words) {
            writeln!(writer, "{}", line_content)?;
        }
    }

    writer.flush()?;

    Ok(())
}

fn format(text: &str) -> String {
    let text = text.to_ascii_lowercase();

    let mut output = String::with_capacity(text.len() * 2);
    let mut number = 0;
    let mut in_number = false;

    for c in text.chars() {
        if let Some(d) = c.to_digit(10) {
            number = number * 10 + d as usize;
            in_number = true;
        } else {
            if in_number {
                output.push_str(&format_human(number));
                number = 0;
                in_number = false;
            }
            output.push(c);
        }
    }

    if in_number {
        output.push_str(&format_human(number));
    }

    output
}

#[cfg(target_arch = "wasm32")]
pub fn compute_haiku_padding(total: u8, id_sylls: &[u8]) -> Vec<(u8, u8)> {
    // Vec<(u8, u8)> {
    let total_len = id_sylls.len() as u8;
    // how many extra 1‐syllables we can distribute
    let max_pad = 17u8.saturating_sub(total);

    let mut sol = Vec::with_capacity(5);

    // try every possible total pad count
    for pad in 0..=max_pad {
        // split that pad between left and right
        for left in 0..=pad {
            let right = pad - left;

            // build the full stream: `left` ones, then core syllables, then `right` ones
            let len = left + total_len + right;

            // try to carve out 5, then 7, then 5
            let mut idx = 0;
            let mut ok = true;
            for &target in &[5, 7, 5] {
                let mut sum = 0;
                while sum < target {
                    if idx >= len {
                        ok = false;
                        break;
                    }
                    sum += if idx < left {
                        1 // left padding
                    } else if idx < left + total_len {
                        id_sylls[(idx - left) as usize] // core syllables
                    } else {
                        1 // right padding
                    };
                    idx += 1;
                }
                if !ok || sum != target {
                    ok = false;
                    break;
                }
            }

            // ensure we've consumed the entire stream
            if ok && idx == len {
                sol.push((left, right));
            }
        }
    }

    sol
}

pub fn is_haiku<I>(mut sylls: I) -> bool
where
    I: Iterator<Item = u8>,
{
    for target in [5, 7, 5] {
        let mut sum = 0;
        for syll in sylls.by_ref() {
            sum += syll;
            if sum >= target {
                break;
            }
        }
        if sum != target {
            return false;
        }
    }

    sylls.next().is_none()
}
