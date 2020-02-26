/// ## Program: rot_n.rs
///
/// ## Project Google doc:
/// https://docs.google.com/document/d/1GaKXhcJAGxK3tKRVn_ZWYFrF0QlMTlOs2Nke4Wdz6u0/edit?usp=sharing
///
/// # Usage:
/// `rotn n file`
///
/// where `rotn` is the binary name, `n` is an integer, and `file` is a filename
/// This program will open the given file and produce a new file which is a copy
/// of the given file, except that each alphabetic ascii character from the input
/// is rotated `n` spaces through the alphabet. If `n` is a negative number, then
/// the alphabetic ascii characters will be rotated that many spaces backwards
/// through the alphabet. We say that a letter is "rotated" because the alphabet
/// wraps from z to a and vice versa. The case of each letter is preserved.
/// The output file is named the same as the input file but with the suffix
/// ".rot" followed by the given `n`
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

const RANGE: isize = 26;
const LOWERCASE_BOUNDS: (isize, isize) = ('a' as isize, 'z' as isize);
const UPPERCASE_BOUNDS: (isize, isize) = ('A' as isize, 'Z' as isize);

fn main() {
    let (mut n, in_name) = parse_args();
    let out_name = format!("{}.rot{}", &in_name, n);
    n %= RANGE;

    // attempt to open input file
    let infile =
        File::open(&in_name).unwrap_or_else(|_| panic!("Could not open input file: {}", in_name));
    // attempt to open output file
    let outfile = File::create(&out_name)
        .unwrap_or_else(|_| panic!("Could not open output file: {}", out_name));

    rotate_file(&infile, &outfile, n);
}

/// Retrieves and parses the command line arguments, returning the number to rotate by
/// and the input filename respectively
fn parse_args() -> (isize, String) {
    //First three arugments from command line
    let mut args: Vec<_> = std::env::args().take(3).collect();
    if args.len() < 3 {
        println!(
            "Missing command line arguments! Expected at least 2, got {}",
            args.len() - 1
        );
        std::process::exit(1);
    }
    let n = args[1]
        .parse()
        .expect("Could not parse first command line argument as a number");
    let fname = args.remove(2);
    (n, fname)
}

/// Reads from `input` and writes to `output`.
/// `output` will be rotated `by_n` as described above.
fn rotate_file(input: &File, output: &File, by_n: isize) {
    const READ_ERR: &str = "Error reading from input file";
    const WRITE_ERR: &str = "Error writing to output file";
    debug_assert!(by_n.abs() < RANGE);

    let mut in_buffer = BufReader::new(input);
    let mut out_buffer = BufWriter::new(output);
    let mut line = String::new();
    loop {
        let bytes_read = in_buffer.read_line(&mut line).expect(READ_ERR);
        if bytes_read > 0 {
            rot_str(&mut line, by_n);
            out_buffer.write_all(line.as_bytes()).expect(WRITE_ERR);
            line.clear();
        } else {
            // reached EOF
            out_buffer.flush().expect(WRITE_ERR);
            return;
        }
    }
}

/// Rotates all of the character in the given String
fn rot_str(s: &mut String, by_n: isize) {
    debug_assert!(by_n.abs() < RANGE);
    let mut dest = String::with_capacity(s.capacity());
    for c in s.chars() {
        dest.push(rot_char(c, by_n));
    }
    // overwrite old str with new
    *s = dest;
}

/// If `c` is an alphabetic ascii char, produces
/// that character rotated `by_n` through the alphabet,
/// looping back to a when z is passed. If `c` is not an
/// alphabetic ascii char, then `c` is returned unchanged.
fn rot_char(c: char, by_n: isize) -> char {
    debug_assert!(by_n.abs() < RANGE);
    if c.is_ascii_alphabetic() {
        // select the appropriate upper and lower bounds
        let (start, end) = if c.is_ascii_uppercase() {
            UPPERCASE_BOUNDS
        } else {
            LOWERCASE_BOUNDS
        };
        // add n and adjust as necessary back into
        // the proper range of start..=end
        let mut adjusted = c as isize + by_n;
        if adjusted < start {
            adjusted += RANGE;
        } else if end < adjusted {
            adjusted -= RANGE;
        }
        adjusted as u8 as char
    } else {
        c
    }
}
