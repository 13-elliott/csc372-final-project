fn main() {
    let (s, n) = parse_args();
    println!("{}", rot_n::rot_str(n, s));
}

fn parse_args() -> (String, isize) {
    let mut args: Vec<_> = std::env::args().collect();
    if args.len() < 3 {
        println!("no command line args given.");
        std::process::exit(1);
    }
    let n = args[2].parse()
        .expect("Could not parse second command line argument as a number");
    let fname = args.remove(1);
    (fname, n)
}

mod rot_n {
    const RANGE: isize = 26;
    const LOWERCASE_BOUNDS: (isize, isize) = ('a' as u8 as isize, 'z' as u8 as isize);
    const UPPERCASE_BOUNDS: (isize, isize) = ('A' as u8 as isize, 'Z' as u8 as isize);

    pub fn rot_str(mut n: isize, src: String) -> String {
        n %= RANGE;
        let mut dest = String::with_capacity(src.capacity());
        for c in src.chars() {
            dest.push(rot_char(n, c));
        }
        dest
    }

    fn rot_char(n: isize, c: char) -> char {
        assert!(0 <= RANGE && n.abs() < RANGE);
        if c.is_ascii_alphabetic() {
            let (start, end) =
                if c.is_ascii_uppercase() {
                    UPPERCASE_BOUNDS
                } else {
                    LOWERCASE_BOUNDS
                };
            let mut adjusted = n + c as u8 as isize;
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
}