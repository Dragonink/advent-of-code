#[cfg(feature = "p2")]
use rayon::prelude::*;
use std::{
	collections::HashMap,
	fmt::{self, Debug, Formatter, Write},
	io::Read,
	ops::Range,
	str::FromStr,
};

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
	fn get(&self, value: usize) -> usize {
		self.0
			.iter()
			.find_map(|(src, dest)| {
				#[allow(clippy::unnecessary_lazy_evaluations)]
				src.contains(&value).then(|| dest.start + value - src.start)
			})
			.unwrap_or(value)
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
	#[cfg(not(feature = "p2"))]
	seeds: Vec<usize>,
	#[cfg(feature = "p2")]
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
		#[cfg(not(feature = "p2"))]
		let seeds: Vec<usize> = lines
			.next()
			.ok_or("missing seeds")?
			.split_ascii_whitespace()
			.skip(1)
			.map(|s| s.parse().map_err(|_err| "could not parse the seed"))
			.collect::<Result<_, _>>()?;
		#[cfg(feature = "p2")]
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
	#[cfg(not(feature = "p2"))]
	fn seed_location(&self) -> impl '_ + Iterator<Item = usize> {
		self.seeds.iter().copied().map(|seed| {
			self.humidity_location.get(
				self.temperature_humidity.get(
					self.light_temperature.get(
						self.water_light.get(
							self.fertilizer_water
								.get(self.soil_fertilizer.get(self.seed_soil.get(seed))),
						),
					),
				),
			)
		})
	}

	#[cfg(feature = "p2")]
	fn seed_location(&self) -> impl '_ + ParallelIterator<Item = usize> {
		self.seeds.par_iter().cloned().flatten().map(|seed| {
			self.humidity_location.get(
				self.temperature_humidity.get(
					self.light_temperature.get(
						self.water_light.get(
							self.fertilizer_water
								.get(self.soil_fertilizer.get(self.seed_soil.get(seed))),
						),
					),
				),
			)
		})
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

	#[test]
	fn map_get() {
		let map = map![
			50 98 2,
			52 50 48,
		];

		for i in 0..50 {
			assert_eq!(map.get(i), i);
		}
		for j in 0..48 {
			assert_eq!(map.get(50 + j), 52 + j);
		}
		assert_eq!(map.get(98), 50);
		assert_eq!(map.get(99), 51);
		assert_eq!(map.get(100), 100);
	}

	#[test]
	fn parameters_from_str() {
		assert_eq!(
			Parameters::from_str(include_str!("test.txt")),
			Ok(Parameters {
				#[cfg(not(feature = "p2"))]
				seeds: vec![79, 14, 55, 13],
				#[cfg(feature = "p2")]
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

	#[cfg(not(feature = "p2"))]
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
