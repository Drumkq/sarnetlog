use std::ffi::c_void;

use crate::utils::ModuleInfo;

use super::pattern::Pattern;

#[derive(Debug, Clone)]
pub struct ScanError {
    pub pattern: String,
    pub begin: *const c_void,
    pub size: usize,
}

pub struct Scanner {
    begin: *const c_void,
    size: usize,
}

impl Scanner {
    pub fn new(mi: ModuleInfo) -> Self {
        Scanner {
            begin: mi.base,
            size: mi.size,
        }
    }

    /// Searches for the desired pattern in the previously defined memory area
    ///
    /// # Returns
    /// ***const c_void** - ``memory address where the pattern specified in the function parameter starts``
    ///
    /// **ScanError** - ``scanner error type``
    ///
    /// # Example
    /// ```rust,ignore
    /// const TEST_ARRAY: &[u8; 13] = b"\xa3\x13\x13\xac\x13\xa3\xa3\x14\xa3\x19\x13\x14\x15";
    /// const TEST_PATTERN: Pattern<5> = Pattern::new("A3 ?? 13 14 15");
    ///
    /// let scanner = Scanner::new(TEST_ARRAY.as_ptr().cast::<c_void>(), TEST_ARRAY.len());
    ///
    /// unsafe {
    ///     let addr = scanner.find(TEST_PATTERN).expect("Not passed");
    ///     // Use addr
    /// }
    /// ```
    pub unsafe fn find<const N: usize>(
        &self,
        pattern: Pattern<N>,
    ) -> Result<*const c_void, ScanError> {
        let Self { begin, size } = *self;

        for i in 0..=size - 1 {
            let mut found = true;
            let Pattern::<N> { mask, bytes, .. } = pattern;

            for n_bytes in 0..N {
                if mask[n_bytes] == false {
                    continue;
                }

                if *(begin as *const u8).byte_offset((i + n_bytes) as isize) != bytes[n_bytes] {
                    found = false;
                    break;
                }
            }

            if found {
                return Ok(self.begin.byte_offset(i as isize));
            }
        }

        Err(ScanError {
            pattern: pattern.raw.into(),
            begin,
            size,
        })
    }
}

#[test]
fn scan_test() {
    const TEST_ARRAY: &[u8; 13] = b"\xa3\x13\x13\xac\x13\xa3\xa3\x14\xa3\x19\x13\x14\x15";
    const TEST_PATTERN: Pattern<5> = Pattern::new("A3 ?? 13 14 15");

    let mi = ModuleInfo {
        base: TEST_ARRAY.as_ptr().cast::<c_void>(),
        size: TEST_ARRAY.len(),
        entry: std::ptr::null(),
    };
    let scanner = Scanner::new(mi);

    unsafe {
        let addr = scanner.find(TEST_PATTERN).expect("Not passed");
        assert_eq!(*(addr as *const u8), TEST_ARRAY[8]);
    }
}
