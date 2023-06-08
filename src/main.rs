use std::io::Write;
use std::fs::read_dir;
use std::env;

fn main() {
	let args: Vec<String> = std::env::args().collect();
	println!("{}", args[0]);
	if args[1] == "-q".to_string() {
		if let Err(_) = read_from_file(&args[2]) {
			println!("err");
		}
	}
	else if args[1] == "-m".to_string() {
		// TODO
	}
}

fn read_from_file(path: &str) -> std::io::Result<()>{
	let stop_pos = | path: &str | {
		for idx in (0..path.len()).rev() {
			if let Some(ch) = path.get(idx..idx + 1) {
				if ch == "/" {
					return idx;
				}
			}
		}
		path.len()
	};
	let pre_stop_pos = | path: &str | {
		for idx in (0..path.len()).rev() {
			if let Some(ch) = path.get(idx - 14..idx + 1) {
				if ch == "/src/main/java/" {
					return idx + 1;
				}
			}
			else {
				break;
			}
		}
		0
	};
	let pos = stop_pos(path);
	let pre_pos = pre_stop_pos(path);
	let project_dir_path = &path[..pre_pos];
	// TODO dg
	println!("{}", project_dir_path);
	let all_path = loop_find(project_dir_path, project_dir_path)?;
	for i in all_path {
		println!("{}${},", i.first, i.second);
	}
	Ok(())
}

fn loop_find(project_dir_path: &str, path: &str) -> std::io::Result<Vec<StringPair>>{
	let mut rt_vec = Vec::<StringPair>::new();
	let in_dir = read_dir(path)?;
	for thing in in_dir {
		match thing {
			Ok(thing) => {
				let path = thing.path();
				if path.is_dir() {
					let new_path = path.to_str();
					if let Some(new_dir_path) = new_path {
						let mut new_vec = loop_find(project_dir_path, new_dir_path)?;
						rt_vec.append(&mut new_vec);
					}
				}
				else {
					let new_path = path.to_str();
					if let Some(new_file_path) = new_path {
						let file_path = new_file_path.to_string();
						let class_name = file_path.replace(project_dir_path, "").replace("/", ".").replace(".java", "");
						rt_vec.push(StringPair::new(class_name, file_path));
					}
				}
			}
			Err(err) => {
				return Err(err);
			}
		}
	}
	Ok(rt_vec)
}

struct StringPair {
	first: String,
	second: String,
}

impl StringPair {
	fn new(first: String, second: String) -> Self {
		Self {
			first,
			second,
		}
	}
}
