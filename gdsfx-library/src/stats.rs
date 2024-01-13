use std::fmt;

#[derive(Debug)]
pub struct Centiseconds(pub i64);

impl fmt::Display for Centiseconds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}s", self.0 as f64 / 100.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_format_centiseconds() {
        macro_rules! test {
            ( $centiseconds:literal, $expected:literal ) => {
                assert_eq!(format!("{}", Centiseconds($centiseconds)), $expected);
            }
        }

        test!(   0,  "0.00s");
        test!(  12,  "0.12s");
        test!( 345,  "3.45s");
        test!(6789, "67.89s");

        test!(   1,  "0.01s");
        test!(  10,  "0.10s");
        test!( 100,  "1.00s");
        test!(1000, "10.00s");
    }
}
