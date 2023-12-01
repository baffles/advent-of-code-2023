use std::env;
mod day1;

fn main() {
	let exe = env::args().nth(0).unwrap();
	let day = env::args().nth(1).expect(format!("Usage: {} [day]", exe).as_str());

	match day.as_str() {
		"1" => { crate::day1::part1(); crate::day1::part2(); },
		_ => { println!("Invalid day!"); },
	}
}
