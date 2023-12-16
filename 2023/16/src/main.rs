use std::{
	collections::{HashSet, VecDeque},
	io::Read,
	str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
	Up,
	Left,
	Down,
	Right,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum LightApparatus {
	#[default]
	Empty,
	LeftMirror,
	RightMirror,
	VerticalSplitter,
	HorizontalSplitter,
}
impl TryFrom<u8> for LightApparatus {
	type Error = &'static str;

	fn try_from(c: u8) -> Result<Self, Self::Error> {
		match c {
			b'.' => Ok(Self::Empty),
			b'\\' => Ok(Self::LeftMirror),
			b'/' => Ok(Self::RightMirror),
			b'|' => Ok(Self::VerticalSplitter),
			b'-' => Ok(Self::HorizontalSplitter),
			_ => Err("invalid light apparatus"),
		}
	}
}
impl LightApparatus {
	fn pass_through(&self, from: Direction) -> Vec<Direction> {
		match (self, from) {
			(Self::Empty, _)
			| (Self::VerticalSplitter, Direction::Up | Direction::Down)
			| (Self::HorizontalSplitter, Direction::Left | Direction::Right) => vec![from],
			(Self::LeftMirror, Direction::Up) | (Self::RightMirror, Direction::Down) => {
				vec![Direction::Left]
			}
			(Self::LeftMirror, Direction::Left) | (Self::RightMirror, Direction::Right) => {
				vec![Direction::Up]
			}
			(Self::LeftMirror, Direction::Down) | (Self::RightMirror, Direction::Up) => {
				vec![Direction::Right]
			}
			(Self::LeftMirror, Direction::Right) | (Self::RightMirror, Direction::Left) => {
				vec![Direction::Down]
			}
			(Self::VerticalSplitter, Direction::Left | Direction::Right) => {
				vec![Direction::Up, Direction::Down]
			}
			(Self::HorizontalSplitter, Direction::Up | Direction::Down) => {
				vec![Direction::Left, Direction::Right]
			}
		}
	}
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Contraption(Vec<Vec<LightApparatus>>);
impl FromStr for Contraption {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		s.lines()
			.map(|s| {
				s.bytes()
					.map(LightApparatus::try_from)
					.collect::<Result<_, _>>()
			})
			.collect::<Result<_, _>>()
			.map(Self)
	}
}
impl Contraption {
	fn walk(&self, start: ((usize, usize), Direction)) -> HashSet<(usize, usize)> {
		let mut ret = HashSet::new();

		let mut positions = VecDeque::from([start]);
		while let Some((pos, dir)) = positions.pop_front() {
			if !ret.insert((pos, dir)) {
				continue;
			}
			self.0[pos.0][pos.1]
				.pass_through(dir)
				.into_iter()
				.for_each(|dir| match dir {
					Direction::Up if pos.0 > 0 => {
						positions.push_back(((pos.0 - 1, pos.1), dir));
					}
					Direction::Left if pos.1 > 0 => {
						positions.push_back(((pos.0, pos.1 - 1), dir));
					}
					Direction::Down if pos.0 < self.0.len() - 1 => {
						positions.push_back(((pos.0 + 1, pos.1), dir));
					}
					Direction::Right if pos.1 < self.0[0].len() - 1 => {
						positions.push_back(((pos.0, pos.1 + 1), dir));
					}
					_ => {}
				});
		}

		ret.into_iter().map(|(pos, _dir)| pos).collect()
	}
}

fn main() {
	#[cfg(not(feature = "p2"))]
	fn map_contraption(contraption: Contraption) -> usize {
		contraption.walk(((0, 0), Direction::Right)).len()
	}

	#[cfg(feature = "p2")]
	fn map_contraption(contraption: Contraption) -> usize {
		(0..contraption.0.len())
			.flat_map(|l| {
				[
					((l, 0), Direction::Right),
					((l, contraption.0[0].len() - 1), Direction::Left),
				]
			})
			.chain((0..contraption.0[0].len()).flat_map(|c| {
				[
					((0, c), Direction::Down),
					((contraption.0.len() - 1, c), Direction::Up),
				]
			}))
			.map(|start| contraption.walk(start).len())
			.max()
			.unwrap()
	}

	let mut input = String::new();
	std::io::stdin().read_to_string(&mut input).unwrap();

	let total: usize = map_contraption(Contraption::from_str(&input).unwrap());
	println!("{total}");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn contraption_from_str() {
		assert_eq!(
			Contraption::from_str(include_str!("test.txt")),
			Ok(Contraption(vec![
				vec![
					LightApparatus::Empty,
					LightApparatus::VerticalSplitter,
					LightApparatus::Empty,
					LightApparatus::Empty,
					LightApparatus::Empty,
					LightApparatus::LeftMirror,
					LightApparatus::Empty,
					LightApparatus::Empty,
					LightApparatus::Empty,
					LightApparatus::Empty,
				],
				vec![
					LightApparatus::VerticalSplitter,
					LightApparatus::Empty,
					LightApparatus::HorizontalSplitter,
					LightApparatus::Empty,
					LightApparatus::LeftMirror,
					LightApparatus::Empty,
					LightApparatus::Empty,
					LightApparatus::Empty,
					LightApparatus::Empty,
					LightApparatus::Empty,
				],
				vec![
					LightApparatus::Empty,
					LightApparatus::Empty,
					LightApparatus::Empty,
					LightApparatus::Empty,
					LightApparatus::Empty,
					LightApparatus::VerticalSplitter,
					LightApparatus::HorizontalSplitter,
					LightApparatus::Empty,
					LightApparatus::Empty,
					LightApparatus::Empty,
				],
				vec![
					LightApparatus::Empty,
					LightApparatus::Empty,
					LightApparatus::Empty,
					LightApparatus::Empty,
					LightApparatus::Empty,
					LightApparatus::Empty,
					LightApparatus::Empty,
					LightApparatus::Empty,
					LightApparatus::VerticalSplitter,
					LightApparatus::Empty,
				],
				vec![
					LightApparatus::Empty,
					LightApparatus::Empty,
					LightApparatus::Empty,
					LightApparatus::Empty,
					LightApparatus::Empty,
					LightApparatus::Empty,
					LightApparatus::Empty,
					LightApparatus::Empty,
					LightApparatus::Empty,
					LightApparatus::Empty,
				],
				vec![
					LightApparatus::Empty,
					LightApparatus::Empty,
					LightApparatus::Empty,
					LightApparatus::Empty,
					LightApparatus::Empty,
					LightApparatus::Empty,
					LightApparatus::Empty,
					LightApparatus::Empty,
					LightApparatus::Empty,
					LightApparatus::LeftMirror,
				],
				vec![
					LightApparatus::Empty,
					LightApparatus::Empty,
					LightApparatus::Empty,
					LightApparatus::Empty,
					LightApparatus::RightMirror,
					LightApparatus::Empty,
					LightApparatus::LeftMirror,
					LightApparatus::LeftMirror,
					LightApparatus::Empty,
					LightApparatus::Empty,
				],
				vec![
					LightApparatus::Empty,
					LightApparatus::HorizontalSplitter,
					LightApparatus::Empty,
					LightApparatus::HorizontalSplitter,
					LightApparatus::RightMirror,
					LightApparatus::Empty,
					LightApparatus::Empty,
					LightApparatus::VerticalSplitter,
					LightApparatus::Empty,
					LightApparatus::Empty,
				],
				vec![
					LightApparatus::Empty,
					LightApparatus::VerticalSplitter,
					LightApparatus::Empty,
					LightApparatus::Empty,
					LightApparatus::Empty,
					LightApparatus::Empty,
					LightApparatus::HorizontalSplitter,
					LightApparatus::VerticalSplitter,
					LightApparatus::Empty,
					LightApparatus::LeftMirror,
				],
				vec![
					LightApparatus::Empty,
					LightApparatus::Empty,
					LightApparatus::RightMirror,
					LightApparatus::RightMirror,
					LightApparatus::Empty,
					LightApparatus::VerticalSplitter,
					LightApparatus::Empty,
					LightApparatus::Empty,
					LightApparatus::Empty,
					LightApparatus::Empty,
				],
			]))
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
