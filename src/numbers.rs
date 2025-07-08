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
fn write_hundreds(n: usize, out: &mut String) {
    // Handles 0..999, but we only call when n > 0
    let mut rem = n;
    if rem >= 100 {
        let h = rem / 100;
        out.push_str(ONES[h]);
        out.push_str(" hundred");
        rem %= 100;
        if rem != 0 {
            out.push_str(" and ");
        }
    }
    if rem >= 20 {
        let t = rem / 10;
        out.push_str(TENS[t]);
        let u = rem % 10;
        if u != 0 {
            out.push('-');
            out.push_str(ONES[u]);
        }
    } else if rem >= 10 {
        out.push_str(TEENS[rem - 10]);
    } else if rem > 0 {
        out.push_str(ONES[rem]);
    }
}

/// Spell out any number 0 <= n < 10^15 (extend SCALES for larger).
pub fn format_human(mut n: usize) -> String {
    if n == 0 {
        return ONES[0].to_string();
    }

    // Split into 3‑digit groups
    let mut parts = Vec::new();
    let mut scale_idx = 0;
    while n > 0 {
        let chunk = n % 1_000;
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
