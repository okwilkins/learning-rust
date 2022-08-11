use std::env;
use text_colorizer::Colorize;

fn main() {
    let args = parse_args();

    let floor_num = calculate_floor(&args.input as &str);
    let first_basement_char = calculate_basement_char_loc(&args.input as &str);

    println!("Number of floors Santa must climb: {floor_num}");
    match first_basement_char {
        None => println!("Santa did not reach the basement!"),
        Some(char_loc) => println!("Santa reached the basement at character: {char_loc}"),
    };
}

fn print_usage() {
    eprintln!("{} - ", "santa-floor-parser".green());
    eprintln!("Usage: santa-floor-parser <INPUT>");
}

#[derive(Debug)]
struct Arguments {
    input: String,
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

    Arguments {
        input: args[0].clone(),
    }
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

/// Finds the position of the first character that causes him to enter the basement (floor -1).
/// The first character in the instructions has position 1,
/// the second character has position 2, and so on.
fn calculate_basement_char_loc(input: &str) -> Option<isize> {
    for i in 0..input.len() {
        let floor_num = calculate_floor(&input[..i + 1]);

        if floor_num == -1 {
            return Some((i + 1) as isize);
        }
    }

    None
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

#[test]
fn test_calculate_basement_char_loc() {
    assert_eq!(calculate_basement_char_loc("(())"), None);
    assert_eq!(calculate_basement_char_loc("()()"), None);
    assert_eq!(calculate_basement_char_loc("((("), None);
    assert_eq!(calculate_basement_char_loc("(()(()("), None);

    assert_eq!(calculate_basement_char_loc(")"), Some(1));
    assert_eq!(calculate_basement_char_loc("()())"), Some(5));

    assert_eq!(calculate_basement_char_loc("())"), Some(3));
    assert_eq!(calculate_basement_char_loc("))("), Some(1));
}
