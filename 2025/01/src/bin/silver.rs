//! Solution for the silver star of <https://adventofcode.com/2025/day/1>

use aoc_2025_01::DIAL_SIZE;

#[allow(
	clippy::allow_attributes,
	clippy::missing_panics_doc,
	reason = "main function"
)]
fn main() {
	let stdin = std::io::stdin();

	println!("{}", rotate(aoc_2025_01::parse(stdin.lock())));
}

/// Rotates the dial according to the given instructions.
///
/// Returns the number of times the dial is left pointing at 0 after any rotation.
fn rotate<I>(rotations: I) -> usize
where
	I: IntoIterator<Item = i32>,
{
	let mut current = 50;
	rotations
		.into_iter()
		.map(|rotation| {
			current = (current + rotation).rem_euclid(DIAL_SIZE);
			usize::from(current == 0)
		})
		.sum()
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
	fn example() {
		let ret = rotate(INPUT.lines().map(aoc_2025_01::parse_line));
		assert_eq!(ret, 3);
	}
}
