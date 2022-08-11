use std::env;
use text_colorizer::Colorize;

fn main() {
    let args = parse_args();
    let floor_num = calculate_floor(&args.input as &str);
    eprintln!("Number of floors Santa must climb: {}", floor_num);
}

fn print_usage() {
    eprintln!(
        "{} - ",
        "santa-floor-parser".green()
    );
    eprintln!("Usage: santa-floor-parser <INPUT>");
}

#[derive(Debug)]
struct Arguments {
    input: String
}

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 1 {
        print_usage();
        eprintln!(
            "{} wrong number of arguments: expected 1, got {}.",
            "error:".red().bold(),
            args.len()
        );
    }

    Arguments { input: args[0].clone() }
}

/// Santa is trying to deliver presents in a large apartment building,
/// but he can't find the right floor - the directions he got are a little confusing.
/// He starts on the ground floor (floor 0) and then follows the
/// instructions one character at a time.
///
/// An opening parenthesis, (, means he should go up one floor,
/// and a closing parenthesis, ), means he should go down one floor.
fn calculate_floor(input: &str) -> isize {
    let up_floor_count: isize = input.matches('(').count() as isize;
    let down_floor_count: isize = input.matches(')').count() as isize;

    up_floor_count - down_floor_count
}

#[test]
fn test_calculate_floor() {
    assert_eq!(calculate_floor("(())"), 0);
    assert_eq!(calculate_floor("()()"), 0);

    assert_eq!(calculate_floor("((("), 3);
    assert_eq!(calculate_floor("(()(()("), 3);

    assert_eq!(calculate_floor("))((((("), 3);

    assert_eq!(calculate_floor("())"), -1);
    assert_eq!(calculate_floor("))("), -1);

    assert_eq!(calculate_floor(")))"), -3);
    assert_eq!(calculate_floor(")())())"), -3);
}