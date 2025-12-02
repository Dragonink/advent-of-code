//! Solution for the gold star of <https://adventofcode.com/2025/day/1>

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
/// Returns the number of times the dial points at 0 during or after any rotation.
#[inline]
pub fn rotate<I>(rotations: I) -> usize
where
	I: IntoIterator<Item = i32>,
{
	let mut current = 50;
	rotations
		.into_iter()
		.map(|rotation| {
			let new = current + rotation;
			let mut ret = (new / DIAL_SIZE).unsigned_abs() as usize;
			if new == 0 || (new < 0 && current != 0) {
				ret += 1;
			}

			current = new.rem_euclid(DIAL_SIZE);
			ret
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
		assert_eq!(ret, 6);
	}

	#[test]
	fn rotate_1000() {
		let ret = rotate("R1000".lines().map(aoc_2025_01::parse_line));
		assert_eq!(ret, 10);
	}
}
