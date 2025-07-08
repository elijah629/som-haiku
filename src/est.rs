use regex::Regex;
use std::sync::LazyLock;

static SUB_CONTAINS: [&str; 12] = [
    "cial", "tia", "cius", "cious", "uiet", "gious", "geous", "priest", "giu", "dge", "ion", "iou",
];

static SUB_ENDS_WITH: [&str; 112] = [
    "sia", "che", "ched", "abe", "ace", "ade", "age", "aged", "ake", "ale", "aled", "ales", "ane",
    "ame", "ape", "are", "ase", "ashed", "asque", "ate", "ave", "azed", "awe", "aze", "aped",
    "athe", "athes", "ece", "ese", "esque", "esques", "eze", "gue", "ibe", "ice", "ide", "ife",
    "ike", "ile", "ime", "ine", "ipe", "iped", "ire", "ise", "ished", "ite", "ive", "ize", "obe",
    "ode", "oke", "ole", "ome", "one", "ope", "oque", "ore", "ose", "osque", "osques", "ote",
    "ove", "pped", "sse", "ssed", "ste", "ube", "uce", "ude", "uge", "uke", "ule", "ules", "uled",
    "ume", "une", "upe", "ure", "use", "ushed", "ute", "ved", "we", "wes", "wed", "yse", "yze",
    "rse", "red", "rce", "rde", "ily", "ely", "des", "gged", "kes", "ced", "ked", "med", "mes",
    "ned", "zed", "sed", "nce", "rles", "nes", "pes", "tes", "res", "ves", "ere",
];

static ADD_CONTAINS: [&str; 12] = [
    "ia", "riet", "dien", "ien", "iet", "iu", "iest", "io", "ii", "ily", "asm", "ism",
];

static ADD_ENDS_WITH: [&str; 9] = [
    "oala", "iara", "ying", "earest", "arer", "aress", "eate", "eation", "dnt",
];

static ADD_STARTS_WITH: [&str; 5] = ["mc", "coad", "coag", "coal", "coax"];

static ADD_REGEX: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    let patterns = [
        "[aeiouym]bl$",
        "[aeiou]{3}",
        "([^aeiouy])1l$",
        "[^l]lien",
        "[^gq]ua[^auieo]",
    ];
    patterns
        .iter()
        .map(|pat| Regex::new(pat).unwrap())
        .collect()
});

static SPLIT_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[^aeiouy]+").unwrap());

/// Estimate the number of syllables in `word`.
pub fn estimate(word: &str) -> u8 {
    match word {
        // ONES
        "zero" => 2,
        "one" => 1,
        "two" => 1,
        "three" => 1,
        "four" => 1,
        "five" => 1,
        "six" => 1,
        "seven" => 2,
        "eight" => 1,
        "nine" => 1,

        // TEENS
        "ten" => 1,
        "eleven" => 3,
        "twelve" => 1,
        "thirteen" => 2,
        "fourteen" => 2,
        "fifteen" => 2,
        "sixteen" => 2,
        "seventeen" => 3,
        "eighteen" => 2,
        "nineteen" => 2,

        // TENS
        "twenty" => 2,
        "thirty" => 2,
        "forty" => 2,
        "fifty" => 2,
        "sixty" => 2,
        "seventy" => 3,
        "eighty" => 2,
        "ninety" => 2,

        // SCALES
        "hundred" => 2,
        "thousand" => 2,
        "million" => 2,
        "billion" => 2,
        "trillion" => 2,

        // If not found
        _ => {
            if word.len() <= 1 {
                return word.len() as u8;
            }

            let w = word.to_ascii_lowercase();
            let w = w.as_str();

            let mut negative_syllables = 0;
            for suffix in SUB_ENDS_WITH {
                if w.ends_with(suffix) {
                    negative_syllables += 1;
                }
            }
            for pattern in SUB_CONTAINS {
                if w.contains(pattern) {
                    negative_syllables += 1;
                }
            }

            let mut syllables = 0;
            for pattern in ADD_CONTAINS {
                if w.contains(pattern) {
                    syllables += 1;
                }
            }
            for prefix in ADD_STARTS_WITH {
                if w.starts_with(prefix) {
                    syllables += 1;
                }
            }
            for suffix in ADD_ENDS_WITH {
                if w.ends_with(suffix) {
                    syllables += 1;
                }
            }
            for re in ADD_REGEX.iter() {
                if re.is_match(w) {
                    syllables += 1;
                }
            }

            // split on runs of non-vowels (vowels + y)
            // equivalent to JS: word.split(/[^aeiouy]+/)
            let parts = SPLIT_RE.split(w).filter(|s| !s.is_empty()).count() as u8;

            ((syllables + parts) - negative_syllables).max(1)
        }
    }
}
