mod utils;

use std::io::Write;
use std::fs::read_dir;
use std::collections::HashMap;

fn main() {
	if let Err(err) = read_from_file("/home/paradoxd/workspace/java/tmp-app/src/main/java/org/paradox/app/App.java") {
		println!("{}", err);
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
	let mut class_to_path = HashMap::<String, String>::new();
	loop_find(project_dir_path, &mut class_to_path)?;
	Ok(())
}

fn loop_find(path: &str, map: &mut HashMap<String, String>) -> std::io::Result<()>{
	let in_dir = read_dir(path)?;
	for thing in in_dir {
		match thing {
			Ok(thing) => {
				let path = thing.path();
				if path.is_dir() {
					let new_path = path.to_str();
					if let Some(new_dir_path) = new_path {
					}
				}
			}
			Err(err) => {
				return Err(err);
			}
		}
	}
	Ok(())
}
