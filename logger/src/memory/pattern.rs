use core::str;

#[derive(Debug)]
#[doc = "docs"]
pub struct Pattern<const N: usize> {
    pub bytes: [u8; N],
    pub mask: [bool; N],
    /// IDA-style raw pattern
    pub raw: &'static str,
}

impl<const N: usize> Pattern<N> {
    pub const fn new(raw: &'static str) -> Self {
        let mut bytes: [u8; N] = [0; N];
        let mut mask: [bool; N] = [true; N];
        let chars: &[u8] = raw.as_bytes();

        let mut i = 0;
        let mut n_bytes = 0;
        while i < raw.len() && n_bytes < N + 1 {
            let current_char = chars[i];
            if current_char == b' ' {
                i += 1;
                n_bytes += 1;
                continue;
            }
            if current_char == b'?' {
                mask[n_bytes] = false;

                if chars[i + 1] != b'?' {
                    panic!("Invalid pattern literal: there must be two '?' in a row");
                }

                i += 2;
                continue;
            }

            let next_char: u8 = chars[i + 1];

            bytes[n_bytes] = Self::char_to_byte(current_char) << 4 | Self::char_to_byte(next_char);

            i += 2;
        }

        Pattern::<N> { raw, mask, bytes }
    }

    const fn char_to_byte(byte: u8) -> u8 {
        if byte >= b'a' && byte <= b'z' {
            return (byte - b'a') + 0xA;
        } else if byte >= b'A' && byte <= b'Z' {
            return (byte - b'A') + 0xA;
        } else if byte >= b'0' && byte <= b'9' {
            return byte - b'0';
        } else {
            return 0;
        }
    }
}

#[macro_export]
macro_rules! decl_pattern {
    () => { compile_error!("invalid expression: the first argument should be a name and the second argument should be a pattern literal") };
    ($name: ident) => { compile_error!("invalid expression: missing pattern literal") };
    ($name: ident, $s: literal) => {
        pub const $name: crate::memory::Pattern::<{($s.len() + 1) / 3}> = crate::memory::Pattern::new($s);
    };
}

#[test]
fn parse_test() {
    const TEST_PATTERN: Pattern<5> = Pattern::new("A3 ?? 13 14 15");
    assert_eq!(TEST_PATTERN.bytes.len(), 5);
    assert_eq!(TEST_PATTERN.mask.len(), 5);

    assert_eq!(TEST_PATTERN.raw, "A3 ?? 13 14 15");
    assert_eq!(TEST_PATTERN.mask, [true, false, true, true, true]);
    assert_eq!(TEST_PATTERN.bytes, *b"\xa3\x00\x13\x14\x15");
}
