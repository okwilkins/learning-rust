use std::fmt::Debug;

fn main() {
    let args = parse_args();
    let hash_limit = usize::MAX;

    let secret_num = match mine_number(&args.input, hash_limit) {
        None => "NaN".to_string(),
        Some(x) => x.to_string()
    };

    println!("Secret code: {}", args.input);
    println!("Has this secret number: {}", secret_num);
}

fn mine_number(secret_key: &str, hash_limit: usize) -> Option<usize> {
    for i in 0..hash_limit {
        let md5_digest = md5::compute(format!("{}{}", secret_key, i));
        if &format!("{:x}", md5_digest)[..5] == "00000" {
            return Some(i);
        }
    } 

    None
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

#[test]
fn test_mine_number() {
    assert_eq!(mine_number("abcdef", usize::MAX), Some(609043));
}
