use std::fs;

pub fn part1() {
	let calibration_values = 
		fs::read_to_string("../inputs/day1/input.txt")
			.expect("Should have been able to read the file")
			.lines()
			.map(|line| {
				line
					.chars()
					.flat_map(|char| {
						char.to_digit(10)
					})
					.collect::<Vec<u32>>()
			})
			.flat_map(|line_digits| {
				match (line_digits.first(), line_digits.last()) {
					(Some(d1), Some(d2)) => Some(d1 * 10 + d2),
					_ => None
				}
			})
			.collect::<Vec<u32>>();

	let cv_sum: u32 = calibration_values.iter().sum();

	println!("sum of calibration values: {cv_sum}")
}

pub fn part2() {
	let calibration_values = 
		fs::read_to_string("../inputs/day1/input.txt")
			.expect("Should have been able to read the file")
			.lines()
			.map(|line| {
				let slices = (0..line.len()).map(|slice_idx| &line[slice_idx..]);

				slices.flat_map(|slice| {
					match slice {
						txt if txt.starts_with("one") => Some(1),
						txt if txt.starts_with("two") => Some(2),
						txt if txt.starts_with("three") => Some(3),
						txt if txt.starts_with("four") => Some(4),
						txt if txt.starts_with("five") => Some(5),
						txt if txt.starts_with("six") => Some(6),
						txt if txt.starts_with("seven") => Some(7),
						txt if txt.starts_with("eight") => Some(8),
						txt if txt.starts_with("nine") => Some(9),
						maybe_num => maybe_num.chars().next().unwrap().to_digit(10)
					}
				}).collect::<Vec<u32>>()
			})
			.flat_map(|line_digits| {
				match (line_digits.first(), line_digits.last()) {
					(Some(d1), Some(d2)) => Some(d1 * 10 + d2),
					_ => None
				}
			})
			.collect::<Vec<u32>>();

	let cv_sum: u32 = calibration_values.iter().sum();

	println!("sum of calibration values: {cv_sum}")
}
