/*use crate::Syl;

const ONES: [u8; 10] = [2, 1, 1, 1, 1, 1, 1, 2, 1, 1];
const TEENS: [u8; 10] = [1, 2, 1, 2, 2, 2, 2, 3, 2, 2];
const TENS: [u8; 10] = [0, 0, 2, 2, 2, 2, 2, 3, 2, 2];
const SCALES: [u8; 5] = [0, 2, 2, 2, 2];

#[inline(always)]
fn write_hundreds(n: u16, out: &mut Vec<Syl>) {
    // Handles 0..999, but we only call when n > 0
    let mut rem = n;
    if rem >= 100 {
        let h = rem / 100;
        out.push(Syl::N(ONES[h as usize]));
        out.push(Syl::B);
        out.push(Syl::N(2)); // hundred
        out.push(Syl::N(1)); // and
        rem %= 100;

        if rem != 0 {
            out.push(Syl::B);
        }
    }

    if rem >= 20 {
        let t = rem / 10;
        out.push(Syl::N(TENS[t as usize]));
        let u = rem % 10;
        if u != 0 {
            //out.push('-');
            out.push(Syl::N(ONES[u as usize]));
        }
    } else if rem >= 10 {
        out.push(Syl::N(TEENS[(rem - 10) as usize]));
    } else if rem > 0 {
        out.push(Syl::N(ONES[rem as usize]));
    }
}

/// Spell out any number 0 <= n < 10^15 (extend SCALES for larger).
pub fn format_human(mut n: u64) -> Vec<Syl> {
    if n == 0 {
        return vec![Syl::N(ONES[0])];
    }

    // Split into 3‑digit groups
    let mut parts = Vec::new();
    let mut scale_idx = 0;
    while n > 0 {
        let chunk = (n % 1_000) as u16;
        if chunk != 0 {
            // Build words for this chunk
            let mut chunk_str = vec![];
            write_hundreds(chunk, &mut chunk_str);
            // Append scale suffix (e.g. "million") if needed
            if SCALES[scale_idx] != 0 {
                //chunk_str.push(' ');
                chunk_str.push(Syl::B);
                chunk_str.push(Syl::N(SCALES[scale_idx]));
            }
            parts.push(chunk_str);
        }
        n /= 1_000;
        scale_idx += 1;
    }

    // Now `parts` holds ["four hundred ninety-two", "two hundred", "one"] for 1_200_492
    // We need to join them in reverse order:
    let mut result = vec![];
    for part in parts.iter().rev() {
        if !result.is_empty() {
            result.push(Syl::B);
        }

        result.extend_from_slice(part);
    }
    result
}*/

const ONES: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

const TEENS: [&str; 10] = [
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

const TENS: [&str; 10] = [
    "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

const SCALES: [&str; 5] = ["", "thousand", "million", "billion", "trillion"];

#[inline(always)]
fn write_hundreds(n: u16, out: &mut String) {
    // Handles 0..999, but we only call when n > 0
    let mut rem = n;
    if rem >= 100 {
        let h = rem / 100;
        out.push_str(ONES[h as usize]);
        out.push_str(" hundred and");
        rem %= 100;
        if rem != 0 {
            out.push(' ');
        }
    }
    if rem >= 20 {
        let t = rem / 10;
        out.push_str(TENS[t as usize]);
        let u = rem % 10;
        if u != 0 {
            out.push('-'); // -
            out.push_str(ONES[u as usize]);
        }
    } else if rem >= 10 {
        out.push_str(TEENS[(rem - 10) as usize]);
    } else if rem > 0 {
        out.push_str(ONES[rem as usize]);
    }
}

/// Spell out any number 0 <= n < 10^15 (extend SCALES for larger).
pub fn format_human(mut n: u64) -> String {
    if n == 0 {
        return ONES[0].to_string();
    }

    // Split into 3‑digit groups
    let mut parts = Vec::new();
    let mut scale_idx = 0;
    while n > 0 {
        let chunk = (n % 1_000) as u16;
        if chunk != 0 {
            // Build words for this chunk
            let mut chunk_str = String::new();
            write_hundreds(chunk, &mut chunk_str);
            // Append scale suffix (e.g. "million") if needed
            if !SCALES[scale_idx].is_empty() {
                chunk_str.push(' ');
                chunk_str.push_str(SCALES[scale_idx]);
            }
            parts.push(chunk_str);
        }
        n /= 1_000;
        scale_idx += 1;
    }

    // Now `parts` holds ["four hundred ninety-two", "two hundred", "one"] for 1_200_492
    // We need to join them in reverse order:
    let mut result = String::new();
    for part in parts.iter().rev() {
        if !result.is_empty() {
            result.push(' ');
        }
        result.push_str(part);
    }
    result
}
