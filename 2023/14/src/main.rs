use std::{io::Read, str::FromStr};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
enum Space {
	#[default]
	Empty,
	SquareRock,
	RoundRock,
}
impl TryFrom<u8> for Space {
	type Error = &'static str;

	fn try_from(c: u8) -> Result<Self, Self::Error> {
		match c {
			b'.' => Ok(Self::Empty),
			b'#' => Ok(Self::SquareRock),
			b'O' => Ok(Self::RoundRock),
			_ => Err("invalid space"),
		}
	}
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
struct Platform(Vec<Vec<Space>>);
impl FromStr for Platform {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		s.lines()
			.map(|s| s.bytes().map(Space::try_from).collect::<Result<_, _>>())
			.collect::<Result<_, _>>()
			.map(Self)
	}
}
impl Platform {
	fn tilt_north(&self) -> Self {
		let mut ret = self.clone();

		let mut northest_line = vec![0; self.0[0].len()];
		self.0.iter().enumerate().for_each(|(l, line)| {
			line.iter().enumerate().for_each(|(c, space)| {
				ret.0[l][c] = *space;
				match space {
					Space::Empty => {}
					Space::SquareRock => {
						northest_line[c] = l + 1;
					}
					Space::RoundRock => {
						ret.0[l][c] = Space::Empty;
						ret.0[northest_line[c]][c] = Space::RoundRock;
						northest_line[c] += 1;
					}
				}
			})
		});

		ret
	}

	#[cfg(feature = "p2")]
	fn tilt_south(&self) -> Self {
		let mut ret = self.clone();

		let mut southest_line = vec![self.0.len() - 1; self.0[0].len()];
		self.0.iter().enumerate().rev().for_each(|(l, line)| {
			line.iter().enumerate().for_each(|(c, space)| {
				ret.0[l][c] = *space;
				match space {
					Space::Empty => {}
					Space::SquareRock => {
						southest_line[c] = l.saturating_sub(1);
					}
					Space::RoundRock => {
						ret.0[l][c] = Space::Empty;
						ret.0[southest_line[c]][c] = Space::RoundRock;
						southest_line[c] = southest_line[c].saturating_sub(1);
					}
				}
			})
		});

		ret
	}

	#[cfg(feature = "p2")]
	fn tilt_west(&self) -> Self {
		let mut ret = self.clone();

		let mut westest_column = vec![0; self.0.len()];
		(0..self.0[0].len()).for_each(|c| {
			(0..self.0.len()).for_each(|l| {
				ret.0[l][c] = self.0[l][c];
				match self.0[l][c] {
					Space::Empty => {}
					Space::SquareRock => {
						westest_column[l] = c + 1;
					}
					Space::RoundRock => {
						ret.0[l][c] = Space::Empty;
						ret.0[l][westest_column[l]] = Space::RoundRock;
						westest_column[l] += 1;
					}
				}
			});
		});

		ret
	}

	#[cfg(feature = "p2")]
	fn tilt_east(&self) -> Self {
		let mut ret = self.clone();

		let mut eastest_column = vec![self.0[0].len() - 1; self.0.len()];
		(0..self.0[0].len()).rev().for_each(|c| {
			(0..self.0.len()).for_each(|l| {
				ret.0[l][c] = self.0[l][c];
				match self.0[l][c] {
					Space::Empty => {}
					Space::SquareRock => {
						eastest_column[l] = c.saturating_sub(1);
					}
					Space::RoundRock => {
						ret.0[l][c] = Space::Empty;
						ret.0[l][eastest_column[l]] = Space::RoundRock;
						eastest_column[l] = eastest_column[l].saturating_sub(1);
					}
				}
			});
		});

		ret
	}

	#[cfg(feature = "p2")]
	fn tilt_cycle(&self) -> Self {
		self.tilt_north().tilt_west().tilt_south().tilt_east()
	}
}

fn main() {
	#[cfg(not(feature = "p2"))]
	fn map_platform(platform: Platform) -> Platform {
		platform
	}

	#[cfg(feature = "p2")]
	fn map_platform(platform: Platform) -> Platform {
		const CYCLES: usize = 1_000_000_000;

		let mut cache = vec![platform];
		let mut loop_start = 0;
		for _ in 1..=CYCLES {
			let new_platform = cache.last().unwrap().tilt_cycle();
			if let Some(j) = cache
				.iter()
				.enumerate()
				.find_map(|(j, cached)| (*cached == new_platform).then_some(j))
			{
				loop_start = j;
				break;
			} else {
				cache.push(new_platform);
			}
		}

		cache[..loop_start]
			.iter()
			.chain(cache[loop_start..].iter().cycle())
			.nth(CYCLES)
			.unwrap()
			.clone()
	}

	let mut input = String::new();
	std::io::stdin().read_to_string(&mut input).unwrap();

	let platform = Platform::from_str(&input).unwrap().tilt_north();
	let len = platform.0.len();
	let sum: usize = map_platform(platform)
		.0
		.iter()
		.enumerate()
		.map(|(l, line)| {
			line.iter()
				.filter(|space| **space == Space::RoundRock)
				.count() * (len - l)
		})
		.sum();
	println!("{sum}");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn platform_from_str() {
		assert_eq!(
			Platform::from_str(include_str!("test.txt")),
			Ok(Platform(vec![
				vec![
					Space::RoundRock,
					Space::Empty,
					Space::Empty,
					Space::Empty,
					Space::Empty,
					Space::SquareRock,
					Space::Empty,
					Space::Empty,
					Space::Empty,
					Space::Empty,
				],
				vec![
					Space::RoundRock,
					Space::Empty,
					Space::RoundRock,
					Space::RoundRock,
					Space::SquareRock,
					Space::Empty,
					Space::Empty,
					Space::Empty,
					Space::Empty,
					Space::SquareRock,
				],
				vec![
					Space::Empty,
					Space::Empty,
					Space::Empty,
					Space::Empty,
					Space::Empty,
					Space::SquareRock,
					Space::SquareRock,
					Space::Empty,
					Space::Empty,
					Space::Empty,
				],
				vec![
					Space::RoundRock,
					Space::RoundRock,
					Space::Empty,
					Space::SquareRock,
					Space::RoundRock,
					Space::Empty,
					Space::Empty,
					Space::Empty,
					Space::Empty,
					Space::RoundRock,
				],
				vec![
					Space::Empty,
					Space::RoundRock,
					Space::Empty,
					Space::Empty,
					Space::Empty,
					Space::Empty,
					Space::Empty,
					Space::RoundRock,
					Space::SquareRock,
					Space::Empty,
				],
				vec![
					Space::RoundRock,
					Space::Empty,
					Space::SquareRock,
					Space::Empty,
					Space::Empty,
					Space::RoundRock,
					Space::Empty,
					Space::SquareRock,
					Space::Empty,
					Space::SquareRock,
				],
				vec![
					Space::Empty,
					Space::Empty,
					Space::RoundRock,
					Space::Empty,
					Space::Empty,
					Space::SquareRock,
					Space::RoundRock,
					Space::Empty,
					Space::Empty,
					Space::RoundRock,
				],
				vec![
					Space::Empty,
					Space::Empty,
					Space::Empty,
					Space::Empty,
					Space::Empty,
					Space::Empty,
					Space::Empty,
					Space::RoundRock,
					Space::Empty,
					Space::Empty,
				],
				vec![
					Space::SquareRock,
					Space::Empty,
					Space::Empty,
					Space::Empty,
					Space::Empty,
					Space::SquareRock,
					Space::SquareRock,
					Space::SquareRock,
					Space::Empty,
					Space::Empty,
				],
				vec![
					Space::SquareRock,
					Space::RoundRock,
					Space::RoundRock,
					Space::Empty,
					Space::Empty,
					Space::SquareRock,
					Space::Empty,
					Space::Empty,
					Space::Empty,
					Space::Empty,
				],
			]))
		);
	}

	#[test]
	fn platform_tilt_north() {
		assert_eq!(
			Platform::from_str(include_str!("test.txt"))
				.unwrap()
				.tilt_north(),
			Platform::from_str(
				"OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#...."
			)
			.unwrap()
		);
	}

	#[cfg(feature = "p2")]
	#[test]
	fn platform_tilt_cycle() {
		let mut platform = Platform::from_str(include_str!("test.txt")).unwrap();

		platform = platform.tilt_cycle();
		assert_eq!(
			platform,
			Platform::from_str(
				".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#...."
			)
			.unwrap()
		);

		platform = platform.tilt_cycle();
		assert_eq!(
			platform,
			Platform::from_str(
				".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O"
			)
			.unwrap()
		);

		platform = platform.tilt_cycle();
		assert_eq!(
			platform,
			Platform::from_str(
				".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O"
			)
			.unwrap()
		);
	}
}

#[cfg(all(test, stars))]
mod stars {
	#[test]
	fn p1() {
		assert!(true);
	}

	#[test]
	fn p2() {
		assert!(true);
	}
}
