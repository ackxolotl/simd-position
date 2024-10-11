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

pub fn split_std(input: &[u8]) -> Vec<&[u8]> {
    input.split(|&i| i == b'\n').collect()
}

pub fn split_simd(input: &[u8]) -> Vec<&[u8]> {
    let mut result = vec![];

    let mask = u8x32::splat(b'\n');
    let mut last = 0;

    let mut pos = 0;
    while pos < input.len() {
        let data = Simd::<u8, 32>::load_or_default(&input[pos..]);
        let mut positions = mask.simd_eq(data).to_bitmask();

        while positions != 0 {
            let bit = positions.trailing_zeros();
            let offset = pos + bit as usize;

            result.push(&input[last..offset]);

            last = offset + 1;
            positions ^= 1 << bit as u64;
        }

        pos += data.len();
    }

    result.push(&input[last..]);

    result
}

pub fn subsplit_std(input: &[u8]) -> Vec<String> {
    input.split(|&i| i == b'\n').map(|s| s.split(|&i| i == b'|')).flatten().map(|slice| std::str::from_utf8(slice).unwrap().to_string()).collect()
}

pub fn subsplit_simd(input: &[u8]) -> Vec<String> {
    let mut result = vec![];
    subsplit_simd_helper(input, b'\n', |i: &[u8]| {
        subsplit_simd_helper(i, b'|', |o: &[u8]| {
            let s = std::str::from_utf8(o).unwrap();
            result.push(s.to_string());
        });
    });
    result
}

pub fn subsplit_simd_helper<F>(input: &[u8], delimiter: u8, mut f: F)
where
    F: FnMut(&[u8]),
{
    let mask = u8x32::splat(delimiter);
    let mut last = 0;

    let mut pos = 0;
    while pos < input.len() {
        let data = Simd::<u8, 32>::load_or_default(&input[pos..]);
        let mut positions = mask.simd_eq(data).to_bitmask();

        while positions != 0 {
            let bit = positions.trailing_zeros();
            let offset = pos + bit as usize;

            f(&input[last..offset]);

            last = offset + 1;
            positions ^= 1 << bit as u64;
        }

        pos += data.len();
    }

    f(&input[last..]);
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

    fn test_split<F>(func: F)
    where
        F: Fn(&[u8]) -> Vec<&[u8]>,
    {
        let input = b"hello\nrustaceans\nthisisaverylonginputstring\n";
        let next = func(input);
        assert_eq!(next, input.split(|&i| i == b'\n').collect::<Vec<_>>());
    }

    #[test]
    fn test_next_newline_std() {
        test_next_newline(next_newline_std);
    }

    #[test]
    fn test_next_newline_simd() {
        test_next_newline(next_newline_simd);
    }

    #[test]
    fn test_split_std() {
        test_split(split_std);
    }

    #[test]
    fn test_split_simd() {
        test_split(split_simd);
    }

    #[test]
    fn test_subsplit_simd() {
        let input = b"hel|o\nru|tace|ns\nthi|isaveryl|nginput|tring\n";
        let next = subsplit_simd(input);
        assert_eq!(next, subsplit_std(input));
    }
}