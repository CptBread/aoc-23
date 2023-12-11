use std::collections::HashSet;
use aoc_util::array2d::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Grid {
	Empty,
	Num(u32, u32),
	Sym(char),
}

#[allow(dead_code)]
pub fn solve()
{
	let mut arr = Array2D::load_file("data/day3.txt", |c| {
		match c {
			'.' => Grid::Empty,
			c if c.is_ascii_digit() => Grid::Num(c as u32 - '0' as u32, 0),
			c => Grid::Sym(c),
		}
	});

	let mut sum_part = 0;
	for row in 0..arr.height {
		let mut num_start = None;
		let mut num = 0;
		let mut part = false;
		for idx in 0..arr.width {
			let pos = Pos::new(idx, row);
			if let Grid::Num(v, _) = arr.get(pos).unwrap() {
				num_start.get_or_insert(idx);
				num *= 10u32;
				num += v;

				if !part {
					for neigh in arr.neighbours_diag(pos) {
						if let Some(p) = neigh {
							if let Some(&Grid::Sym(_)) = arr.get(p) {
								part = true;
								break;
							}
						}
					}
				}
			}
			else if let Some(start) = num_start {
				for i in start..=idx {
					let pos = Pos::new(i, row);
					let g = arr.get_mut(pos).unwrap();
					if let Grid::Num(_, t) = g {
						*t = num;
					}
				}
				if part {
					sum_part += num;
				}
				num_start = None;
				num = 0;
				part = false;
			}
		}
		if let Some(start) = num_start {
			for i in start..arr.width {
				let pos = Pos::new(i, row);
				let g = arr.get_mut(pos).unwrap();
				if let Grid::Num(_, t) = g {
					*t = num;
				}
			}
			if part {
				sum_part += num;
			}
		}
	}
	println!("{}", sum_part); // 527446

	let mut gear_sum = 0;
	for y in 0..arr.height {
		for x in 0..arr.width {
			let pos = Pos::new(x, y);
			if Some(Grid::Sym('*')) == arr.get_copy(pos) {
				let mut nums = HashSet::new();
				for n in arr.neighbours_diag(pos) {
					if let Some(p) = n {
						if let Some(Grid::Num(_, num)) = arr.get(p) {
							nums.insert(*num);
						}
					}
				}
				if nums.len() == 2 {
					gear_sum += nums.iter().product::<u32>();
				}
			}
		}
	}
	println!("{}", gear_sum);


	// arr.print(|v| {
	// 	match v {
	// 		Grid::Empty => '.',
	// 		Grid::Num(v, _) => char::from_u32(*v + '0' as u32).unwrap(),
	// 		Grid::Sym(c) => *c,
	// 	}
	// });

	// let mut last_num = None;
	// for g in arr.data.iter() {
	// 	match g {
	// 		Grid::Num(_, t) => {
	// 			if last_num.is_none() || last_num.unwrap() != t {
	// 				println!("{}", t);
	// 				last_num = Some(t);
	// 			}
	// 		}
	// 		_ => last_num = None,
	// 	}
	// }

}