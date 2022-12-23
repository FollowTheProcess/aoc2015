/*
--- Day 5: Doesn't He Have Intern-Elves For This? ---

Santa needs help figuring out which strings in his text file are naughty or nice.

A nice string is one with all of the following properties:

It contains at least three vowels (aeiou only), like aei, xazegov, or aeiouaeiouaeiou.
It contains at least one letter that appears twice in a row, like xx, abcdde (dd), or aabbccdd (aa, bb, cc, or dd).
It does not contain the strings ab, cd, pq, or xy, even if they are part of one of the other requirements.
For example:

ugknbfddgicrmopn is nice because it has at least three vowels (u...i...o...), a double letter (...dd...), and none of the disallowed substrings.
aaa is nice because it has at least three vowels and a double letter, even though the letters used by different rules overlap.
jchzalrnumimnmhp is naughty because it has no double letter.
haegwjzuvuyypxyu is naughty because it contains the string xy.
dvszwmarrgswjxmb is naughty because it contains only one vowel.

How many strings are nice?

--- Part Two ---

Realizing the error of his ways, Santa has switched to a better model of determining whether a string is naughty or nice.
None of the old rules apply, as they are all clearly ridiculous.

Now, a nice string is one with all of the following properties:

It contains a pair of any two letters that appears at least twice in the string without overlapping,
like xyxy (xy) or aabcdefgaa (aa), but not like aaa (aa, but it overlaps).

It contains at least one letter which repeats with exactly one letter between them, like xyx, abcdefeghi (efe), or even aaa.
For example:

qjhvhtzxzqqjkmpb is nice because is has a pair that appears twice (qj) and a letter that repeats with exactly one letter between them (zxz).
xxyxx is nice because it has a pair that appears twice and a letter that repeats with one between, even though the letters used by each rule overlap.
uurcxstgmygtbstg is naughty because it has a pair (tg) but no repeat with a single letter between them.
ieodomkazucvgmuy is naughty because it has a repeating letter with one between (odo), but no pair that appears twice.

How many strings are nice under these new rules?
*/

#![warn(clippy::pedantic)]

use itertools::Itertools;

const PUZZLE_INPUT: &str = include_str!("../inputs/day05.txt");

fn contains_at_least_3_vowels(line: &str) -> bool {
    let mut vowel_count = 0;
    for c in line.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => vowel_count += 1,
            _ => (),
        }
    }
    vowel_count >= 3
}

fn contains_double_letter(line: &str) -> bool {
    let mut last_char = ' ';
    for c in line.chars() {
        if c == last_char {
            return true;
        }
        last_char = c;
    }
    false
}

fn contains_disallowed_substring(line: &str) -> bool {
    line.contains("ab") || line.contains("cd") || line.contains("pq") || line.contains("xy")
}

fn is_nice(line: &str) -> bool {
    contains_at_least_3_vowels(line)
        && contains_double_letter(line)
        && !contains_disallowed_substring(line)
}

fn pair_appears_twice_without_overlap(line: &str) -> bool {
    for i in 0..line.len() - 1 {
        let pair = &line[i..i + 2];
        if line[i + 2..].contains(pair) {
            return true;
        }
    }
    false
}

fn letter_repeats_with_one_between(line: &str) -> bool {
    for (a, _, c) in line.chars().tuple_windows() {
        if a == c {
            return true;
        }
    }
    false
}

fn is_nice_part2(line: &str) -> bool {
    pair_appears_twice_without_overlap(line) && letter_repeats_with_one_between(line)
}

fn main() {
    let mut nice_count_part1 = 0;
    let mut nice_count_part2 = 0;
    for line in PUZZLE_INPUT.trim().lines() {
        if is_nice(line) {
            nice_count_part1 += 1;
        }
        if is_nice_part2(line) {
            nice_count_part2 += 1;
        }
    }

    println!("Part 1: {nice_count_part1}");
    println!("Part 2: {nice_count_part2}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_contains_3_vowels() {
        assert!(contains_at_least_3_vowels("ugknbfddgicrmopn"));
        assert!(!contains_at_least_3_vowels("kjhsgdlkhdslk"));
        assert!(!contains_at_least_3_vowels("akhtyrnbi"));
    }

    #[test]
    fn test_contains_double_letter() {
        assert!(contains_double_letter("ugknbfddgicrmopn"));
        assert!(!contains_double_letter("kjhsgdlkhdslk"));
        assert!(!contains_double_letter("akhtyrnbi"));
    }

    #[test]
    fn test_contains_disallowed_substring() {
        assert!(!contains_disallowed_substring("ugknbfddgicrmopn"));
        assert!(!contains_disallowed_substring("kjhsgdlkhdslk"));
        assert!(!contains_disallowed_substring("akhtyrnbi"));
        assert!(contains_disallowed_substring("ab"));
        assert!(contains_disallowed_substring("cd"));
        assert!(contains_disallowed_substring("pq"));
        assert!(contains_disallowed_substring("xy"));
    }

    #[test]
    fn test_is_nice() {
        assert!(is_nice("ugknbfddgicrmopn"));
        assert!(is_nice("aaa"));
        assert!(!is_nice("jchzalrnumimnmhp"));
        assert!(!is_nice("haegwjzuvuyypxyu"));
        assert!(!is_nice("dvszwmarrgswjxmb"));
    }

    #[test]
    fn test_pair_appears_twice_without_overlap() {
        assert!(pair_appears_twice_without_overlap("xyxy"));
        assert!(pair_appears_twice_without_overlap("aabcdefgaa"));
        assert!(!pair_appears_twice_without_overlap("aaa"));
    }

    #[test]
    fn test_letter_repeats_with_one_between() {
        assert!(letter_repeats_with_one_between("xyx"));
        assert!(letter_repeats_with_one_between("abcdefeghi"));
        assert!(letter_repeats_with_one_between("aaa"));
        assert!(!letter_repeats_with_one_between("uurcxstgmygtbstg"));
    }

    #[test]
    fn test_is_nice_part2() {
        assert!(is_nice_part2("qjhvhtzxzqqjkmpb"));
        assert!(is_nice_part2("xxyxx"));
        assert!(!is_nice_part2("uurcxstgmygtbstg"));
        assert!(!is_nice_part2("ieodomkazucvgmuy"));
    }
}
