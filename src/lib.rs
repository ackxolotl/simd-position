#![feature(portable_simd)]

use std::simd::{u8x32, Simd};
use std::simd::prelude::*;

pub fn next_newline_std(input: &[u8]) -> Option<usize> {
    for (pos, chr) in input.iter().enumerate() {
        if *chr == b'\n' {
            return Some(pos);
        }
    }
    None
}

pub fn next_newline_simd(input: &[u8]) -> Option<usize> {
    let mask = u8x32::splat(b'\n');
    let mut pos = 0;
    while pos < input.len() {
        let data = Simd::<u8, 32>::load_or_default(&input[pos..]);
        let positions = mask.simd_eq(data).to_bitmask();
        if positions != 0 {
            return Some(pos + positions.trailing_zeros() as usize);
        }
        pos += data.len();
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_next_newline<F>(func: F)
    where
        F: Fn(&[u8]) -> Option<usize>,
    {
        let input = b"hello\nrustaceans\nthisisaverylonginputstring\n";
        let next = func(input);
        assert_eq!(next, Some(5));

        let next = func(&input[6..]);
        assert_eq!(next, Some(10));

        let next = func(&input[17..]);
        assert_eq!(next, Some(26));

        let input = b"hello rustaceans thisisaverylonginputstring\n";
        let next = func(input);
        assert_eq!(next, Some(43));
    }

    #[test]
    fn test_next_newline_std() {
        test_next_newline(next_newline_std);
    }

    #[test]
    fn test_next_newline_simd() {
        test_next_newline(next_newline_simd);
    }
}