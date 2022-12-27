/*
--- Day 11: Corporate Policy ---

Santa's previous password expired, and he needs help choosing a new one.

To help him remember his new password after the old one expires, Santa has devised a method of coming up with a
password based on the previous one. Corporate policy dictates that passwords must be exactly eight lowercase letters
(for security reasons), so he finds his new password by incrementing his old password string repeatedly until it is valid.

Incrementing is just like counting with numbers: xx, xy, xz, ya, yb, and so on. Increase the rightmost letter one step;
if it was z, it wraps around to a, and repeat with the next letter to the left until one doesn't wrap around.

Unfortunately for Santa, a new Security-Elf recently started, and he has imposed some additional password requirements:

Passwords must include one increasing straight of at least three letters, like abc, bcd, cde, and so on, up to xyz.
hey cannot skip letters; abd doesn't count.

Passwords may not contain the letters i, o, or l, as these letters can be mistaken for other characters and are therefore confusing.
Passwords must contain at least two different, non-overlapping pairs of letters, like aa, bb, or zz.

For example:

hijklmmn meets the first requirement (because it contains the straight hij) but fails the second
requirement requirement (because it contains i and l).

abbceffg meets the third requirement (because it repeats bb and ff) but fails the first requirement.

abbcegjk fails the third requirement, because it only has one double letter (bb).

The next password after abcdefgh is abcdffaa.

The next password after ghijklmn is ghjaabcc, because you eventually skip all the passwords that start with
ghi..., since i is not allowed.

Given Santa's current password (your puzzle input), what should his next password be?

Your puzzle input is cqjxjnds.

--- Part Two ---

Santa's password expired again. What's the next one?
*/

#![warn(clippy::pedantic)]

const PUZZLE_INPUT: &str = include_str!("../inputs/day11.txt");

fn no_forbidden_letters(password: &str) -> bool {
    !password.contains('i') && !password.contains('o') && !password.contains('l')
}

fn has_two_pairs(password: &str) -> bool {
    let mut pairs = 0;
    let mut last = ' ';
    let mut last_pair = ' ';
    for c in password.chars() {
        if c == last {
            if c != last_pair {
                pairs += 1;
                last_pair = c;
            }
        } else {
            last_pair = ' ';
        }
        last = c;
    }
    pairs >= 2
}

fn has_straight(password: &str) -> bool {
    let mut straight = 0;
    let mut last = ' ';
    for c in password.chars() {
        if c as u8 == last as u8 + 1 {
            straight += 1;
            if straight == 2 {
                return true;
            }
        } else {
            straight = 0;
        }
        last = c;
    }
    false
}

fn is_valid(password: &str) -> bool {
    no_forbidden_letters(password) && has_two_pairs(password) && has_straight(password)
}

fn increment(password: &str) -> String {
    let mut password = password.to_string();
    let mut i = password.len() - 1;
    loop {
        let c = password.chars().nth(i).unwrap();
        if c == 'z' {
            password.replace_range(i..=i, "a");
            i -= 1;
        } else {
            password.replace_range(i..=i, &((c as u8 + 1) as char).to_string());
            break;
        }
    }
    password
}

fn increment_until_valid(start: &str) -> String {
    let mut password = start.to_string();
    loop {
        password = increment(&password);
        if is_valid(&password) {
            break;
        }
    }

    password
}

fn main() {
    let password = PUZZLE_INPUT.trim();
    let part1 = increment_until_valid(password);
    let part2 = increment_until_valid(&part1);
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_no_forbidden_letters() {
        assert!(no_forbidden_letters("abc"));
        assert!(!no_forbidden_letters("abi")); // i
        assert!(!no_forbidden_letters("alb")); // l
        assert!(!no_forbidden_letters("obc")); // o
    }

    #[test]
    fn test_has_two_pairs() {
        assert!(has_two_pairs("abbceffg"));
        assert!(!has_two_pairs("abbcegjk"));
    }

    #[test]
    fn test_has_straight() {
        assert!(has_straight("hijklmmn"));
        assert!(!has_straight("abbcegjk"));
    }

    #[test]
    fn test_is_valid() {
        assert!(is_valid("abcdffaa"));
        assert!(is_valid("ghjaabcc"));
        assert!(!is_valid("hijklmmn"));
        assert!(!is_valid("abbceffg"));
        assert!(!is_valid("abbcegjk"));
    }

    #[test]
    fn test_increment() {
        assert_eq!(increment("abcd"), "abce");
        assert_eq!(increment("abce"), "abcf");
        assert_eq!(increment("abcz"), "abda");
        assert_eq!(increment("abzz"), "acaa");
    }
}
