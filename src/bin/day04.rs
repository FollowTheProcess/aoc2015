/*
--- Day 4: The Ideal Stocking Stuffer ---

Santa needs help mining some AdventCoins (very similar to bitcoins) to use as gifts for all the
economically forward-thinking little girls and boys.

To do this, he needs to find MD5 hashes which, in hexadecimal, start with at least five zeroes.
The input to the MD5 hash is some secret key (your puzzle input, given below) followed by a number in decimal.
To mine AdventCoins, you must find Santa the lowest positive number (no leading zeroes: 1, 2, 3, ...) that produces such a hash.

For example:

If your secret key is abcdef, the answer is 609043, because the MD5 hash of abcdef609043 starts with five zeroes (000001dbbfa...),
and it is the lowest such number to do so.
If your secret key is pqrstuv, the lowest number it combines with to make an MD5 hash starting with five zeroes is 1048970; that is,
the MD5 hash of pqrstuv1048970 looks like 000006136ef....

--- Part Two ---

Now find one that starts with six zeroes.
*/

#![warn(clippy::pedantic)]

const PUZZLE_INPUT: &str = include_str!("../inputs/day04.txt");

fn find_md5_with_five_zeros(secret: &str, startswith: &str) -> u32 {
    let mut i = 0;
    loop {
        let hash = md5::compute(format!("{secret}{i}"));
        let hex = format!("{hash:x}");
        if hex.starts_with(startswith) {
            return i;
        }
        i += 1;
    }
}

fn main() {
    let part1 = find_md5_with_five_zeros(PUZZLE_INPUT.trim(), "00000");
    let part2 = find_md5_with_five_zeros(PUZZLE_INPUT.trim(), "000000");

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(find_md5_with_five_zeros("abcdef", "00000"), 609_043);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(find_md5_with_five_zeros("pqrstuv", "00000"), 1_048_970);
    }
}
