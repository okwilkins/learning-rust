use std::env;

fn main() {
    let args = parse_args();

    let (wrapping_order, ribbon_order) = calculate_wrapping_order(&args.input);
    println!("The elves' wrapping order is: {wrapping_order:?} sq ft");
    println!("The elves' ribbon order is: {ribbon_order:?} sq ft");
}

#[derive(Debug, PartialEq)]
struct PresentDimensions {
    l: usize,
    w: usize,
    h: usize,
}

#[derive(Debug)]
struct Arguments {
    input: String,
}

fn calculate_wrapping_order(input: &str) -> (usize, usize) {
    let mut wrapping_order_size: usize = 0;
    let mut ribbon_order_size: usize = 0;

    for dimensions in input.split('\n') {
        wrapping_order_size += match parse_wrapping_dimensions(dimensions) {
            None => 0,
            Some(x) => calculate_wrapping_footage(x),
        };
        ribbon_order_size += match parse_wrapping_dimensions(dimensions) {
            None => 0,
            Some(x) => calculate_ribbon_footage(x),
        };
    }

    (wrapping_order_size, ribbon_order_size)
}

fn calculate_wrapping_footage(input: PresentDimensions) -> usize {
    let area_1 = input.l * input.w;
    let area_2 = input.w * input.h;
    let area_3 = input.h * input.l;

    (2 * area_1) + (2 * area_2) + (2 * area_3) + [area_1, area_2, area_3].iter().min().unwrap()
}

fn calculate_ribbon_footage(input: PresentDimensions) -> usize {
    let mut sorted_inputs = [input.l, input.w, input.h];
    sorted_inputs.sort();
    2 * sorted_inputs[0] + 2 * sorted_inputs[1] + (input.l * input.w * input.h)
}

fn parse_wrapping_dimensions(input: &str) -> Option<PresentDimensions> {
    let input_split: Vec<usize> = input
        .split('x')
        .map({
            |x| match x.trim().parse::<usize>() {
                Ok(s) => Some(s),
                Err(_) => None,
            }
        })
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect();

    if input_split.len() != 3 {
        return None;
    }

    Some(PresentDimensions {
        l: input_split[0],
        w: input_split[1],
        h: input_split[2],
    })
}

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 1 {
        eprintln!("Wrong number of arguments: expected 1, got {}.", args.len());
    }

    Arguments {
        input: args[0].clone(),
    }
}

#[test]
fn test_calculate_wrapping_footage() {
    assert_eq!(
        calculate_wrapping_footage(PresentDimensions { l: 2, w: 3, h: 4 }),
        58
    );
    assert_eq!(
        calculate_wrapping_footage(PresentDimensions { l: 1, w: 1, h: 10 }),
        43
    );
}

#[test]
fn test_calculate_ribbon_footage() {
    assert_eq!(
        calculate_ribbon_footage(PresentDimensions { l: 2, w: 3, h: 4 }),
        34
    );
    assert_eq!(
        calculate_ribbon_footage(PresentDimensions { l: 1, w: 1, h: 10 }),
        14
    );
}

#[test]
fn test_parse_wrapping_dimensions() {
    assert_eq!(
        parse_wrapping_dimensions("2x3x4"),
        Some(PresentDimensions { l: 2, w: 3, h: 4 })
    );
    assert_eq!(
        parse_wrapping_dimensions("1x1x10"),
        Some(PresentDimensions { l: 1, w: 1, h: 10 })
    );
    assert_eq!(parse_wrapping_dimensions("234"), None);
    assert_eq!(parse_wrapping_dimensions(""), None);
    assert_eq!(parse_wrapping_dimensions("2x34"), None);
}

#[test]
fn test_calculate_wrapping_order() {
    assert_eq!(calculate_wrapping_order("2x3x4"), (58, 34));
    assert_eq!(calculate_wrapping_order("1x1x10"), (43, 14));
    assert_eq!(calculate_wrapping_order("234"), (0, 0));
    assert_eq!(calculate_wrapping_order(""), (0, 0));
    assert_eq!(calculate_wrapping_order("2x34"), (0, 0));
    assert_eq!(
        calculate_wrapping_order(
            r#"2x3x4
            1x1x10
            "#
        ),
        (58 + 43, 34 + 14)
    );
    assert_eq!(
        calculate_wrapping_order(
            r#"2x3x4
            1x1x10
            asdasd
            "#
        ),
        (58 + 43 + 0, 34 + 14 + 0)
    );
    assert_eq!(
        calculate_wrapping_order(
            r#"2x3x4
                1x1x10
            "#
        ),
        (58 + 43, 34 + 14)
    );
    assert_eq!(
        calculate_wrapping_order(
            r#"2x3x4
                1x1x10 2x3x4
            "#
        ),
        (58 + 0, 34 + 0)
    );
    assert_eq!(
        calculate_wrapping_order(
            r#"2x3x4asdasd
            "#
        ),
        (0, 0)
    );
    assert_eq!(
        calculate_wrapping_order(
            r#"2x3x4asdasd
            1x1x10
            "#
        ),
        (0 + 43, 0 + 14)
    );
}
