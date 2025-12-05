//! Common code for the solutions of <https://adventofcode.com/2025/day/3>

/// Parses the given line from the input.
#[inline]
pub fn parse_line(line: &str) -> impl Iterator<Item = usize> {
	line.trim()
		.as_bytes()
		.iter()
		.copied()
		.map(|b| (b - b'0').into())
}
