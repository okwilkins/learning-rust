use std::env;
use text_colorizer::Colorize;

fn main() {
    let args = parse_args();
    let floor_num = parse_input(&args.input as &str);
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

fn parse_input(input: &str) -> isize {
    let up_floor_count: isize = input.matches('(').count() as isize;
    let down_floor_count: isize = input.matches(')').count() as isize;

    up_floor_count - down_floor_count
}

#[test]
fn test_parse_input() {
    assert_eq!(parse_input("(())"), 0);
    assert_eq!(parse_input("()()"), 0);

    assert_eq!(parse_input("((("), 3);
    assert_eq!(parse_input("(()(()("), 3);

    assert_eq!(parse_input("))((((("), 3);

    assert_eq!(parse_input("())"), -1);
    assert_eq!(parse_input("))("), -1);

    assert_eq!(parse_input(")))"), -3);
    assert_eq!(parse_input(")())())"), -3);
}