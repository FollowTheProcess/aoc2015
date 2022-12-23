#![warn(clippy::pedantic)]

use anyhow::{anyhow, Result};
use std::env;
use std::process::Command;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let day: usize = args
        .get(1)
        .ok_or_else(|| anyhow!("No day provided"))?
        .parse()?;

    // Zero pad
    let day = format!("{day:02}");

    let cmd = Command::new("cargo")
        .args(["run", "--release", "--bin", &format!("day{day}")])
        .output()?;

    if !cmd.status.success() {
        return Err(anyhow!(
            "Failed to run day {day}: {}",
            String::from_utf8(cmd.stderr)?
        ));
    }

    let output = String::from_utf8(cmd.stdout)?;

    let message = if output.is_empty() {
        format!("Day {day} not solved!")
    } else {
        output.trim().to_string()
    };

    println!("{message}");

    Ok(())
}
