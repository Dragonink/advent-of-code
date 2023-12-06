#[cfg(feature = "p2-brute")]
use rayon::prelude::*;
use std::{
	collections::HashMap,
	fmt::{self, Debug, Formatter, Write},
	io::Read,
	ops::Range,
	str::FromStr,
};

#[cfg(feature = "p2")]
fn inter_diff(a: Range<usize>, b: Range<usize>) -> (Range<usize>, Range<usize>, Range<usize>) {
	let inter = a.start.max(b.start)..a.end.min(b.end);
	let diff_before = a.start..a.end.min(inter.start);
	let diff_after = a.start.max(inter.end)..a.end;

	(diff_before, inter, diff_after)
}

#[derive(Default, Clone, PartialEq, Eq)]
struct Map(HashMap<Range<usize>, Range<usize>>);
impl PartialEq<HashMap<Range<usize>, Range<usize>>> for Map {
	#[inline]
	fn eq(&self, other: &HashMap<Range<usize>, Range<usize>>) -> bool {
		self.0 == *other
	}
}
impl FromStr for Map {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		s.lines()
			.map(|line| {
				let mut params = line.split_ascii_whitespace();
				let dest = params
					.next()
					.ok_or("missing destination range start")?
					.parse()
					.map_err(|_err| "could not parse destination range start")?;
				let src = params
					.next()
					.ok_or("missing source range start")?
					.parse()
					.map_err(|_err| "could not parse source range start")?;
				let len: usize = params
					.next()
					.ok_or("missing range length")?
					.parse()
					.map_err(|_err| "could not parse range length")?;

				Ok((src..(src + len), dest..(dest + len)))
			})
			.collect::<Result<_, _>>()
			.map(Self)
	}
}
impl Map {
	#[cfg(not(feature = "p2"))]
	fn map(&self, value: usize) -> usize {
		self.0
			.iter()
			.find_map(|(src, dest)| {
				#[allow(clippy::unnecessary_lazy_evaluations)]
				src.contains(&value).then(|| value + dest.start - src.start)
			})
			.unwrap_or(value)
	}

	#[cfg(feature = "p2")]
	fn map(&self, range: Range<usize>) -> Vec<Range<usize>> {
		self.0
			.iter()
			.find_map(|(src, dest)| {
				let (diff_before, inter, diff_after) = inter_diff(range.clone(), src.clone());
				(!inter.is_empty()).then(|| {
					let mut mapped = vec![
						(inter.start + dest.start - src.start)
							..(inter.end + dest.start - src.start),
					];
					if !diff_before.is_empty() {
						mapped.extend_from_slice(&self.map(diff_before));
					}
					if !diff_after.is_empty() {
						mapped.extend_from_slice(&self.map(diff_after));
					}
					mapped
				})
			})
			.unwrap_or_else(|| vec![range])
	}
}
impl Debug for Map {
	#[inline]
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		Debug::fmt(&self.0, f)
	}
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Parameters {
	#[cfg(not(any(feature = "p2-brute", feature = "p2")))]
	seeds: Vec<usize>,
	#[cfg(any(feature = "p2-brute", feature = "p2"))]
	seeds: Vec<Range<usize>>,
	seed_soil: Map,
	soil_fertilizer: Map,
	fertilizer_water: Map,
	water_light: Map,
	light_temperature: Map,
	temperature_humidity: Map,
	humidity_location: Map,
}
impl FromStr for Parameters {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut lines = s.lines();
		#[cfg(not(any(feature = "p2-brute", feature = "p2")))]
		let seeds: Vec<usize> = lines
			.next()
			.ok_or("missing seeds")?
			.split_ascii_whitespace()
			.skip(1)
			.map(|s| s.parse().map_err(|_err| "could not parse the seed"))
			.collect::<Result<_, _>>()?;
		#[cfg(any(feature = "p2-brute", feature = "p2"))]
		let seeds = {
			let mut seeds = Vec::new();
			let mut values = lines
				.next()
				.ok_or("missing seeds")?
				.split_ascii_whitespace()
				.skip(1)
				.map(|s| s.parse().map_err(|_err| "could not parse the seed"));
			while let Some(start) = values.next() {
				let start = start?;
				let len = values.next().ok_or("missing seed range length")??;
				seeds.push(start..(start + len));
			}
			seeds
		};
		_ = lines.next();
		let mut take_map = || {
			lines.by_ref().skip(1).take_while(|s| !s.is_empty()).fold(
				String::new(),
				|mut s, line| {
					writeln!(s, "{line}").unwrap();
					s
				},
			)
		};
		let seed_soil = Map::from_str(&take_map())?;
		let soil_fertilizer = Map::from_str(&take_map())?;
		let fertilizer_water = Map::from_str(&take_map())?;
		let water_light = Map::from_str(&take_map())?;
		let light_temperature = Map::from_str(&take_map())?;
		let temperature_humidity = Map::from_str(&take_map())?;
		let humidity_location = Map::from_str(&take_map())?;

		Ok(Parameters {
			seeds,
			seed_soil,
			soil_fertilizer,
			fertilizer_water,
			water_light,
			light_temperature,
			temperature_humidity,
			humidity_location,
		})
	}
}
impl Parameters {
	#[cfg(not(any(feature = "p2-brute", feature = "p2")))]
	fn seed_location(&self) -> impl '_ + Iterator<Item = usize> {
		self.seeds.iter().copied().map(|seed| {
			self.humidity_location.map(
				self.temperature_humidity.map(
					self.light_temperature.map(
						self.water_light.map(
							self.fertilizer_water
								.map(self.soil_fertilizer.map(self.seed_soil.map(seed))),
						),
					),
				),
			)
		})
	}

	#[cfg(all(feature = "p2-brute", not(feature = "p2")))]
	fn seed_location(&self) -> impl '_ + ParallelIterator<Item = usize> {
		self.seeds.par_iter().cloned().flatten().map(|seed| {
			self.humidity_location.map(
				self.temperature_humidity.map(
					self.light_temperature.map(
						self.water_light.map(
							self.fertilizer_water
								.map(self.soil_fertilizer.map(self.seed_soil.map(seed))),
						),
					),
				),
			)
		})
	}

	#[cfg(all(not(feature = "p2-brute"), feature = "p2"))]
	fn seed_location(&self) -> Range<usize> {
		self.seeds
			.iter()
			.cloned()
			.flat_map(|range| self.seed_soil.map(range))
			.flat_map(|range| self.soil_fertilizer.map(range))
			.flat_map(|range| self.fertilizer_water.map(range))
			.flat_map(|range| self.water_light.map(range))
			.flat_map(|range| self.light_temperature.map(range))
			.flat_map(|range| self.temperature_humidity.map(range))
			.flat_map(|range| self.humidity_location.map(range))
			.min_by_key(|range| range.start)
			.unwrap()
	}
}

fn main() {
	let mut input = String::new();
	std::io::stdin().read_to_string(&mut input).unwrap();

	let min = Parameters::from_str(&input)
		.unwrap()
		.seed_location()
		.min()
		.unwrap();
	println!("{min}");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[cfg(feature = "p2")]
	#[test]
	fn inter_diff() {
		const B: Range<usize> = 10..20;

		let disjoint_before = super::inter_diff(0..5, B);
		assert_eq!(disjoint_before.0, 0..5);
		assert!(disjoint_before.1.is_empty());
		assert!(disjoint_before.2.is_empty());
		let disjoint_after = super::inter_diff(25..30, B);
		assert!(disjoint_after.0.is_empty());
		assert!(disjoint_after.1.is_empty());
		assert_eq!(disjoint_after.2, 25..30);
		let before = super::inter_diff(5..15, B);
		assert_eq!(before.0, 5..10);
		assert_eq!(before.1, 10..15);
		assert!(before.2.is_empty());
		let in_b = super::inter_diff(12..18, B);
		assert!(in_b.0.is_empty());
		assert_eq!(in_b.1, 12..18);
		assert!(in_b.2.is_empty());
		let after = super::inter_diff(15..25, B);
		assert!(after.0.is_empty());
		assert_eq!(after.1, 15..20);
		assert_eq!(after.2, 20..25);
		let b_included = super::inter_diff(5..25, B);
		assert_eq!(b_included.0, 5..10);
		assert_eq!(b_included.1, 10..20);
		assert_eq!(b_included.2, 20..25);
	}

	macro_rules! map {
		($( $dest:literal $src:literal $len:literal ),* $(,)?) => {
			Map(HashMap::from([
				$( ($src..($src + $len), $dest..($dest + $len)) ),*
			]))
		};
	}

	#[test]
	fn map_from_str() {
		assert_eq!(
			Map::from_str("50 98 2\n52 50 48"),
			Ok(map![
				50 98 2,
				52 50 48,
			])
		);
		assert_eq!(
			Map::from_str("0 15 37\n37 52 2\n39 0 15"),
			Ok(map![
				0 15 37,
				37 52 2,
				39 0 15,
			])
		);
		assert_eq!(
			Map::from_str("49 53 8\n0 11 42\n42 0 7\n57 7 4"),
			Ok(map![
				49 53 8,
				0 11 42,
				42 0 7,
				57 7 4,
			])
		);
		assert_eq!(
			Map::from_str("88 18 7\n18 25 70"),
			Ok(map![
				88 18 7,
				18 25 70,
			])
		);
		assert_eq!(
			Map::from_str("45 77 23\n81 45 19\n68 64 13"),
			Ok(map![
				45 77 23,
				81 45 19,
				68 64 13,
			])
		);
		assert_eq!(
			Map::from_str("0 69 1\n1 0 69"),
			Ok(map![
				0 69 1,
				1 0 69,
			])
		);
		assert_eq!(
			Map::from_str("60 56 37\n56 93 4"),
			Ok(map![
				60 56 37,
				56 93 4,
			])
		);
	}

	#[cfg(not(feature = "p2"))]
	#[test]
	fn map_map() {
		let map = map![
			50 98 2,
			52 50 48,
		];

		for i in 0..50 {
			assert_eq!(map.map(i), i);
		}
		for j in 0..48 {
			assert_eq!(map.map(50 + j), 52 + j);
		}
		assert_eq!(map.map(98), 50);
		assert_eq!(map.map(99), 51);
		assert_eq!(map.map(100), 100);
	}

	#[test]
	fn parameters_from_str() {
		assert_eq!(
			Parameters::from_str(include_str!("test.txt")),
			Ok(Parameters {
				#[cfg(not(any(feature = "p2-brute", feature = "p2")))]
				seeds: vec![79, 14, 55, 13],
				#[cfg(any(feature = "p2-brute", feature = "p2"))]
				seeds: vec![79..(79 + 14), 55..(55 + 13)],
				seed_soil: map![
					50 98 2,
					52 50 48,
				],
				soil_fertilizer: map![
					0 15 37,
					37 52 2,
					39 0 15,
				],
				fertilizer_water: map![
					49 53 8,
					0 11 42,
					42 0 7,
					57 7 4,
				],
				water_light: map![
					88 18 7,
					18 25 70,
				],
				light_temperature: map![
					45 77 23,
					81 45 19,
					68 64 13,
				],
				temperature_humidity: map![
					0 69 1,
					1 0 69,
				],
				humidity_location: map![
					60 56 37,
					56 93 4,
				],
			})
		);
	}

	#[cfg(not(any(feature = "p2-brute", feature = "p2")))]
	#[test]
	fn parameters_seed_location() {
		assert_eq!(
			Parameters::from_str(include_str!("test.txt"))
				.unwrap()
				.seed_location()
				.collect::<Vec<_>>(),
			vec![82, 43, 86, 35]
		);
	}
}
