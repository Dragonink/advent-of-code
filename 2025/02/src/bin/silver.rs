//! Solution for the silver star of <https://adventofcode.com/2025/day/2>

use std::io::Read as _;

#[allow(
	clippy::allow_attributes,
	clippy::missing_panics_doc,
	reason = "main function"
)]
fn main() {
	let mut input = String::new();
	std::io::stdin()
		.read_to_string(&mut input)
		.expect("stdin should be readable");

	println!(
		"{}",
		aoc_2025_02::sum_invalid_ids(id_is_invalid, aoc_2025_02::parse(&input))
	);
}

/// Checks if the given ID is invalid.
///
/// Returns `true` if the ID is made of some sequence of digits repeated twice.
fn id_is_invalid(id: usize) -> bool {
	let repr = id.to_string();

	let (first, last) = repr.split_at(repr.len() / 2);
	first == last
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
		let find_invalid_ids =
			|range| aoc_2025_02::find_invalid_ids(id_is_invalid, range).collect::<Vec<_>>();

		assert_eq!(find_invalid_ids(11..=22), vec![11, 22]);
		assert_eq!(find_invalid_ids(95..=115), vec![99]);
		assert_eq!(find_invalid_ids(998..=1012), vec![1010]);
		assert_eq!(
			find_invalid_ids(1_188_511_880..=1_188_511_890),
			vec![1_188_511_885]
		);
		assert_eq!(find_invalid_ids(222_220..=222_224), vec![222_222]);
		assert_eq!(find_invalid_ids(1_698_522..=1_698_528), vec![]);
		assert_eq!(find_invalid_ids(446_443..=446_449), vec![446_446]);
		assert_eq!(find_invalid_ids(38_593_856..=38_593_862), vec![38_593_859]);
	}

	#[test]
	fn example() {
		let ret: usize = aoc_2025_02::sum_invalid_ids(id_is_invalid, aoc_2025_02::parse(INPUT));
		assert_eq!(ret, 1_227_775_554);
	}
}
