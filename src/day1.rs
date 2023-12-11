use std::collections::HashMap;

#[allow(dead_code)]
pub fn solve()
{
	let data = std::fs::read_to_string("data/day1.txt").unwrap();
	let text_dig = HashMap::from([
		// ("zero", 0),
		("one", 1),
		("two", 2),
		("three", 3),
		("four", 4),
		("five", 5),
		("six", 6),
		("seven", 7),
		("eight", 8),
		("nine", 9),
	]);

	// let mut totals = Vec::new();
	let mut tot = 0;
	for mut s in data.split("\r\n") {
		let mut first = None;
		let mut last = None;
		let mut found_num = |v| {
			if first.is_none() {
				first = Some(v);
			}
			last = Some(v);
		};
		println!("{}", s);
		while !s.is_empty() {
			let (c, next_s) = s.split_at(1);
			let c = c.chars().next().unwrap();
			if c.is_ascii_digit() {
				found_num(c as u32 - '0' as u32);
			}
			else {
				for (k, d) in text_dig.iter() {
					if s.starts_with(k) {
						found_num(*d);
						break;
					}
				}
			}
			s = next_s;
		}
		if let Some(f) = first {
			let res = f * 10 + last.unwrap();
			tot += res;
			println!("{:?}", res);
		}
	}
	println!("{:?}", tot);
}