use crate::numbers::format_human;
use std::io::Write;
use std::io::{self, BufRead, BufWriter};

mod est;
mod numbers;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let handle = stdin.lock();

    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    for line in handle.lines() {
        let line_content = line?;

        let words = format(&line_content)
            .split(' ')
            .map(est::estimate)
            .collect::<Vec<_>>();

        let sum = words.iter().sum();

        /*for (left, right) in compute_haiku_padding(sum, &words) {
            for _ in 0..left {
                write!(writer, "a ")?;
            }

            // Push main content
            write!(writer, "{}", &line_content)?;

            for _ in 0..right {
                write!(writer, " a")?;
            }

            writeln!(writer)?;
        }*/

        if compute_haiku_padding(sum, &words) {
            writeln!(writer, "{line_content}")?;
        }
    }

    writer.flush()?;

    Ok(())
}

fn format(text: &str) -> String {
    let text = text.to_ascii_lowercase();

    let mut output = String::with_capacity(text.len() * 2);
    let mut number: u16 = 0;
    let mut in_number = false;

    for c in text.chars() {
        if let Some(d) = c.to_digit(10) {
            number = number * 10 + d as u16;
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

pub fn compute_haiku_padding(total: u8, id_sylls: &[u8]) -> bool {
    // Vec<(u8, u8)> {
    let total_len = id_sylls.len() as u8;
    // how many extra 1‐syllables we can distribute
    let max_pad = 17u8.saturating_sub(total);

    //let mut sol = Vec::with_capacity(5);

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
                return left == 0 && right == 0;
                //return Some((left, right));
                //sol.push((left, right));
            }
        }
    }

    false
    //sol
    //None
}
