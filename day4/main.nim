#!/usr/bin/env -S nim r

import std/strutils
import std/strformat

proc part1_condition(first_low: int, first_high: int, second_low: int, second_high: int): bool =
  first_low <= second_low and first_high >= second_high

proc part2_condition(first_low: int, first_high: int, second_low: int, second_high: int): bool =
  for i in first_low .. first_high:
    if i >= second_low and i <= second_high:
      return true

var part1 = 0
var part2 = 0

for line in lines "input.txt":
  let splitted = line.split(',')

  let first = splitted[0].split('-')
  let first_low = parseInt(first[0])
  let first_high = parseInt(first[1])

  let second = splitted[1].split('-')
  let second_low = parseInt(second[0])
  let second_high = parseInt(second[1])
  
  if part1_condition(first_low, first_high, second_low, second_high) or part1_condition(second_low, second_high, first_low, first_high):
    part1 += 1

  if part2_condition(first_low, first_high, second_low, second_high) or part2_condition(second_low, second_high, first_low, first_high):
    part2 += 1

echo &"part1: {part1}"
echo &"part2: {part2}"