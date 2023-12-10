use std::{borrow::Cow, collections::HashMap, io::Read, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
	Left,
	Right,
}
impl FromStr for Direction {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"L" => Ok(Self::Left),
			"R" => Ok(Self::Right),
			_ => Err("invalid direction"),
		}
	}
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Graph(HashMap<Cow<'static, str>, (Cow<'static, str>, Cow<'static, str>)>);
impl FromStr for Graph {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		s.lines()
			.map(|s| {
				let mut nodes = s.split('=');
				let node = nodes.next().ok_or("missing node")?.trim();
				let mut adjacents = nodes
					.next()
					.ok_or("missing adjacent nodes")?
					.trim()
					.trim_start_matches('(')
					.trim_end_matches(')')
					.split(',');
				let left = adjacents.next().ok_or("missing left adjacent node")?.trim();
				let right = adjacents
					.next()
					.ok_or("missing right adjacent node")?
					.trim();

				Ok((
					node.to_owned().into(),
					(left.to_owned().into(), right.to_owned().into()),
				))
			})
			.collect::<Result<_, _>>()
			.map(Self)
	}
}
impl Graph {
	fn get(&self, node: &str, direction: Direction) -> Option<&str> {
		let (left, right) = self.0.get(node)?;
		Some(match direction {
			Direction::Left => left,
			Direction::Right => right,
		})
	}
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Parameters {
	path: Vec<Direction>,
	graph: Graph,
}
impl FromStr for Parameters {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut lines = s.lines();
		let path = lines
			.next()
			.ok_or("missing path")?
			.bytes()
			.map(|b| Direction::from_str(&(b as char).to_string()))
			.collect::<Result<_, _>>()?;
		let graph = Graph::from_str(&lines.skip(1).map(|s| format!("{s}\n")).collect::<String>())?;

		Ok(Self { path, graph })
	}
}
impl Parameters {
	#[cfg(not(feature = "p2"))]
	fn traverse(&self) -> usize {
		let mut steps = 0;

		let mut current_node = "AAA";
		for direction in self.path.iter().cycle() {
			if current_node == "ZZZ" {
				break;
			} else {
				current_node = self.graph.get(current_node, *direction).unwrap();
				steps += 1;
			}
		}

		steps
	}

	#[cfg(feature = "p2")]
	fn traverse(&self) -> usize {
		let mut current_nodes = self
			.graph
			.0
			.keys()
			.filter(|node| node.ends_with('A'))
			.map(|node| node.as_ref())
			.collect::<Vec<_>>();
		let mut steps_to_loop = vec![0; current_nodes.len()];
		for direction in self.path.iter().cycle() {
			if current_nodes.iter().all(|node| node.ends_with('Z')) {
				break;
			} else {
				current_nodes
					.iter_mut()
					.zip(steps_to_loop.iter_mut())
					.for_each(|(node, steps)| {
						if !node.ends_with('Z') {
							*node = self.graph.get(node, *direction).unwrap();
							*steps += 1;
						}
					});
			}
		}

		steps_to_loop.into_iter().reduce(num::integer::lcm).unwrap()
	}
}

fn main() {
	let mut input = String::new();
	std::io::stdin().read_to_string(&mut input).unwrap();

	let steps = Parameters::from_str(&input).unwrap().traverse();
	println!("{steps}");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parameters_from_str() {
		assert_eq!(
			Parameters::from_str(include_str!("test.txt")),
			Ok(Parameters {
				path: vec![Direction::Right, Direction::Left],
				graph: Graph(HashMap::from([
					("AAA".into(), ("BBB".into(), "CCC".into())),
					("BBB".into(), ("DDD".into(), "EEE".into())),
					("CCC".into(), ("ZZZ".into(), "GGG".into())),
					("DDD".into(), ("DDD".into(), "DDD".into())),
					("EEE".into(), ("EEE".into(), "EEE".into())),
					("GGG".into(), ("GGG".into(), "GGG".into())),
					("ZZZ".into(), ("ZZZ".into(), "ZZZ".into())),
				]))
			})
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
