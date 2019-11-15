use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

const RANGE: isize = 26;
const LOWERCASE_BOUNDS: (isize, isize) = ('a' as isize, 'z' as isize);
const UPPERCASE_BOUNDS: (isize, isize) = ('A' as isize, 'Z' as isize);

fn main() {
    let (n, in_name) = parse_args();
    let out_name = format!("{}.rot{}", &in_name, n);
    let (infile, mut outfile) = get_files(&in_name, &out_name);
    rotate_file(&infile, &mut outfile, n);
}

/// TODO
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

/// TODO
fn get_files(in_name: &str, out_name: &str) -> (File, File) {
    (
        // attempt to open input file
        File::open(in_name).unwrap_or_else(|_| panic!("Could not open input file: {}", in_name)),
        // attempt to open output file
        File::create(out_name).unwrap_or_else(|_| panic!("Could not open input file: {}", in_name)),
    )
}

/// TODO
fn rotate_file(input: &File, output: &mut File, by_n: isize) {
    const READ_ERR: &str = "Error reading from input file";
    const WRITE_ERR: &str = "Error writing to output file";

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

/// TODO
fn rot_str(s: &mut String, mut by_n: isize) {
    let mut dest = String::with_capacity(s.capacity());
    by_n %= RANGE;
    for c in s.chars() {
        dest.push(rot_char(c, by_n));
    }
    *s = dest;
}

/// If `c` is an alphabetic ascii char, produces
/// that character rotated `by_n` through the alphabet,
/// looping back to a when z is passed. If `c` is not an
/// alphabetic ascii char, then `c` is returned unchanged.
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
