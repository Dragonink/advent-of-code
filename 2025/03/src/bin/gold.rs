//! Solution for the gold star of <https://adventofcode.com/2025/day/3>

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
	aoc_2025_03::find_max_joltage(12, batteries)
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
			987_654_321_111
		);
		assert_eq!(
			find_max_joltage(aoc_2025_03::parse_line("811111111111119")),
			811_111_111_119
		);
		assert_eq!(
			find_max_joltage(aoc_2025_03::parse_line("234234234234278")),
			434_234_234_278
		);
		assert_eq!(
			find_max_joltage(aoc_2025_03::parse_line("818181911112111")),
			888_911_112_111
		);
	}

	#[test]
	fn example() {
		let ret: usize = INPUT
			.lines()
			.map(|line| find_max_joltage(aoc_2025_03::parse_line(line)))
			.sum();
		assert_eq!(ret, 3_121_910_778_619);
	}
}
