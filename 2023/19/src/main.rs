use regex::Regex;
use std::{cmp::Ordering, collections::HashMap, convert::Infallible, str::FromStr, sync::OnceLock};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Rating {
	X,
	M,
	A,
	S,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Ratings {
	x: usize,
	m: usize,
	a: usize,
	s: usize,
}
impl Ratings {
	#[cfg(not(feature = "p2"))]
	fn get_rating(&self, rating: Rating) -> usize {
		match rating {
			Rating::X => self.x,
			Rating::M => self.m,
			Rating::A => self.a,
			Rating::S => self.s,
		}
	}
}
impl FromStr for Ratings {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let map: HashMap<&str, usize> = s
			.strip_prefix('{')
			.ok_or("missing '{'")?
			.strip_suffix('}')
			.ok_or("missing '}'")?
			.split(',')
			.map(|s| {
				let mut pair = s.split('=');
				let key = pair.next().ok_or("missing rating key")?;
				let value = pair
					.next()
					.ok_or("missing rating value")?
					.parse()
					.map_err(|_err| "could not parse the rating value")?;
				Ok((key, value))
			})
			.collect::<Result<_, &'static str>>()?;
		Ok(Self {
			x: *map.get("x").ok_or("missing 'x' rating")?,
			m: *map.get("m").ok_or("missing 'm' rating")?,
			a: *map.get("a").ok_or("missing 'a' rating")?,
			s: *map.get("s").ok_or("missing 's' rating")?,
		})
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Destination {
	Workflow(String),
	Rejected,
	Accepted,
}
impl FromStr for Destination {
	type Err = Infallible;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"A" => Ok(Self::Accepted),
			"R" => Ok(Self::Rejected),
			s => Ok(Self::Workflow(s.to_owned())),
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Rule {
	condition: Option<(Rating, Ordering, usize)>,
	then: Destination,
}
impl FromStr for Rule {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		static RE: OnceLock<Regex> = OnceLock::new();
		let re =
			RE.get_or_init(|| Regex::new(r"^(?<rating>\w+)(?<op><|>)(?<threshold>\d+)$").unwrap());

		let rule = s.split(':').collect::<Vec<_>>();
		let then = rule
			.last()
			.ok_or("missing the rule destination")?
			.parse()
			.unwrap();
		let condition = (rule.len() > 1)
			.then(|| {
				let caps = re.captures(rule[0]).ok_or("invalid rule")?;
				let rating = match caps.name("rating").unwrap().as_str() {
					"x" => Rating::X,
					"m" => Rating::M,
					"a" => Rating::A,
					"s" => Rating::S,
					_ => unreachable!(),
				};
				let op = match caps.name("op").unwrap().as_str() {
					"<" => Ordering::Less,
					">" => Ordering::Greater,
					_ => unreachable!(),
				};
				let threshold: usize = caps.name("threshold").unwrap().as_str().parse().unwrap();
				Result::<_, &'static str>::Ok((rating, op, threshold))
			})
			.transpose()?;
		Ok(Self { condition, then })
	}
}
impl Rule {
	#[cfg(not(feature = "p2"))]
	fn apply(&self, ratings: &Ratings) -> bool {
		self.condition
			.map(|condition| {
				let rating = ratings.get_rating(condition.0);
				match condition.1 {
					Ordering::Less => rating < condition.2,
					Ordering::Greater => rating > condition.2,
					Ordering::Equal => unreachable!(),
				}
			})
			.unwrap_or(true)
	}
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
struct Workflow(Vec<Rule>);
impl FromStr for Workflow {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		s.split(',')
			.map(Rule::from_str)
			.collect::<Result<_, _>>()
			.map(Self)
	}
}
impl Workflow {
	#[cfg(not(feature = "p2"))]
	fn run<'this, 'ratings>(&'this self, ratings: &'ratings Ratings) -> &'this Destination {
		self.0
			.iter()
			.find_map(|rule| rule.apply(ratings).then_some(&rule.then))
			.unwrap()
	}
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Workflows(HashMap<String, Workflow>);
impl FromStr for Workflows {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		static RE: OnceLock<Regex> = OnceLock::new();
		let re = RE.get_or_init(|| Regex::new(r"^(?<key>\w+)\{(?<rules>.*)\}$").unwrap());

		s.lines()
			.map(|s| {
				let caps = re.captures(s).ok_or("invalid workflow")?;
				Ok((
					caps.name("key")
						.ok_or("missing the workflow key")?
						.as_str()
						.to_owned(),
					caps.name("rules")
						.ok_or("missing the workflow rules")?
						.as_str()
						.parse()?,
				))
			})
			.collect::<Result<_, _>>()
			.map(Self)
	}
}
impl Workflows {
	#[cfg(not(feature = "p2"))]
	fn is_accepted(&self, ratings: &Ratings) -> bool {
		let mut destination = &Destination::Workflow("in".to_owned());
		while let Destination::Workflow(workflow) = destination {
			destination = self.0.get(workflow).unwrap().run(ratings);
		}
		destination == &Destination::Accepted
	}

	#[cfg(feature = "p2")]
	#[inline]
	fn accepted_combinations(&self) -> usize {
		use std::ops::Range;

		fn recurse(
			workflows: &Workflows,
			current: String,
			x: Range<usize>,
			m: Range<usize>,
			a: Range<usize>,
			s: Range<usize>,
		) -> usize {
			workflows
				.0
				.get(&current)
				.unwrap()
				.0
				.iter()
				.fold((0_usize, x, m, a, s), |(sum, x, m, a, s), rule| {
					let rec = |x: Range<usize>,
					           m: Range<usize>,
					           a: Range<usize>,
					           s: Range<usize>| match &rule.then {
						Destination::Workflow(next) => {
							recurse(workflows, next.to_owned(), x, m, a, s)
						}
						Destination::Rejected => 0,
						Destination::Accepted => x.len() * m.len() * a.len() * s.len(),
					};
					let split_range = |range: Range<usize>, op: Ordering, threshold: usize| match op
					{
						Ordering::Less => (range.start..threshold, threshold..range.end),
						Ordering::Greater => {
							((threshold + 1)..range.end, range.start..(threshold + 1))
						}
						Ordering::Equal => unreachable!(),
					};
					if let Some(condition) = rule.condition {
						match condition.0 {
							Rating::X => {
								let (x, new_x) = split_range(x, condition.1, condition.2);
								(
									rec(x, m.clone(), a.clone(), s.clone()) + sum,
									new_x,
									m,
									a,
									s,
								)
							}
							Rating::M => {
								let (m, new_m) = split_range(m, condition.1, condition.2);
								(
									rec(x.clone(), m, a.clone(), s.clone()) + sum,
									x,
									new_m,
									a,
									s,
								)
							}
							Rating::A => {
								let (a, new_a) = split_range(a, condition.1, condition.2);
								(
									rec(x.clone(), m.clone(), a, s.clone()) + sum,
									x,
									m,
									new_a,
									s,
								)
							}
							Rating::S => {
								let (s, new_s) = split_range(s, condition.1, condition.2);
								(
									rec(x.clone(), m.clone(), a.clone(), s) + sum,
									x,
									m,
									a,
									new_s,
								)
							}
						}
					} else {
						(rec(x, m, a, s) + sum, 0..0, 0..0, 0..0, 0..0)
					}
				})
				.0
		}
		recurse(self, "in".to_owned(), 1..4001, 1..4001, 1..4001, 1..4001)
	}
}

fn main() {
	#[cfg(not(feature = "p2"))]
	fn map_lines(workflows: Workflows, lines: impl Iterator<Item = Ratings>) -> usize {
		lines
			.filter(|ratings| workflows.is_accepted(ratings))
			.map(|ratings| ratings.x + ratings.m + ratings.a + ratings.s)
			.sum()
	}

	#[cfg(feature = "p2")]
	fn map_lines(workflows: Workflows, _lines: impl Iterator<Item = Ratings>) -> usize {
		workflows.accepted_combinations()
	}

	let mut input = std::io::stdin().lines();
	let workflows = Workflows::from_str(
		&input
			.by_ref()
			.map(|res| res.unwrap())
			.take_while(|s| !s.trim().is_empty())
			.map(|s| format!("{s}\n"))
			.collect::<String>(),
	)
	.unwrap();

	let sum: usize = map_lines(
		workflows,
		input.map(|res| Ratings::from_str(&res.unwrap()).unwrap()),
	);
	println!("{sum}");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn ratings_from_str() {
		assert_eq!(
			Ratings::from_str("{x=787,m=2655,a=1222,s=2876}"),
			Ok(Ratings {
				x: 787,
				m: 2655,
				a: 1222,
				s: 2876,
			})
		);
		assert_eq!(
			Ratings::from_str("{x=1679,m=44,a=2067,s=496}"),
			Ok(Ratings {
				x: 1679,
				m: 44,
				a: 2067,
				s: 496,
			})
		);
		assert_eq!(
			Ratings::from_str("{x=2036,m=264,a=79,s=2244}"),
			Ok(Ratings {
				x: 2036,
				m: 264,
				a: 79,
				s: 2244,
			})
		);
		assert_eq!(
			Ratings::from_str("{x=2461,m=1339,a=466,s=291}"),
			Ok(Ratings {
				x: 2461,
				m: 1339,
				a: 466,
				s: 291,
			})
		);
		assert_eq!(
			Ratings::from_str("{x=2127,m=1623,a=2188,s=1013}"),
			Ok(Ratings {
				x: 2127,
				m: 1623,
				a: 2188,
				s: 1013,
			})
		);
	}

	#[test]
	fn workflow_from_str() {
		assert_eq!(
			Workflow::from_str("x>10:one,m<20:two,a>30:R,A"),
			Ok(Workflow(vec![
				Rule {
					condition: Some((Rating::X, Ordering::Greater, 10)),
					then: Destination::Workflow("one".to_owned()),
				},
				Rule {
					condition: Some((Rating::M, Ordering::Less, 20)),
					then: Destination::Workflow("two".to_owned()),
				},
				Rule {
					condition: Some((Rating::A, Ordering::Greater, 30)),
					then: Destination::Rejected,
				},
				Rule {
					condition: None,
					then: Destination::Accepted,
				},
			]))
		);
	}

	#[cfg(not(feature = "p2"))]
	#[test]
	fn workflow_run() {
		let workflow = Workflow::from_str("x>10:one,m<20:two,a>30:R,A").unwrap();

		assert_eq!(
			workflow.run(&Ratings {
				x: 11,
				m: 0,
				a: 0,
				s: 0,
			}),
			&Destination::Workflow("one".to_string())
		);
		assert_eq!(
			workflow.run(&Ratings {
				x: 10,
				m: 19,
				a: 0,
				s: 0,
			}),
			&Destination::Workflow("two".to_string())
		);
		assert_eq!(
			workflow.run(&Ratings {
				x: 10,
				m: 20,
				a: 31,
				s: 0,
			}),
			&Destination::Rejected
		);
		assert_eq!(
			workflow.run(&Ratings {
				x: 10,
				m: 20,
				a: 30,
				s: 0,
			}),
			&Destination::Accepted
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
