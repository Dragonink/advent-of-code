//! Common code for the solutions of <https://adventofcode.com/2025/day/1>

use std::io::BufRead;

/// Parses the given input.
///
/// # Panics
/// Panics if the parsing fails.
#[inline]
pub fn parse<R: BufRead>(input: R) -> impl Iterator<Item = i32> {
	input
		.lines()
		.map(|res| parse_line(&res.expect("input line should be readable")))
}

/// Parses the given line from the input.
///
/// # Panics
/// Panics if the parsing fails.
#[inline]
pub fn parse_line(line: &str) -> i32 {
	let (direction, distance) = line.split_at(1);

	let distance: i32 = distance
		.parse()
		.unwrap_or_else(|err| panic!("{distance:?} could not be parsed: {err}"));
	match direction {
		"L" => -distance,
		"R" => distance,
		_ => panic!("{direction:?} is an invalid rotation direction"),
	}
}

/// Number of values on the dial
pub const DIAL_SIZE: i32 = 100;
