const RANGE: isize = 26;
const LOWERCASE_BOUNDS: (isize, isize) = ('a' as isize, 'z' as isize);
const UPPERCASE_BOUNDS: (isize, isize) = ('A' as isize, 'Z' as isize);

fn main() {
    let (n, fname) = parse_args();
    // TODO: file handling
}

fn parse_args() -> (isize, String) {
    let mut args: Vec<_> = std::env::args().take(3).collect();
    if args.len() < 3 {
        println!("Missing command line arguments! Expected at least 2, got {}", args.len() - 1);
        std::process::exit(1);
    }
    let n = args[1].parse()
        .expect("Could not parse first command line argument as a number");
    let fname = args.remove(2);
    (n, fname)
}

/// Consumes `self` and produces a new String in which each char of self
/// is rotated by `n`.
fn rot_str(s: &mut String, mut by_n: isize) {
    let mut dest = String::with_capacity(s.len());
    by_n %= RANGE;
    for c in s.chars() {
        dest.push(rot_char(c, by_n));
    }
    *s = dest;
}

/// If this char is an alphabetic ascii char,
/// produces that character rotated by n through the alphabet, looping
/// back to a when z is passed.
fn rot_char(c: char, by_n: isize) -> char {
    assert!(by_n.abs() < RANGE);
    if c.is_ascii_alphabetic() {
        // select the appropriate upper and lower bounds
        let (start, end) =
            if c.is_ascii_uppercase() {
                UPPERCASE_BOUNDS
            } else {
                LOWERCASE_BOUNDS
            };
        // add n and adjust as necessary back into
        // the proper range of start..=end
        let mut adjusted = by_n + c as isize;
        if adjusted < start {
            adjusted += RANGE;
        } else if end < adjusted {
            adjusted -= RANGE;
        }
        adjusted as u8 as char
    } else {
        // return unchanged if it cannot be rotated.
        c
    }
}
