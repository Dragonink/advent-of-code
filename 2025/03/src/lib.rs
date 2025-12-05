//! Common code for the solutions of <https://adventofcode.com/2025/day/3>

use std::cmp::Reverse;

/// Parses the given line from the input.
#[inline]
pub fn parse_line(line: &str) -> impl Iterator<Item = usize> {
	line.trim()
		.as_bytes()
		.iter()
		.copied()
		.map(|b| (b - b'0').into())
}

/// Finds the maximum joltage in the given battery bank.
///
/// The parameter `n` is the number of batteries to turn on.
///
/// # Panics
/// Panics if the given bank has less than `n` batteries.
#[inline]
pub fn find_max_joltage<I>(n: u32, batteries: I) -> usize
where
	I: IntoIterator<Item = usize>,
{
	let batteries: Vec<_> = batteries.into_iter().collect();
	assert!(
		batteries.len() >= n as usize,
		"bank should have at least {n} batteries"
	);

	let mut start = 0;
	(0..n)
		.map(|p| {
			let (new_start, jolt) = batteries[start..=batteries.len() - (n - p) as usize]
				.iter()
				.copied()
				.enumerate()
				.max_by_key(|&(i, jolt)| (jolt, Reverse(i)))
				.unwrap_or_else(|| unreachable!());

			start += new_start + 1;
			jolt * 10_usize.pow(n - p - 1)
		})
		.sum()
}
