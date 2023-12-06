#!/bin/bash

test_part() {
  year="$1"
  day="$2"
  t="$3"
  shift 3

  RUSTFLAGS='--cfg star' cargo test --package "aoc-$year-$day" "$t" -- "$@" 2>/dev/null
}

readarray -t DAYS < <(seq --equal-width 1 25)
YEARS=()
for year in *; do
  if [[ "$year" =~ ^[[:digit:]]+$ ]]; then
    YEARS+=("$year")
  fi
done

echo '# Advent of Code stars'

header="||"
separator="|:-:|"
for year in "${YEARS[@]}"; do
  header+="$year|"
  separator+=":-:|"
done
echo "$header"
echo "$separator"

for day in "${DAYS[@]}"; do
  printf '|**%d**|' $(( 10#$day ))
  for year in "${YEARS[@]}"; do
    if [ -d "$year/$day" ]; then
      if [ "$(test_part "$year" "$day" '' --list | grep -Po '^tests::p\d(?=:)' | wc --lines)" -ne 2 ]; then
        printf '%s/%s is missing star tests' "$year" "$day" >&2
        exit 1
      fi
      if test_part "$year" "$day" 'tests::p2' --exact >/dev/null; then
        printf '[⭐⭐](https://adventofcode.com/%s/day/%d)|' "$year" $(( 10#$day ))
        continue
      elif test_part "$year" "$day" 'tests::p1' --exact >/dev/null; then
        printf '[⭐](https://adventofcode.com/%s/day/%d)|' "$year" $(( 10#$day ))
        continue
      fi
    fi
    printf '[⬛](https://adventofcode.com/%s/day/%d)|' "$year" $(( 10#$day ))
  done
  echo
done
