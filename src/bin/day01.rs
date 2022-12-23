/*
--- Day 1: Not Quite Lisp ---

Santa was hoping for a white Christmas, but his weather machine's "snow" function is powered by stars, and he's fresh out!
To save Christmas, he needs you to collect fifty stars by December 25th.

Collect stars by helping Santa solve puzzles. Two puzzles will be made available on each day in the Advent calendar;
the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

Here's an easy puzzle to warm you up.

Santa is trying to deliver presents in a large apartment building, but he can't find the right floor -
the directions he got are a little confusing. He starts on the ground floor (floor 0) and then follows the instructions one character at a time.

An opening parenthesis, (, means he should go up one floor, and a closing parenthesis, ), means he should go down one floor.

The apartment building is very tall, and the basement is very deep; he will never find the top or bottom floors.

For example:

(()) and ()() both result in floor 0.
((( and (()(()( both result in floor 3.
))((((( also results in floor 3.
()) and ))( both result in floor -1 (the first basement level).
))) and )())()) both result in floor -3.

To what floor do the instructions take Santa?

--- Part Two ---

Now, given the same instructions, find the position of the first character that causes him to enter the
basement (floor -1). The first character in the instructions has position 1, the second character has position 2, and so on.

For example:

) causes him to enter the basement at character position 1.
()()) causes him to enter the basement at character position 5.

What is the position of the character that causes Santa to first enter the basement?
*/

#![warn(clippy::pedantic)]

use anyhow::{anyhow, Result};

const PUZZLE_INPUT: &str = include_str!("../inputs/day01.txt");

fn main() -> Result<()> {
    let floor = get_floor(PUZZLE_INPUT)?;

    let first_in_basement = first_char_to_enter_basement(PUZZLE_INPUT)?;

    println!("Part 1: {floor}");
    println!("Part 2: {first_in_basement}");
    Ok(())
}

fn get_floor(s: &str) -> Result<i32> {
    let mut floor = 0;
    for char in s.chars() {
        match char {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => return Err(anyhow!("Invalid character: '{char}'")),
        }
    }
    Ok(floor)
}

fn first_char_to_enter_basement(s: &str) -> Result<usize> {
    let mut floor = 0;
    for (i, char) in s.chars().enumerate() {
        match char {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => return Err(anyhow!("Invalid character: '{char}'")),
        }
        if floor == -1 {
            return Ok(i + 1);
        }
    }
    Err(anyhow!("Santa never entered the basement"))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_floor() -> Result<()> {
        assert_eq!(get_floor("(())")?, 0);
        assert_eq!(get_floor("()()")?, 0);
        assert_eq!(get_floor("(((")?, 3);
        assert_eq!(get_floor("(()(()(")?, 3);
        assert_eq!(get_floor("))(((((")?, 3);
        assert_eq!(get_floor("())")?, -1);
        assert_eq!(get_floor("))(")?, -1);
        assert_eq!(get_floor("))(")?, -1);
        assert_eq!(get_floor(")))")?, -3);
        assert_eq!(get_floor(")())())")?, -3);
        Ok(())
    }

    #[test]
    fn test_first_char_to_enter_basement() -> Result<()> {
        assert_eq!(first_char_to_enter_basement(")")?, 1);
        assert_eq!(first_char_to_enter_basement("()())")?, 5);
        Ok(())
    }
}
