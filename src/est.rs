use once_cell::sync::Lazy;
use regex::Regex;

/// SyllableEstimator: estimates syllables in English words.
pub struct SyllableEstimator;

static SUB_SYLLABLES: Lazy<Vec<Regex>> = Lazy::new(|| {
    let patterns = [
        "cial", "tia", "cius", "cious", "uiet", "gious", "geous", "priest", "giu", "dge", "ion",
        "iou", "sia$", ".che$", ".ched$", ".abe$", ".ace$", ".ade$", ".age$", ".aged$", ".ake$",
        ".ale$", ".aled$", ".ales$", ".ane$", ".ame$", ".ape$", ".are$", ".ase$", ".ashed$",
        ".asque$", ".ate$", ".ave$", ".azed$", ".awe$", ".aze$", ".aped$", ".athe$", ".athes$",
        ".ece$", ".ese$", ".esque$", ".esques$", ".eze$", ".gue$", ".ibe$", ".ice$", ".ide$",
        ".ife$", ".ike$", ".ile$", ".ime$", ".ine$", ".ipe$", ".iped$", ".ire$", ".ise$",
        ".ished$", ".ite$", ".ive$", ".ize$", ".obe$", ".ode$", ".oke$", ".ole$", ".ome$", ".one$",
        ".ope$", ".oque$", ".ore$", ".ose$", ".osque$", ".osques$", ".ote$", ".ove$", ".pped$",
        ".sse$", ".ssed$", ".ste$", ".ube$", ".uce$", ".ude$", ".uge$", ".uke$", ".ule$", ".ules$",
        ".uled$", ".ume$", ".une$", ".upe$", ".ure$", ".use$", ".ushed$", ".ute$", ".ved$", ".we$",
        ".wes$", ".wed$", ".yse$", ".yze$", ".rse$", ".red$", ".rce$", ".rde$", ".ily$", ".ely$",
        ".des$", ".gged$", ".kes$", ".ced$", ".ked$", ".med$", ".mes$", ".ned$", ".[sz]ed$",
        ".nce$", ".rles$", ".nes$", ".pes$", ".tes$", ".res$", ".ves$", "ere$",
    ];
    patterns
        .iter()
        .map(|pat| Regex::new(pat).unwrap())
        .collect()
});

static ADD_SYLLABLES: Lazy<Vec<Regex>> = Lazy::new(|| {
    let patterns = [
        "ia",
        "riet",
        "dien",
        "ien",
        "iet",
        "iu",
        "iest",
        "io",
        "ii",
        "ily",
        ".oala$",
        ".iara$",
        ".ying$",
        ".earest",
        ".arer",
        ".aress",
        ".eate$",
        ".eation$",
        "[aeiouym]bl$",
        "[aeiou]{3}",
        "^mc",
        "ism",
        "^mc",
        "asm",
        "([^aeiouy])1l$",
        "[^l]lien",
        "^coa[dglx].",
        "[^gq]ua[^auieo]",
        "dnt$",
    ];
    patterns
        .iter()
        .map(|pat| Regex::new(pat).unwrap())
        .collect()
});

impl SyllableEstimator {
    /// Estimate the number of syllables in `word`.
    pub fn estimate(word: &str) -> usize {
        if word.is_empty() {
            return 0;
        }

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
            "trillion" => 3,

            // If not found
            _ => {
                let w = word.to_lowercase();

                // split on runs of non-vowels (vowels + y)
                // equivalent to JS: word.split(/[^aeiouy]+/)
                let parts: Vec<&str> = Regex::new(r"[^aeiouy]+")
                    .unwrap()
                    .split(&w)
                    .filter(|s| !s.is_empty())
                    .collect();

                let mut syllables = parts.len();

                for re in SUB_SYLLABLES.iter() {
                    if re.is_match(&w) {
                        syllables = syllables.saturating_sub(1);
                    }
                }

                for re in ADD_SYLLABLES.iter() {
                    if re.is_match(&w) {
                        syllables += 1;
                    }
                }

                if syllables == 0 { 1 } else { syllables }
            }
        }
    }
}
