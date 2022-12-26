/*
--- Day 9: All in a Single Night ---

Every year, Santa manages to deliver all of his presents in a single night.

This year, however, he has some new locations to visit; his elves have provided him the distances between every pair of locations.
He can start and end at any two (different) locations he wants, but he must visit each location exactly once.

What is the shortest distance he can travel to achieve this?

For example, given the following distances:

London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141

The possible routes are therefore:

Dublin -> London -> Belfast = 982
London -> Dublin -> Belfast = 605
London -> Belfast -> Dublin = 659
Dublin -> Belfast -> London = 659
Belfast -> Dublin -> London = 605
Belfast -> London -> Dublin = 982

The shortest of these is London -> Dublin -> Belfast = 605, and so the answer is 605 in this example.

What is the distance of the shortest route?

--- Part Two ---

The next year, just to show off, Santa decides to take the route with the longest distance instead.

He can still start and end at any two (different) locations he wants, and he still must visit each location exactly once.

For example, given the distances above, the longest route would be 982 via (for example) Dublin -> London -> Belfast.

What is the distance of the longest route?
*/

#![warn(clippy::pedantic)]

use anyhow::{anyhow, Result};

use itertools::Itertools;
use std::collections::{HashMap, HashSet};

const PUZZLE_INPUT: &str = include_str!("../inputs/day09.txt");

#[derive(Debug, PartialEq, Eq)]
struct Leg<'a> {
    from: &'a str,
    to: &'a str,
    distance: u32,
}

impl<'a> Leg<'a> {
    fn parse(line: &'a str) -> Result<Self> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts.as_slice() {
            [from, "to", to, "=", distance] => {
                let distance = distance.parse()?;
                Ok(Self { from, to, distance })
            }
            _ => Err(anyhow!("Invalid line: {line}")),
        }
    }
}

#[derive(Debug)]
struct Solution {
    shortest: u32,
    longest: u32,
}

fn solve(input: &str) -> Result<Solution> {
    let mut distances: HashMap<(&str, &str), u32> = HashMap::new();
    let mut locations: HashSet<&str> = HashSet::new();

    for line in input.trim().lines() {
        let leg = Leg::parse(line)?;
        distances.insert((leg.from, leg.to), leg.distance);
        distances.insert((leg.to, leg.from), leg.distance);
        locations.insert(leg.from);
        locations.insert(leg.to);
    }

    let mut shortest_distance: Option<u32> = None;
    let mut longest_distance: Option<u32> = None;

    for permutation in locations.iter().permutations(locations.len()).unique() {
        let mut distance = 0;
        for (from, to) in permutation.iter().tuple_windows() {
            distance += distances
                .get(&(**from, **to))
                .ok_or_else(|| anyhow!("No distance found for leg {from} -> {to}"))?;
        }
        shortest_distance = Some(shortest_distance.map_or(distance, |d| d.min(distance)));
        longest_distance = Some(longest_distance.map_or(distance, |d| d.max(distance)));
    }

    Ok(Solution {
        shortest: shortest_distance.ok_or_else(|| anyhow!("No shortest distance found"))?,
        longest: longest_distance.ok_or_else(|| anyhow!("No longest distance found"))?,
    })
}

fn main() -> Result<()> {
    let solution = solve(PUZZLE_INPUT)?;

    println!("Part 1: {}", solution.shortest);
    println!("Part 2: {}", solution.longest);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141";

    #[test]
    fn test_parse_leg() -> Result<()> {
        assert_eq!(
            Leg::parse("London to Dublin = 464")?,
            Leg {
                from: "London",
                to: "Dublin",
                distance: 464
            }
        );
        assert_eq!(
            Leg::parse("London to Belfast = 518")?,
            Leg {
                from: "London",
                to: "Belfast",
                distance: 518
            }
        );
        assert_eq!(
            Leg::parse("Dublin to Belfast = 141")?,
            Leg {
                from: "Dublin",
                to: "Belfast",
                distance: 141
            }
        );
        Ok(())
    }

    #[test]
    fn test_shortest_route_example_1() -> Result<()> {
        assert_eq!(solve(TEST_INPUT)?.shortest, 605);
        Ok(())
    }

    #[test]
    fn test_longest_route_example_1() -> Result<()> {
        assert_eq!(solve(TEST_INPUT)?.longest, 982);
        Ok(())
    }
}
