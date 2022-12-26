/*
--- Day 7: Some Assembly Required ---

This year, Santa brought little Bobby Tables a set of wires and bitwise logic gates! Unfortunately,
little Bobby is a little under the recommended age range, and he needs help assembling the circuit.

Each wire has an identifier (some lowercase letters) and can carry a 16-bit signal (a number from 0 to 65535).
A signal is provided to each wire by a gate, another wire, or some specific value. Each wire can only get a signal from one source,
but can provide its signal to multiple destinations. A gate provides no signal until all of its inputs have a signal.

The included instructions booklet describes how to connect the parts together: x AND y -> z means to connect wires x and y to an AND gate,
and then connect its output to wire z.

For example:

123 -> x means that the signal 123 is provided to wire x.
x AND y -> z means that the bitwise AND of wire x and wire y is provided to wire z.
p LSHIFT 2 -> q means that the value from wire p is left-shifted by 2 and then provided to wire q.
NOT e -> f means that the bitwise complement of the value from wire e is provided to wire f.
Other possible gates include OR (bitwise OR) and RSHIFT (right-shift).
If, for some reason, you'd like to emulate the circuit instead, almost all programming languages
(for example, C, JavaScript, or Python) provide operators for these gates.

For example, here is a simple circuit:

123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i

After it is run, these are the signals on the wires:

d: 72
e: 507
f: 492
g: 114
h: 65412
i: 65079
x: 123
y: 456

In little Bobby's kit's instructions booklet (provided as your puzzle input), what signal is ultimately provided to wire a?
*/

#![warn(clippy::pedantic)]

use anyhow::{anyhow, Result};
use std::collections::HashMap;

const PUZZLE_INPUT: &str = include_str!("../inputs/day07.txt");

/// This took a lot of attempts and thinking!
fn solve(input: &str, wire: &str, override_b: bool) -> Result<u16> {
    let mut signals: Vec<&str> = input.trim().lines().collect();

    let mut results: HashMap<&str, u16> = HashMap::new();

    while !signals.is_empty() {
        // Queue for signals that couldn't be processed yet
        let mut todo: Vec<&str> = Vec::new();

        for signal in signals {
            let parts: Vec<&str> = signal.split_whitespace().collect();

            match parts.as_slice() {
                [value, "->", wire] => {
                    // Just sending a value to a wire
                    if let Ok(x) = value.parse() {
                        // The value is already a u16, can just insert it straight
                        // into results
                        if override_b && wire == &"b" {
                            // Override the value of b with the answer from part 1
                            results.insert(wire, 16076);
                            continue;
                        }
                        results.insert(wire, x);
                    } else if let Some(x) = results.get(value) {
                        // The value is a signal that has already been processed
                        results.insert(wire, *x);
                    } else {
                        // The value is a signal that hasn't been processed yet
                        todo.push(signal);
                    }
                }
                ["NOT", value, "->", wire] => {
                    // NOT gate
                    if let Some(x) = results.get(value) {
                        // We're "notting" a signal that has already been processed
                        results.insert(wire, !x);
                    } else {
                        // Not been processed yet
                        todo.push(signal);
                    }
                }
                [left, op, right, "->", wire] => {
                    if let Ok(x) = left.parse() {
                        // Left is already a u16
                        results.insert(left, x);
                    }
                    if let Ok(x) = right.parse() {
                        // Right is already a u16
                        results.insert(right, x);
                    }

                    if let Some(x) = results.get(left) {
                        if let Some(y) = results.get(right) {
                            // Both left and right have been processed
                            match *op {
                                "AND" => results.insert(wire, x & y),
                                "OR" => results.insert(wire, x | y),
                                "LSHIFT" => results.insert(wire, x << y),
                                "RSHIFT" => results.insert(wire, x >> y),
                                _ => unreachable!("Invalid op: {op}"),
                            };
                        } else {
                            // Left has been processed, right hasn't
                            todo.push(signal);
                        }
                    } else {
                        // Left hasn't been processed
                        todo.push(signal);
                    }
                }
                _ => unreachable!("Invalid signal: {signal}"),
            }
        }

        // Process the todo signals next time round
        signals = todo;
    }

    let answer = results
        .get(wire)
        .ok_or_else(|| anyhow!("Wire not found: {wire}"))?;

    Ok(*answer)
}

fn main() -> Result<()> {
    let part1 = solve(PUZZLE_INPUT, "a", false)?;
    let part2 = solve(PUZZLE_INPUT, "a", true)?;

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i";

    #[test]
    fn test_solve_example() -> Result<()> {
        assert_eq!(solve(TEST_INPUT, "d", false)?, 72);
        assert_eq!(solve(TEST_INPUT, "e", false)?, 507);
        assert_eq!(solve(TEST_INPUT, "f", false)?, 492);
        assert_eq!(solve(TEST_INPUT, "g", false)?, 114);
        assert_eq!(solve(TEST_INPUT, "h", false)?, 65412);
        assert_eq!(solve(TEST_INPUT, "i", false)?, 65079);
        assert_eq!(solve(TEST_INPUT, "x", false)?, 123);
        assert_eq!(solve(TEST_INPUT, "y", false)?, 456);

        Ok(())
    }
}
