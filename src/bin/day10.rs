/*
--- Day 10: Elves Look, Elves Say ---

Today, the Elves are playing a game called look-and-say. They take turns making sequences by reading aloud the previous
sequence and using that reading as the next sequence.

For example, 211 is read as "one two, two ones", which becomes 1221 (1 2, 2 1s).

Look-and-say sequences are generated iteratively, using the previous value as input for the next step.
For each step, take the previous value, and replace each run of digits (like 111) with the number of digits (3)
followed by the digit itself (1).

For example:

1 becomes 11 (1 copy of digit 1).
11 becomes 21 (2 copies of digit 1).
21 becomes 1211 (one 2 followed by one 1).
1211 becomes 111221 (one 1, one 2, and two 1s).
111221 becomes 312211 (three 1s, two 2s, and one 1).

Starting with the digits in your puzzle input, apply this process 40 times. What is the length of the result?

--- Part Two ---

Neat, right? You might also enjoy hearing John Conway talking about this sequence (that's Conway of Conway's Game of Life fame).

Now, starting again with the digits in your puzzle input, apply this process 50 times. What is the length of the new result?
*/

#![warn(clippy::pedantic)]

const PUZZLE_INPUT: &str = include_str!("../inputs/day10.txt");

fn main() {
    let part1 = look_and_say(PUZZLE_INPUT, 40);
    let part2 = look_and_say(PUZZLE_INPUT, 50);

    println!("Part 1: {}", part1.len());
    println!("Part 2: {}", part2.len());
}

fn look_and_say(input: &str, n: u32) -> String {
    let mut result = String::new();
    let mut chars = input.chars();
    let mut current_char = chars.next().unwrap();
    let mut current_count = 1;

    for c in chars {
        if c == current_char {
            current_count += 1;
        } else {
            result.push_str(&current_count.to_string());
            result.push(current_char);
            current_char = c;
            current_count = 1;
        }
    }

    result.push_str(&current_count.to_string());
    result.push(current_char);

    if n == 1 {
        result
    } else {
        look_and_say(&result, n - 1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_look_and_say() {
        assert_eq!(look_and_say("1", 1), "11");
        assert_eq!(look_and_say("11", 1), "21");
        assert_eq!(look_and_say("21", 1), "1211");
        assert_eq!(look_and_say("1211", 1), "111221");
        assert_eq!(look_and_say("111221", 1), "312211");
    }

    #[test]
    fn test_look_and_say_40() {
        assert_eq!(look_and_say("1", 5), "312211");
    }
}
