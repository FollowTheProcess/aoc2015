/*
--- Day 2: I Was Told There Would Be No Math ---

The elves are running low on wrapping paper, and so they need to submit an order for more.
They have a list of the dimensions (length l, width w, and height h) of each present, and only want to order exactly as much as they need.

Fortunately, every present is a box (a perfect right rectangular prism), which makes calculating
the required wrapping paper for each gift a little easier: find the surface area of the box, which is 2*l*w + 2*w*h + 2*h*l.
The elves also need a little extra paper for each present: the area of the smallest side.

For example:

A present with dimensions 2x3x4 requires 2*6 + 2*12 + 2*8 = 52 square feet of wrapping paper plus 6 square feet of slack, for a total of 58 square feet.
A present with dimensions 1x1x10 requires 2*1 + 2*10 + 2*10 = 42 square feet of wrapping paper plus 1 square foot of slack, for a total of 43 square feet.

All numbers in the elves' list are in feet. How many total square feet of wrapping paper should they order?

--- Part Two ---

The elves are also running low on ribbon. Ribbon is all the same width, so they only have to worry about the length they
need to order, which they would again like to be exact.

The ribbon required to wrap a present is the shortest distance around its sides, or the smallest perimeter of any one face.
Each present also requires a bow made out of ribbon as well; the feet of ribbon required for the perfect bow is equal to the cubic feet of volume of the present.
Don't ask how they tie the bow, though; they'll never tell.

For example:

A present with dimensions 2x3x4 requires 2+2+3+3 = 10 feet of ribbon to wrap the present plus 2*3*4 = 24 feet of ribbon for the bow, for a total of 34 feet.
A present with dimensions 1x1x10 requires 1+1+1+1 = 4 feet of ribbon to wrap the present plus 1*1*10 = 10 feet of ribbon for the bow, for a total of 14 feet.

How many total feet of ribbon should they order?
*/

#![warn(clippy::pedantic)]

use anyhow::{anyhow, Result};

const PUZZLE_INPUT: &str = include_str!("../inputs/day02.txt");

#[derive(Debug, Eq, PartialEq)]
struct Present {
    length: u32,
    width: u32,
    height: u32,
}

impl Present {
    fn parse(line: &str) -> Result<Present> {
        let parts: Vec<&str> = line.trim().split('x').collect();
        if parts.len() != 3 {
            return Err(anyhow!("Invalid present: {}", line));
        }

        let length = parts[0].parse()?;
        let width = parts[1].parse()?;
        let height = parts[2].parse()?;

        Ok(Present {
            length,
            width,
            height,
        })
    }

    fn paper_required(&self) -> u32 {
        let side1 = self.length * self.width;
        let side2 = self.width * self.height;
        let side3 = self.height * self.length;

        let slack = side1.min(side2).min(side3);

        2 * side1 + 2 * side2 + 2 * side3 + slack
    }

    fn ribbon_required(&self) -> u32 {
        let volume = self.length * self.width * self.height;

        // Perimeters of faces
        let end_face = 2 * self.width + 2 * self.height;
        let side_face = 2 * self.length + 2 * self.height;
        let top_face = 2 * self.length + 2 * self.width;

        let smallest = end_face.min(side_face).min(top_face);

        smallest + volume
    }
}

fn main() -> Result<()> {
    let mut total_paper_required = 0;
    let mut total_ribbon_required = 0;
    for line in PUZZLE_INPUT.trim().lines() {
        let present = Present::parse(line)?;
        total_paper_required += present.paper_required();
        total_ribbon_required += present.ribbon_required();
    }

    println!("Part 1: {total_paper_required}");
    println!("Part 2: {total_ribbon_required}");
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_present_parse() -> Result<()> {
        let present = Present::parse("2x3x4")?;
        let want = Present {
            length: 2,
            width: 3,
            height: 4,
        };

        assert_eq!(present, want);
        Ok(())
    }

    #[test]
    fn test_paper_required() -> Result<()> {
        let present = Present {
            length: 2,
            width: 3,
            height: 4,
        };
        assert_eq!(present.paper_required(), 58);

        let present = Present {
            length: 1,
            width: 1,
            height: 10,
        };
        assert_eq!(present.paper_required(), 43);
        Ok(())
    }

    #[test]
    fn test_ribbon_required() -> Result<()> {
        let present = Present {
            length: 2,
            width: 3,
            height: 4,
        };
        assert_eq!(present.ribbon_required(), 34);

        let present = Present {
            length: 1,
            width: 1,
            height: 10,
        };
        assert_eq!(present.ribbon_required(), 14);
        Ok(())
    }
}
