/*
--- Day 8: Matchsticks ---

Space on the sleigh is limited this year, and so Santa will be bringing his list as a digital copy.
He needs to know how much space it will take up when stored.

It is common in many programming languages to provide a way to escape special characters in strings.
For example, C, JavaScript, Perl, Python, and even PHP handle special characters in very similar ways.

However, it is important to realize the difference between the number of characters in the code representation
of the string literal and the number of characters in the in-memory string itself.

For example:

"" is 2 characters of code (the two double quotes), but the string contains zero characters.

"abc" is 5 characters of code, but 3 characters in the string data.

"aaa\"aaa" is 10 characters of code, but the string itself contains six "a" characters and a single, escaped quote
character, for a total of 7 characters in the string data.

"\x27" is 6 characters of code, but the string itself contains just one - an apostrophe ('), escaped using hexadecimal notation.

Santa's list is a file that contains many double-quoted string literals, one on each line.
The only escape sequences used are \\ (which represents a single backslash), \" (which represents a lone double-quote character),
and \x plus two hexadecimal characters (which represents a single character with that ASCII code).

Disregarding the whitespace in the file, what is the number of characters of code for string literals minus the number o
characters in memory for the values of the strings in total for the entire file?

For example, given the four strings above, the total number of characters of
string code (2 + 5 + 10 + 6 = 23) minus the total number of characters in memory for string values (0 + 3 + 7 + 1 = 11) is 23 - 11 = 12.

--- Part Two ---

Now, let's go the other way. In addition to finding the number of characters of code, you should now encode each code
representation as a new string and find the number of characters of the new encoded representation, including the surrounding double quotes.

For example:

"" encodes to "\"\"", an increase from 2 characters to 6.
"abc" encodes to "\"abc\"", an increase from 5 characters to 9.
"aaa\"aaa" encodes to "\"aaa\\\"aaa\"", an increase from 10 characters to 16.
"\x27" encodes to "\"\\x27\"", an increase from 6 characters to 11.

Your task is to find the total number of characters to represent the newly encoded strings minus the number of characters of
code in each original string literal. For example, for the strings above, the total encoded length (6 + 9 + 16 + 11 = 42) minus
the characters in the original code representation (23, just like in the first part of this puzzle) is 42 - 23 = 19.
*/

#![warn(clippy::pedantic)]

const PUZZLE_INPUT: &str = include_str!("../inputs/day08.txt");

fn literal_size(s: &str) -> usize {
    s.len()
}

fn memory_size(s: &str) -> usize {
    let mut count = 0;

    let mut chars = s.chars().peekable();
    while chars.peek().is_some() {
        let c = chars.next().unwrap();

        if c == '"' {
            continue;
        }

        if c == '\\' {
            let next = chars.next().unwrap();
            match next {
                '\\' | '"' => {}
                'x' => {
                    assert!(chars.next().unwrap().is_ascii_hexdigit());
                    assert!(chars.next().unwrap().is_ascii_hexdigit());
                }
                _ => panic!("invalid escape sequence"),
            }
        }

        count += 1;
    }

    count
}

fn encode(s: &str) -> String {
    // Opening quote
    let mut encoded = '"'.to_string();

    // Do the encoding
    for c in s.chars() {
        match c {
            '"' => encoded.push_str(r#"\""#),
            '\\' => encoded.push_str(r#"\\"#),
            c => encoded.push(c),
        }
    }

    // Closing quote
    encoded.push('"');

    encoded
}

fn solve(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| literal_size(line) - memory_size(line))
        .sum()
}

fn solve_part2(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| encode(line).len() - literal_size(line))
        .sum()
}

fn main() {
    let part1 = solve(PUZZLE_INPUT);
    let part2 = solve_part2(PUZZLE_INPUT);

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#"
""
"abc"
"aaa\"aaa"
"\x27"
"#;

    #[test]
    fn test_literal_size() {
        assert_eq!(literal_size(r#""""#), 2);
        assert_eq!(literal_size(r#""abc""#), 5);
        assert_eq!(literal_size(r#""aaa\"aaa""#), 10);
        assert_eq!(literal_size(r#""\x27""#), 6);
    }

    #[test]
    fn test_memory_size() {
        assert_eq!(memory_size(r#""""#), 0);
        assert_eq!(memory_size(r#""abc""#), 3);
        assert_eq!(memory_size(r#""aaa\"aaa""#), 7);
        assert_eq!(memory_size(r#""\x27""#), 1);
    }

    #[test]
    fn test_solve_example_1() {
        assert_eq!(solve(TEST_INPUT), 12);
    }

    #[test]
    fn test_encode() {
        assert_eq!(encode(r#""""#), r#""\"\"""#);
        assert_eq!(encode(r#""abc""#), r#""\"abc\"""#);
        assert_eq!(encode(r#""aaa\"aaa""#), r#""\"aaa\\\"aaa\"""#);
        assert_eq!(encode(r#""\x27""#), r#""\"\\x27\"""#);
    }

    #[test]
    fn test_solve_part2_example_1() {
        assert_eq!(solve_part2(TEST_INPUT), 19);
    }
}
