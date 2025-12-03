//! Solution for the gold star of <https://adventofcode.com/2025/day/2>

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
/// Returns `true` if the ID is made of some sequence of digits repeated at least twice.
#[inline]
pub fn id_is_invalid(id: usize) -> bool {
	let repr = id.to_string();
	let repr = repr.as_bytes();

	for len in 1..=repr.len() / 2 {
		let mut chunks = repr.chunks_exact(len);
		let chunk = chunks.next().unwrap_or_else(|| unreachable!());
		if chunks.remainder().is_empty() && chunks.len() > 0 && chunks.all(|c| c == chunk) {
			return true;
		}
	}
	false
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
		assert_eq!(find_invalid_ids(95..=115), vec![99, 111]);
		assert_eq!(find_invalid_ids(998..=1012), vec![999, 1010]);
		assert_eq!(
			find_invalid_ids(1_188_511_880..=1_188_511_890),
			vec![1_188_511_885]
		);
		assert_eq!(find_invalid_ids(222_220..=222_224), vec![222_222]);
		assert_eq!(find_invalid_ids(1_698_522..=1_698_528), vec![]);
		assert_eq!(find_invalid_ids(446_443..=446_449), vec![446_446]);
		assert_eq!(find_invalid_ids(38_593_856..=38_593_862), vec![38_593_859]);
		assert_eq!(find_invalid_ids(565_653..=565_659), vec![565_656]);
		assert_eq!(
			find_invalid_ids(824_824_821..=824_824_827),
			vec![824_824_824]
		);
		assert_eq!(
			find_invalid_ids(2_121_212_118..=2_121_212_124),
			vec![2_121_212_121]
		);
	}

	#[test]
	fn example() {
		let ret: usize = aoc_2025_02::sum_invalid_ids(id_is_invalid, aoc_2025_02::parse(INPUT));
		assert_eq!(ret, 4_174_379_265);
	}
}
