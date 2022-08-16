use fancy_regex::Regex;
use std::fmt::Debug;

fn main() {
    let args = parse_args();
    let mut num_nice_text: usize = 0;

    for arg_line in args.input.lines() {
        let nice_text = detect_nice_text(arg_line);
        if nice_text {
            num_nice_text += nice_text as usize;
        }
    }

    println!("Number of nice texts: {}", num_nice_text)
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
    let good_re_1 = Regex::new(r"([a-zA-Z])\1").unwrap();
    let good_re_2 = Regex::new(r"(\w*[aeuio]\w*){3,}").unwrap();
    let bad_re = Regex::new(r"ab|cd|pq|xy").unwrap();

    good_re_1.is_match(input).unwrap()
        && good_re_2.is_match(input).unwrap()
        && !bad_re.is_match(input).unwrap()
}

#[test]
fn test_detect_nice_text() {
    assert_eq!(detect_nice_text("ugknbfddgicrmopn"), true);
    assert_eq!(detect_nice_text("aaa"), true);
    assert_eq!(detect_nice_text("jchzalrnumimnmhp"), false);
    assert_eq!(detect_nice_text("haegwjzuvuyypxyu"), false);
    assert_eq!(detect_nice_text("dvszwmarrgswjxmb"), false);
}
