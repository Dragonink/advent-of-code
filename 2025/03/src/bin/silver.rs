//! Solution for the silver star of <https://adventofcode.com/2025/day/3>

use std::cmp::Reverse;

#[allow(
	clippy::allow_attributes,
	clippy::missing_panics_doc,
	reason = "main function"
)]
fn main() {
	let stdin = std::io::stdin();

	println!(
		"{}",
		stdin
			.lines()
			.map(|res| find_max_joltage(aoc_2025_03::parse_line(
				&res.expect("input line should be readable")
			)))
			.sum::<usize>()
	);
}

/// Finds the maximum joltage in the given battery bank.
///
/// # Panics
/// Panics if the given bank has less than 2 batteries.
fn find_max_joltage<I>(batteries: I) -> usize
where
	I: IntoIterator<Item = usize>,
{
	let batteries: Vec<_> = batteries.into_iter().collect();

	let (i, tens) = batteries[..batteries.len() - 1]
		.iter()
		.copied()
		.enumerate()
		.max_by_key(|&(i, jolt)| (jolt, Reverse(i)))
		.expect("bank should have a maximum battery");
	let units = batteries[i + 1..]
		.iter()
		.copied()
		.max()
		.expect("bank should have a maximum battery");

	units + 10 * tens
}

#[cfg(test)]
#[allow(
	clippy::allow_attributes,
	clippy::missing_panics_doc,
	clippy::unwrap_used,
	reason = "test functions"
)]
mod tests {
	use super::*;

	const INPUT: &str = include_str!("../../example.txt");

	#[test]
	fn detailed_example() {
		assert_eq!(
			find_max_joltage(aoc_2025_03::parse_line("987654321111111")),
			98
		);
		assert_eq!(
			find_max_joltage(aoc_2025_03::parse_line("811111111111119")),
			89
		);
		assert_eq!(
			find_max_joltage(aoc_2025_03::parse_line("234234234234278")),
			78
		);
		assert_eq!(
			find_max_joltage(aoc_2025_03::parse_line("818181911112111")),
			92
		);
	}

	#[test]
	fn example() {
		let ret: usize = INPUT
			.lines()
			.map(|line| find_max_joltage(aoc_2025_03::parse_line(line)))
			.sum();
		assert_eq!(ret, 357);
	}
}
