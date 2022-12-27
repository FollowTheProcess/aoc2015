/*
--- Day 12: JSAbacusFramework.io ---

Santa's Accounting-Elves need help balancing the books after a recent order. Unfortunately, their accounting software uses a
peculiar storage format. That's where you come in.

They have a JSON document which contains a variety of things: arrays ([1,2,3]), objects ({"a":1, "b":2}), numbers, and strings.
Your first job is to simply find all of the numbers throughout the document and add them together.

For example:

[1,2,3] and {"a":2,"b":4} both have a sum of 6.
[[[3]]] and {"a":{"b":4},"c":-1} both have a sum of 3.
{"a":[-1,1]} and [-1,{"a":1}] both have a sum of 0.
[] and {} both have a sum of 0.

You will not encounter any strings containing numbers.

What is the sum of all numbers in the document?

--- Part Two ---

Uh oh - the Accounting-Elves have realized that they double-counted everything red.

Ignore any object (and all of its children) which has any property with the value "red".

Do this only for objects ({...}), not arrays ([...]).

[1,2,3] still has a sum of 6.

[1,{"c":"red","b":2},3] now has a sum of 4, because the middle object is ignored.

{"d":"red","e":[1,2,3,4],"f":5} now has a sum of 0, because the entire structure is ignored.

[1,"red",5] has a sum of 6, because "red" in an array has no effect.
*/

#![warn(clippy::pedantic)]

const PUZZLE_INPUT: &str = include_str!("../inputs/day12.txt");
const NUMERIC_REGEX: &str = r"(-?\d+)";

use anyhow::Result;
use regex::Regex;

fn sum_all_digits(input: &str) -> Result<i32> {
    let re = Regex::new(NUMERIC_REGEX)?;

    let sum = re
        .captures_iter(input)
        .map(|cap| cap[1].parse::<i32>().unwrap())
        .sum();

    Ok(sum)
}

fn main() -> Result<()> {
    let part1 = sum_all_digits(PUZZLE_INPUT.trim())?;
    println!("Part 1: {part1}");

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_digits() -> Result<()> {
        assert_eq!(sum_all_digits("1")?, 1);
        assert_eq!(sum_all_digits("1,2,3")?, 6);
        assert_eq!(sum_all_digits("[1,2,3]")?, 6);
        assert_eq!(sum_all_digits(r#"{"a":2,"b":4}"#)?, 6);
        assert_eq!(sum_all_digits("[[[3]]]")?, 3);
        assert_eq!(sum_all_digits(r#"{"a":{"b":4},"c":-1}"#)?, 3);
        assert_eq!(sum_all_digits("{}")?, 0);

        Ok(())
    }
}
