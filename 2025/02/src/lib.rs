//! Common code for the solutions of <https://adventofcode.com/2025/day/2>

use std::ops::RangeInclusive;

/// Parses the given input.
///
/// # Panics
/// Panics if the parsing fails.
#[inline]
pub fn parse(input: &str) -> impl Iterator<Item = RangeInclusive<usize>> {
	input.trim().split(',').map(|range| {
		let mut bounds = range.split('-').map(|bound| {
			bound
				.parse()
				.unwrap_or_else(|err| panic!("{bound:?} could not be parsed: {err}"))
		});
		let start = bounds
			.next()
			.unwrap_or_else(|| panic!("{range:?} does not contain a start bound"));
		let end = bounds
			.next()
			.unwrap_or_else(|| panic!("{range:?} does not contain an end bound"));

		start..=end
	})
}

/// Sums all invalid IDs contained in the given ranges.
#[inline]
pub fn sum_invalid_ids<F, I>(filter: F, ranges: I) -> usize
where
	F: Fn(usize) -> bool,
	I: IntoIterator<Item = RangeInclusive<usize>>,
{
	ranges
		.into_iter()
		.flat_map(|range| find_invalid_ids(&filter, range))
		.sum()
}

/// Finds invalid IDs using the given filter in the given range.
#[inline]
pub fn find_invalid_ids<F>(filter: F, range: RangeInclusive<usize>) -> impl Iterator<Item = usize>
where
	F: Fn(usize) -> bool,
{
	range.filter(move |id| filter(*id))
}
