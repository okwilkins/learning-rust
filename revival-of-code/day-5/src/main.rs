use std::fmt::Debug;

fn main() {
    let args = parse_args();
    let mut num_nice_text: usize = 0;
    let mut num_improved_nice_text: usize = 0;

    for arg_line in args.input.lines() {
        num_nice_text += detect_nice_text(arg_line) as usize;
        num_improved_nice_text += improved_detect_nice_text(arg_line) as usize;
    }

    println!("Number of nice texts: {}", num_nice_text);
    println!(
        "Number of nice texts (improved detection): {}",
        num_improved_nice_text
    );
}

#[derive(Debug)]
struct Arguments {
    input: String,
}

fn parse_args() -> Arguments {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.len() != 1 {
        eprintln!("Wrong number of arguments: expected 1, got {}.", args.len());
    }

    Arguments {
        input: args[0].clone(),
    }
}

fn detect_nice_text(input: &str) -> bool {
    // Manually create alpha chars for Regex as this can avoid backreferences
    // which is very bad for performance
    let lower_alphas = (b'a'..=b'z')
        .map(|c| format!("{}{}", c as char, c as char))
        .collect::<Vec<_>>();
    let mut lower_alpha_match = false;

    let vowels = vec!["a", "e", "u", "i", "o"];
    let mut vowels_count: usize = 0;

    let bad_patterns = vec!["ab", "cd", "pq", "xy"];
    let mut bad_match = false;

    for lower_alpha in lower_alphas {
        if input.matches(lower_alpha.as_str()).count() >= 1 {
            lower_alpha_match = true;
            break;
        }
    }

    for vowel in vowels {
        vowels_count += input.matches(vowel).count();
    }

    for bad_pattern in bad_patterns {
        if input.matches(bad_pattern).count() >= 1 {
            bad_match = true;
            break;
        }
    }

    lower_alpha_match && vowels_count >= 3 && !bad_match
}

fn improved_detect_nice_text(input: &str) -> bool {
    let mut one_letter_repeat = false;
    let mut two_letter_repeat = false;

    for i in 0..input.len() - 2 {
        if input.chars().nth(i) == input.chars().nth(i + 2) {
            one_letter_repeat = true;
            break;
        }
    }

    for i in 0..input.len() - 3 {
        let slice = &input[i..i + 2];
        let comparison_slice = &input[i + 2..];

        if comparison_slice.matches(slice).count() >= 1 {
            two_letter_repeat = true;
            break;
        }
    }

    one_letter_repeat && two_letter_repeat
}

#[test]
fn test_detect_nice_text() {
    assert_eq!(detect_nice_text("ugknbfddgicrmopn"), true);
    assert_eq!(detect_nice_text("aaa"), true);
    assert_eq!(detect_nice_text("jchzalrnumimnmhp"), false);
    assert_eq!(detect_nice_text("haegwjzuvuyypxyu"), false);
    assert_eq!(detect_nice_text("dvszwmarrgswjxmb"), false);
}

#[test]
fn test_improved_detect_nice_text() {
    assert_eq!(improved_detect_nice_text("qjhvhtzxzqqjkmpb"), true);
    assert_eq!(improved_detect_nice_text("xxyxx"), true);
    assert_eq!(improved_detect_nice_text("uurcxstgmygtbstg"), false);
    assert_eq!(improved_detect_nice_text("ieodomkazucvgmuy"), false);
}
