use std::{
	collections::{HashMap, HashSet, VecDeque},
	io::Read,
	ops::Not,
	str::FromStr,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Pulse {
	#[default]
	Low,
	High,
}
impl Not for Pulse {
	type Output = Self;

	fn not(self) -> Self::Output {
		match self {
			Self::Low => Self::High,
			Self::High => Self::Low,
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Module {
	FlipFlop {
		id: String,
		destinations: Vec<String>,

		on: bool,
	},
	Conjunction {
		id: String,
		destinations: Vec<String>,

		prev_pulses: HashMap<String, Pulse>,
	},
	Broadcast {
		destinations: Vec<String>,
	},
	Output(String),
}
impl FromStr for Module {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut data = s.split("->");
		let id = data.next().ok_or("missing module ID")?.trim();
		let destinations = data
			.next()
			.ok_or("missing module destinations")?
			.split(',')
			.map(|s| s.trim().to_owned())
			.collect();
		match id.as_bytes()[0] {
			b'%' => Ok(Self::FlipFlop {
				id: id[1..].to_owned(),
				destinations,
				on: false,
			}),
			b'&' => Ok(Self::Conjunction {
				id: id[1..].to_owned(),
				prev_pulses: HashMap::new(),
				destinations,
			}),
			b'b' if id == "broadcaster" => Ok(Self::Broadcast { destinations }),
			_ => Err("invalid module"),
		}
	}
}
impl Module {
	fn id(&self) -> &str {
		match self {
			Self::FlipFlop { id, .. } => id,
			Self::Conjunction { id, .. } => id,
			Self::Broadcast { .. } => "broadcaster",
			Self::Output(id) => id,
		}
	}

	fn destinations(&self) -> &[String] {
		match self {
			Self::FlipFlop { destinations, .. } => destinations,
			Self::Conjunction { destinations, .. } => destinations,
			Self::Broadcast { destinations, .. } => destinations,
			Self::Output(_) => unreachable!(),
		}
	}

	fn propagate(&mut self, sender_id: &str, pulse: Pulse) -> Option<Pulse> {
		match self {
			Self::FlipFlop { on, .. } => {
				if pulse == Pulse::Low {
					*on = !*on;
					return Some(if *on { Pulse::High } else { Pulse::Low });
				}
			}
			Self::Conjunction { prev_pulses, .. } => {
				*prev_pulses
					.entry(sender_id.to_owned())
					.or_insert(Pulse::Low) = pulse;
				return Some(if prev_pulses.values().all(|pulse| *pulse == Pulse::High) {
					Pulse::Low
				} else {
					Pulse::High
				});
			}
			Self::Broadcast { .. } => {
				return Some(pulse);
			}
			Self::Output(_) => {}
		}
		None
	}
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Modules(HashMap<String, Module>);
impl FromStr for Modules {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut map: HashMap<String, Module> = s
			.lines()
			.map(|s| Module::from_str(s).map(|module| (module.id().to_owned(), module)))
			.collect::<Result<_, _>>()?;

		let conjunction_inputs: HashMap<String, HashSet<String>> = map
			.values()
			.filter_map(|module| match module {
				Module::Conjunction { .. } => Some(module),
				_ => None,
			})
			.map(|conjunction| {
				(
					conjunction.id().to_owned(),
					map.values()
						.filter_map(|module| {
							module
								.destinations()
								.contains(&conjunction.id().to_owned())
								.then(|| module.id().to_owned())
						})
						.collect(),
				)
			})
			.collect();
		conjunction_inputs
			.into_iter()
			.for_each(|(conjunction, inputs)| {
				let Module::Conjunction { prev_pulses, .. } = map.get_mut(&conjunction).unwrap()
				else {
					unreachable!()
				};
				prev_pulses.extend(
					inputs
						.into_iter()
						.map(|input| (input.to_owned(), Pulse::Low)),
				);
			});

		Ok(Self(map))
	}
}
impl Modules {
	#[cfg(not(feature = "p2"))]
	fn send(&mut self, pulse: Pulse) -> [usize; 2] {
		let mut ret = [0; 2];

		let mut queue = VecDeque::from([("broadcaster".to_owned(), "button".to_owned(), pulse)]);
		while let Some((id, sender_id, pulse)) = queue.pop_front() {
			ret[pulse as usize] += 1;
			let module = self
				.0
				.entry(id.clone())
				.or_insert(Module::Output(id.clone()));
			if let Some(pulse) = module.propagate(&sender_id, pulse) {
				queue.extend(
					module
						.destinations()
						.iter()
						.map(|dest| (dest.to_owned(), id.clone(), pulse)),
				);
			}
		}

		ret
	}

	#[cfg(feature = "p2")]
	fn send(&mut self, pulse: Pulse) -> HashMap<String, bool> {
		let Some(Module::Conjunction {
			prev_pulses: rx_inputs,
			..
		}) = self.0.values().find(|module| match module {
			Module::Conjunction { destinations, .. } => destinations == &["rx"],
			_ => false,
		})
		else {
			unreachable!()
		};
		let mut rx_inputs = rx_inputs
			.keys()
			.map(|id| (id.to_owned(), false))
			.collect::<HashMap<_, _>>();

		let mut queue = VecDeque::from([("broadcaster".to_owned(), "button".to_owned(), pulse)]);
		while let Some((id, sender_id, pulse)) = queue.pop_front() {
			let module = self
				.0
				.entry(id.clone())
				.or_insert(Module::Output(id.clone()));
			if let Some(pulse) = module.propagate(&sender_id, pulse) {
				if let Some(on) = rx_inputs.get_mut(&id) {
					*on |= pulse == Pulse::High;
				}
				queue.extend(
					module
						.destinations()
						.iter()
						.map(|dest| (dest.to_owned(), id.clone(), pulse)),
				);
			}
		}

		rx_inputs
	}
}

fn main() {
	#[cfg(not(feature = "p2"))]
	fn map_modules(mut modules: Modules) -> usize {
		let sums = (0..1000).fold([0_usize; 2], |acc, _i| {
			let results = modules.send(Pulse::Low);
			[acc[0] + results[0], acc[1] + results[1]]
		});
		sums[0] * sums[1]
	}

	#[cfg(feature = "p2")]
	fn map_modules(mut modules: Modules) -> usize {
		let mut rx_inputs = HashMap::new();

		let mut count = 0;
		while rx_inputs.is_empty() || rx_inputs.values().any(|count| *count == 0) {
			count += 1;
			modules.send(Pulse::Low).into_iter().for_each(|(id, on)| {
				let input_count = rx_inputs.entry(id).or_insert(0);
				if on && *input_count == 0 {
					*input_count = count;
				}
			});
		}

		rx_inputs.into_values().reduce(num::integer::lcm).unwrap()
	}

	let mut input = String::new();
	std::io::stdin().read_to_string(&mut input).unwrap();

	let modules = Modules::from_str(&input).unwrap();
	let ret = map_modules(modules);
	println!("{ret}");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn module_from_str() {
		assert_eq!(
			Module::from_str("broadcaster -> a, b, c"),
			Ok(Module::Broadcast {
				destinations: vec!["a".to_owned(), "b".to_owned(), "c".to_owned()]
			})
		);
		assert_eq!(
			Module::from_str("%a -> b"),
			Ok(Module::FlipFlop {
				id: "a".to_owned(),
				destinations: vec!["b".to_owned()],
				on: false,
			})
		);
		assert_eq!(
			Module::from_str("%b -> c"),
			Ok(Module::FlipFlop {
				id: "b".to_owned(),
				destinations: vec!["c".to_owned()],
				on: false,
			})
		);
		assert_eq!(
			Module::from_str("%c -> inv"),
			Ok(Module::FlipFlop {
				id: "c".to_owned(),
				destinations: vec!["inv".to_owned()],
				on: false,
			})
		);
		assert_eq!(
			Module::from_str("&inv -> a"),
			Ok(Module::Conjunction {
				id: "inv".to_owned(),
				destinations: vec!["a".to_owned()],
				prev_pulses: HashMap::new(),
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
