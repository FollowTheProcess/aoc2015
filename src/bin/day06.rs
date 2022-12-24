/*
--- Day 6: Probably a Fire Hazard ---

Because your neighbors keep defeating you in the holiday house decorating contest year after year,
you've decided to deploy one million lights in a 1000x1000 grid.

Furthermore, because you've been especially nice this year, Santa has mailed you instructions on how to display the ideal lighting configuration.

Lights in your grid are numbered from 0 to 999 in each direction; the lights at each corner are at 0,0, 0,999, 999,999, and 999,0.
The instructions include whether to turn on, turn off, or toggle various inclusive ranges given as coordinate pairs. Each coordinate
pair represents opposite corners of a rectangle, inclusive; a coordinate pair like 0,0 through 2,2 therefore refers to 9 lights in a 3x3 square.
The lights all start turned off.

To defeat your neighbors this year, all you have to do is set up your lights by doing the instructions Santa sent you in order.

For example:

turn on 0,0 through 999,999 would turn on (or leave on) every light.
toggle 0,0 through 999,0 would toggle the first line of 1000 lights, turning off the ones that were on, and turning on the ones that were off.
turn off 499,499 through 500,500 would turn off (or leave off) the middle four lights.

After following the instructions, how many lights are lit?

--- Part Two ---

You just finish implementing your winning light pattern when you realize you mistranslated Santa's message from Ancient Nordic Elvish.

The light grid you bought actually has individual brightness controls; each light can have a brightness of zero or more. The lights all start at zero.

The phrase turn on actually means that you should increase the brightness of those lights by 1.

The phrase turn off actually means that you should decrease the brightness of those lights by 1, to a minimum of zero.

The phrase toggle actually means that you should increase the brightness of those lights by 2.

What is the total brightness of all lights combined after following Santa's instructions?

For example:

turn on 0,0 through 0,0 would increase the total brightness by 1.
toggle 0,0 through 999,999 would increase the total brightness by 2000000.
*/

#![warn(clippy::pedantic)]

use anyhow::{anyhow, Result};
use std::collections::HashMap;

const PUZZLE_INPUT: &str = include_str!("../inputs/day06.txt");

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn parse(raw: &str) -> Result<Point> {
        let mut parts = raw.split(',');
        let x = parts.next().ok_or_else(|| anyhow!("No x"))?.parse()?;
        let y = parts.next().ok_or_else(|| anyhow!("No y"))?.parse()?;
        Ok(Point { x, y })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Instruction {
    TurnOn { from: Point, to: Point },
    TurnOff { from: Point, to: Point },
    Toggle { from: Point, to: Point },
}

impl Instruction {
    fn parse(line: &str) -> Result<Instruction> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        match &parts[..] {
            ["turn", "on", from, "through", to] => Ok(Instruction::TurnOn {
                from: Point::parse(from)?,
                to: Point::parse(to)?,
            }),
            ["turn", "off", from, "through", to] => Ok(Instruction::TurnOff {
                from: Point::parse(from)?,
                to: Point::parse(to)?,
            }),
            ["toggle", from, "through", to] => Ok(Instruction::Toggle {
                from: Point::parse(from)?,
                to: Point::parse(to)?,
            }),
            _ => Err(anyhow!("Unknown instruction: {}", line)),
        }
    }

    fn apply(&self, lights: &mut HashMap<Point, bool>) {
        match self {
            Instruction::TurnOn { from, to } => {
                for x in from.x..=to.x {
                    for y in from.y..=to.y {
                        lights.insert(Point { x, y }, true);
                    }
                }
            }
            Instruction::TurnOff { from, to } => {
                for x in from.x..=to.x {
                    for y in from.y..=to.y {
                        lights.insert(Point { x, y }, false);
                    }
                }
            }
            Instruction::Toggle { from, to } => {
                for x in from.x..=to.x {
                    for y in from.y..=to.y {
                        let point = Point { x, y };
                        let current = lights.get(&point).unwrap_or(&false);
                        lights.insert(point, !current);
                    }
                }
            }
        }
    }

    fn apply_part2(&self, lights: &mut HashMap<Point, u32>) {
        match self {
            Instruction::TurnOn { from, to } => {
                for x in from.x..=to.x {
                    for y in from.y..=to.y {
                        let point = Point { x, y };
                        let current = lights.get(&point).unwrap_or(&0);
                        lights.insert(point, current + 1);
                    }
                }
            }
            Instruction::TurnOff { from, to } => {
                for x in from.x..=to.x {
                    for y in from.y..=to.y {
                        let point = Point { x, y };
                        let current = lights.get(&point).unwrap_or(&0);
                        lights.insert(point, current.saturating_sub(1));
                    }
                }
            }
            Instruction::Toggle { from, to } => {
                for x in from.x..=to.x {
                    for y in from.y..=to.y {
                        let point = Point { x, y };
                        let current = lights.get(&point).unwrap_or(&0);
                        lights.insert(point, current + 2);
                    }
                }
            }
        }
    }
}

fn build_lights() -> HashMap<Point, bool> {
    let mut lights = HashMap::new();
    for x in 0..1000 {
        for y in 0..1000 {
            lights.insert(Point { x, y }, false);
        }
    }
    lights
}

fn build_lights_part2() -> HashMap<Point, u32> {
    let mut lights = HashMap::new();
    for x in 0..1000 {
        for y in 0..1000 {
            lights.insert(Point { x, y }, 0);
        }
    }
    lights
}

fn main() -> Result<()> {
    let mut lights = build_lights();
    for line in PUZZLE_INPUT.trim().lines() {
        let instruction = Instruction::parse(line)?;
        instruction.apply(&mut lights);
    }
    let lit = lights.values().filter(|&&lit| lit).count();

    let mut lights2 = build_lights_part2();
    for line in PUZZLE_INPUT.trim().lines() {
        let instruction = Instruction::parse(line)?;
        instruction.apply_part2(&mut lights2);
    }

    let brightness: u32 = lights2.values().sum();

    println!("Part 1: {lit}");
    println!("Part 2: {brightness}");
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_point_parse() -> Result<()> {
        assert_eq!(Point::parse("0,0")?, Point { x: 0, y: 0 });
        assert_eq!(Point::parse("660,55")?, Point { x: 660, y: 55 });
        Ok(())
    }

    #[test]
    fn test_instruction_parse() -> Result<()> {
        assert_eq!(
            Instruction::parse("turn on 0,0 through 999,999")?,
            Instruction::TurnOn {
                from: Point { x: 0, y: 0 },
                to: Point { x: 999, y: 999 }
            }
        );
        assert_eq!(
            Instruction::parse("toggle 0,0 through 999,0")?,
            Instruction::Toggle {
                from: Point { x: 0, y: 0 },
                to: Point { x: 999, y: 0 }
            }
        );
        assert_eq!(
            Instruction::parse("turn off 499,499 through 500,500")?,
            Instruction::TurnOff {
                from: Point { x: 499, y: 499 },
                to: Point { x: 500, y: 500 }
            }
        );
        Ok(())
    }

    #[test]
    fn test_instruction_apply() -> Result<()> {
        let mut lights = build_lights();
        Instruction::parse("turn on 0,0 through 999,999")?.apply(&mut lights);
        assert_eq!(lights.len(), 1_000_000);
        assert_eq!(lights.values().filter(|&&v| v).count(), 1_000_000);

        let mut lights = build_lights();
        Instruction::parse("toggle 0,0 through 999,0")?.apply(&mut lights);
        assert_eq!(lights.len(), 1_000_000);
        assert_eq!(lights.values().filter(|&&v| v).count(), 1_000);

        let mut lights = build_lights();
        Instruction::parse("turn off 499,499 through 500,500")?.apply(&mut lights);
        assert_eq!(lights.len(), 1_000_000);
        assert_eq!(lights.values().filter(|&&v| v).count(), 0);

        Ok(())
    }
}
