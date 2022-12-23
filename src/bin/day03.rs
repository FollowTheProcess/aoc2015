/*
--- Day 3: Perfectly Spherical Houses in a Vacuum ---

Santa is delivering presents to an infinite two-dimensional grid of houses.

He begins by delivering a present to the house at his starting location, and then an elf at the North Pole calls
him via radio and tells him where to move next. Moves are always exactly one house to the north (^), south (v), east (>),
or west (<). After each move, he delivers another present to the house at his new location.

However, the elf back at the north pole has had a little too much eggnog, and so his directions are a little off,
and Santa ends up visiting some houses more than once.

How many houses receive at least one present?

For example:

> delivers presents to 2 houses: one at the starting location, and one to the east.
^>v< delivers presents to 4 houses in a square, including twice to the house at his starting/ending location.
^v^v^v^v^v delivers a bunch of presents to some very lucky children at only 2 houses.

--- Part Two ---

The next year, to speed up the process, Santa creates a robot version of himself, Robo-Santa, to deliver presents with him.

Santa and Robo-Santa start at the same location (delivering two presents to the same starting house), then take
turns moving based on instructions from the elf, who is eggnoggedly reading from the same script as the previous year.

This year, how many houses receive at least one present?

For example:

^v delivers presents to 3 houses, because Santa goes north, and then Robo-Santa goes south.
^>v< now delivers presents to 3 houses, and Santa and Robo-Santa end up back where they started.
^v^v^v^v^v now delivers presents to 11 houses, with Santa going one direction and Robo-Santa going the other.
*/

#![warn(clippy::pedantic)]

use std::collections::HashSet;

use anyhow::{anyhow, Result};

const PUZZLE_INPUT: &str = include_str!("../inputs/day03.txt");

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn move_north(&mut self) {
        self.y += 1;
    }

    fn move_south(&mut self) {
        self.y -= 1;
    }

    fn move_east(&mut self) {
        self.x += 1;
    }

    fn move_west(&mut self) {
        self.x -= 1;
    }
}

fn count_houses_with_a_present(raw: &str) -> Result<usize> {
    let mut visited: HashSet<Position> = HashSet::new();
    let mut pos = Position { x: 0, y: 0 };

    // Insert the starting position
    visited.insert(pos);

    for char in raw.trim().chars() {
        match char {
            '^' => pos.move_north(),
            'v' => pos.move_south(),
            '>' => pos.move_east(),
            '<' => pos.move_west(),
            _ => return Err(anyhow!("Invalid character: {char}")),
        }
        visited.insert(pos);
    }

    Ok(visited.len())
}

fn count_houses_with_robo_santa(raw: &str) -> Result<usize> {
    let mut visited: HashSet<Position> = HashSet::new();

    let mut santa = Position { x: 0, y: 0 };
    let mut robo_santa = Position { x: 0, y: 0 };

    // Insert starting position for both (same)
    visited.insert(santa);

    for (i, char) in raw.trim().chars().enumerate() {
        let pos = if i % 2 == 0 {
            &mut santa
        } else {
            &mut robo_santa
        };

        match char {
            '^' => pos.move_north(),
            'v' => pos.move_south(),
            '>' => pos.move_east(),
            '<' => pos.move_west(),
            _ => return Err(anyhow!("Invalid character: {char}")),
        }
        visited.insert(*pos);
    }

    Ok(visited.len())
}

fn main() -> Result<()> {
    let visited = count_houses_with_a_present(PUZZLE_INPUT)?;
    let visited_part2 = count_houses_with_robo_santa(PUZZLE_INPUT)?;

    println!("Part 1: {visited}");
    println!("Part 2: {visited_part2}");

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_houses_with_a_present() -> Result<()> {
        assert_eq!(count_houses_with_a_present(">")?, 2);
        assert_eq!(count_houses_with_a_present("^>v<")?, 4);
        assert_eq!(count_houses_with_a_present("^v^v^v^v^v")?, 2);
        Ok(())
    }

    #[test]
    fn test_count_houses_with_robo_santa() -> Result<()> {
        assert_eq!(count_houses_with_robo_santa("^v")?, 3);
        assert_eq!(count_houses_with_robo_santa("^>v<")?, 3);
        assert_eq!(count_houses_with_robo_santa("^v^v^v^v^v")?, 11);
        Ok(())
    }
}
