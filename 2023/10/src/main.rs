use std::{
	fmt::{self, Debug, Formatter, Write},
	io::Read,
	ops::{BitOr, BitOrAssign, Index},
	str::FromStr,
};

#[derive(Default, Clone, Copy, PartialEq, Eq)]
struct Pipe {
	up: bool,
	right: bool,
	down: bool,
	left: bool,
}
impl Pipe {
	const UP: Self = Self {
		up: true,
		right: false,
		down: false,
		left: false,
	};
	const RIGHT: Self = Self {
		up: false,
		right: true,
		down: false,
		left: false,
	};
	const DOWN: Self = Self {
		up: false,
		right: false,
		down: true,
		left: false,
	};
	const LEFT: Self = Self {
		up: false,
		right: false,
		down: false,
		left: true,
	};

	const NONE: Self = Self {
		up: false,
		right: false,
		down: false,
		left: false,
	};
	const START: Self = Self {
		up: true,
		right: true,
		down: true,
		left: true,
	};
}
impl Debug for Pipe {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_char(match self {
			Self {
				up: false,
				right: false,
				down: false,
				left: false,
			} => '.',
			Self {
				up: true,
				right: true,
				down: false,
				left: false,
			} => '└',
			Self {
				up: true,
				right: false,
				down: true,
				left: false,
			} => '│',
			Self {
				up: true,
				right: false,
				down: false,
				left: true,
			} => '┘',
			Self {
				up: false,
				right: true,
				down: true,
				left: false,
			} => '┌',
			Self {
				up: false,
				right: true,
				down: false,
				left: true,
			} => '─',
			Self {
				up: false,
				right: false,
				down: true,
				left: true,
			} => '┐',
			Self {
				up: true,
				right: true,
				down: true,
				left: true,
			} => 'S',
			_ => unreachable!(),
		})
	}
}
impl BitOr for Pipe {
	type Output = Self;

	#[inline]
	fn bitor(self, rhs: Self) -> Self::Output {
		Self {
			up: self.up || rhs.up,
			right: self.right || rhs.right,
			down: self.down || rhs.down,
			left: self.left || rhs.left,
		}
	}
}
impl BitOrAssign for Pipe {
	#[inline]
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}
impl TryFrom<u8> for Pipe {
	type Error = &'static str;

	fn try_from(c: u8) -> Result<Self, Self::Error> {
		match c {
			b'|' => Ok(Self::UP | Self::DOWN),
			b'-' => Ok(Self::LEFT | Self::RIGHT),
			b'L' => Ok(Self::UP | Self::RIGHT),
			b'J' => Ok(Self::UP | Self::LEFT),
			b'7' => Ok(Self::LEFT | Self::DOWN),
			b'F' => Ok(Self::RIGHT | Self::DOWN),
			b'.' => Ok(Self::NONE),
			b'S' => Ok(Self::START),
			_ => Err("invalid pipe"),
		}
	}
}

#[derive(Default, Clone, PartialEq, Eq)]
struct Map(Vec<Vec<Pipe>>);
impl FromStr for Map {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		s.lines()
			.map(|s| {
				s.as_bytes()
					.iter()
					.copied()
					.map(Pipe::try_from)
					.collect::<Result<_, _>>()
			})
			.collect::<Result<_, _>>()
			.map(Self)
	}
}
impl Debug for Map {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		self.0.iter().try_for_each(|l| {
			l.iter().try_for_each(|c| Debug::fmt(c, f))?;
			writeln!(f)
		})
	}
}
impl Index<(usize, usize)> for Map {
	type Output = Pipe;

	#[inline]
	fn index(&self, (l, c): (usize, usize)) -> &Self::Output {
		&self.0[l][c]
	}
}
impl Map {
	fn start(&self) -> (usize, usize) {
		self.0
			.iter()
			.enumerate()
			.flat_map(|(l, line)| line.iter().enumerate().map(move |(c, pipe)| (l, c, pipe)))
			.find_map(|(l, c, pipe)| (*pipe == Pipe::START).then_some((l, c)))
			.unwrap()
	}

	fn connected(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
		let mut connected = Vec::with_capacity(4);

		if pos.0 > 0 && self[pos].up {
			let pos = (pos.0 - 1, pos.1);
			if self[pos].down {
				connected.push(pos);
			}
		}
		if pos.1 < self.0[0].len() && self[pos].right {
			let pos = (pos.0, pos.1 + 1);
			if self[pos].left {
				connected.push(pos);
			}
		}
		if pos.0 < self.0.len() && self[pos].down {
			let pos = (pos.0 + 1, pos.1);
			if self[pos].up {
				connected.push(pos);
			}
		}
		if pos.1 > 0 && self[pos].left {
			let pos = (pos.0, pos.1 - 1);
			if self[pos].right {
				connected.push(pos);
			}
		}

		connected
	}

	#[cfg(not(feature = "p2"))]
	fn distances(&self) -> Vec<Vec<Option<usize>>> {
		fn traverse(this: &Map, distances: &mut Vec<Vec<Option<usize>>>, pos: (usize, usize)) {
			this.connected(pos).into_iter().for_each(|adj| {
				let distance = distances[pos.0][pos.1].unwrap() + 1;
				if distances[adj.0][adj.1]
					.map(|d| d > distance)
					.unwrap_or(true)
				{
					distances[adj.0][adj.1] = Some(distance);
					traverse(this, distances, adj);
				}
			});
		}

		let start = self.start();
		let mut distances = vec![vec![None; self.0[0].len()]; self.0.len()];
		distances[start.0][start.1] = Some(0);
		traverse(self, &mut distances, start);
		distances
	}

	#[cfg(feature = "p2")]
	fn inside_loop(&self) -> impl '_ + Iterator<Item = (usize, usize)> {
		fn traverse(this: &Map, main_loop: &mut Vec<Vec<bool>>, pos: (usize, usize)) {
			this.connected(pos).into_iter().for_each(|adj| {
				if !main_loop[adj.0][adj.1] {
					main_loop[adj.0][adj.1] = true;
					traverse(this, main_loop, adj);
				}
			});
		}

		let start = self.start();
		let mut main_loop = vec![vec![false; self.0[0].len()]; self.0.len()];
		main_loop[start.0][start.1] = true;
		traverse(self, &mut main_loop, start);

		self.0
			.iter()
			.zip(main_loop)
			.enumerate()
			.flat_map(|(l, (line, main_loop))| {
				let mut inside = false;
				let mut last_wall: Option<Pipe> = None;
				line.iter()
					.zip(main_loop)
					.enumerate()
					.filter(|&(c, (&pipe, is_loop))| {
						let mut pipe = pipe;
						if pipe == Pipe::START {
							pipe = Pipe::NONE;
							if l > 0 && self[(l, c)].up && self[(l - 1, c)].down {
								pipe |= Pipe::UP;
							}
							if c < self.0[0].len() && self[(l, c)].right && self[(l, c + 1)].left {
								pipe |= Pipe::RIGHT;
							}
							if l < self.0.len() && self[(l, c)].down && self[(l + 1, c)].up {
								pipe |= Pipe::DOWN;
							}
							if c > 0 && self[(l, c)].left && self[(l, c - 1)].right {
								pipe |= Pipe::LEFT;
							}
						}
						if !is_loop {
							inside
						} else if pipe == Pipe::LEFT | Pipe::RIGHT {
							false
						} else if pipe == Pipe::UP | Pipe::DOWN {
							debug_assert!(last_wall.is_none());
							inside = !inside;
							false
						} else if let Some(wall) = last_wall {
							if (pipe.up && wall.down) || (pipe.down && wall.up) {
								inside = !inside;
							}
							last_wall = None;
							false
						} else {
							last_wall = Some(pipe);
							false
						}
					})
					.map(move |(c, _pipe)| (l, c))
					.collect::<Vec<_>>()
			})
	}
}

fn main() {
	let mut input = String::new();
	std::io::stdin().read_to_string(&mut input).unwrap();

	#[cfg(not(feature = "p2"))]
	let res: usize = Map::from_str(&input)
		.unwrap()
		.distances()
		.iter()
		.flatten()
		.copied()
		.max()
		.unwrap()
		.unwrap();
	#[cfg(feature = "p2")]
	let res: usize = Map::from_str(&input).unwrap().inside_loop().count();

	println!("{res}");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn map_from_str() {
		assert_eq!(
			Map::from_str(include_str!("test.txt")),
			Ok(Map(vec![
				vec![
					Pipe::NONE,
					Pipe::NONE,
					Pipe::RIGHT | Pipe::DOWN,
					Pipe::LEFT | Pipe::DOWN,
					Pipe::NONE,
				],
				vec![
					Pipe::NONE,
					Pipe::RIGHT | Pipe::DOWN,
					Pipe::UP | Pipe::LEFT,
					Pipe::UP | Pipe::DOWN,
					Pipe::NONE,
				],
				vec![
					Pipe::START,
					Pipe::UP | Pipe::LEFT,
					Pipe::NONE,
					Pipe::UP | Pipe::RIGHT,
					Pipe::LEFT | Pipe::DOWN,
				],
				vec![
					Pipe::UP | Pipe::DOWN,
					Pipe::RIGHT | Pipe::DOWN,
					Pipe::LEFT | Pipe::RIGHT,
					Pipe::LEFT | Pipe::RIGHT,
					Pipe::UP | Pipe::LEFT,
				],
				vec![
					Pipe::UP | Pipe::RIGHT,
					Pipe::UP | Pipe::LEFT,
					Pipe::NONE,
					Pipe::NONE,
					Pipe::NONE,
				],
			]))
		);
	}

	#[test]
	fn map_connected() {
		let map = Map::from_str(include_str!("test.txt")).unwrap();

		assert_eq!(map.connected((2, 0)), vec![(2, 1), (3, 0)]);
		assert_eq!(map.connected((1, 2)), vec![(0, 2), (1, 1)]);
		assert_eq!(map.connected((3, 2)), vec![(3, 3), (3, 1)]);
		assert_eq!(map.connected((1, 3)), vec![(0, 3), (2, 3)]);
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
