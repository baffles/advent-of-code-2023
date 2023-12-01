#!/usr/bin/env scala-cli

import scala.io.Source

val part1 = {
	Source
		.fromFile("../day1/input.txt")
		.getLines
		.map { line =>
			val digits = line.collect {
				case x if x.isDigit => x.asDigit
			}

			digits.head * 10 + digits.last
		}
		.sum
}

println(s"part 1: sum of calibration values = $part1")


val part2 = {
	Source
		.fromFile("../day1/input.txt")
		.getLines
		.map { line =>
			val digits = {
				line
					.tails
					.filter(_.nonEmpty)
					.collect {
						case dig if dig.head.isDigit => dig.head.asDigit
						case txt if txt.startsWith("one") => 1
						case txt if txt.startsWith("two") => 2
						case txt if txt.startsWith("three") => 3
						case txt if txt.startsWith("four") => 4
						case txt if txt.startsWith("five") => 5
						case txt if txt.startsWith("six") => 6
						case txt if txt.startsWith("seven") => 7
						case txt if txt.startsWith("eight") => 8
						case txt if txt.startsWith("nine") => 9
					}
					.toList
			}

			digits.head * 10 + digits.last
		}
		.sum
}

println(s"part 2: sum of calibration values = $part2")
