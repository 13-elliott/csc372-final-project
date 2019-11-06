use rot_n::Rotate;

fn main() {
    let (n, s) = parse_args();
    println!("{}", s.rotate_by(n));
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

mod rot_n {
    const RANGE: isize = 26;
    const LOWERCASE_BOUNDS: (isize, isize) = ('a' as u8 as isize, 'z' as u8 as isize);
    const UPPERCASE_BOUNDS: (isize, isize) = ('A' as u8 as isize, 'Z' as u8 as isize);

    pub trait Rotate {
        fn rotate_by(self, n: isize) -> Self;
    }

    impl Rotate for String {
        fn rotate_by(self, n: isize) -> Self {
            let mut dest = Self::with_capacity(self.len());
            for c in self.chars() {
                dest.push(c.rotate_by(n));
            }
            dest
        }
    }

    impl Rotate for char {
        fn rotate_by(self, n: isize) -> Self {
            assert!(n.abs() < RANGE);
            if self.is_ascii_alphabetic() {
                let (start, end) =
                    if self.is_ascii_uppercase() {
                        UPPERCASE_BOUNDS
                    } else {
                        LOWERCASE_BOUNDS
                    };
                let mut adjusted = n + self as u8 as isize;
                if adjusted < start {
                    adjusted += RANGE;
                } else if end < adjusted {
                    adjusted -= RANGE;
                }
                adjusted as u8 as char
            } else {
                self
            }
        }
    }
}