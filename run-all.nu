#!/usr/bin/env nu

def main []: nothing -> table<day: datetime, silver: any, gold: any> {
	^cargo build --workspace --release

	open Cargo.toml
	| get workspace.members
	| each { glob $in }
	| flatten
	| each { path relative-to $env.FILE_PWD }
	| par-each {
		let day = (
			$in
			| parse '{year}/{day}'
			| first
		)
		let input = (
			$in |
			path join 'input.txt' |
			open --raw
		)

		let outputs = (
			['silver', 'gold']
			| par-each --keep-order {|part|
				let output = (
					$input
					| ^cargo run --package $"aoc-($day.year)-($day.day)" --bin $"aoc-($day.year)-($day.day)-($part)" --release
					| complete
				)
				{
					part: $part,
					output: (if $output.exit_code == 0 { $output.stdout | str trim } else { null }),
				}
			}
			| reduce --fold {} {|row, rec| $rec | insert $row.part $row.output}
		)
		{
			day: $"($day.year)/($day.day)",
			silver: $outputs.silver?,
			gold: $outputs.gold?,
		}
	}
	| sort-by day
}
