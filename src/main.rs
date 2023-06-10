use std::io::Write;
use std::fs::read_dir;
use std::env;

fn main() {
	let args: Vec<String> = std::env::args().collect();
	if args[1] == "-q".to_string() {
		if let Err(_) = read_from_file(&args[2]) {
			print!("err");
		}
	}
	else if args[1] == "-c".to_string() {
		match create_file(&args[2], &args[3]) {
			Ok(path) => {
				print!("{}", path);
			}
			Err(_) => {
				print!("err");
			}
		}
	}
	else if args[1] == "-p".to_string() {
		print!("{}", now_class_name(&args[2]))
	}
}

fn create_file(class_name: &str, path: &str) -> std::io::Result<String>{
	let stop = get_project_root(path);
	let filename = format!("{}{}.java", &path[..stop], class_name.replace(".", "/"));
	let mut file = std::fs::File::create(&filename)?;
	let stop_point = get_last_point(class_name);
	let pre = &class_name[..stop_point];
	let class = &class_name[stop_point + 1 ..];
	file.write_all(format!("package {};\n\npublic class {} {{\n}}", pre, class).as_bytes())?;
	Ok(filename)
}

fn get_project_root(path: &str) -> usize {
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
}

fn get_last_point(class_name: &str) -> usize{
	for idx in (0..class_name.len()).rev() {
		if let Some(ch) = class_name.get(idx..idx + 1) {
			if ch == "." {
				return idx;
			}
		}
		else {
			break;
		}
	}
	0
}

fn now_class_name(path: &str) -> String {
	let stop = get_project_root(path);
	let st = &path[stop..].replace(".java", "").replace("/", ".");
	return st.to_string();
}

fn read_from_file(path: &str) -> std::io::Result<()>{
	let pre_pos = get_project_root(path);
	let project_dir_path = &path[..pre_pos];
	let all_path = loop_find(project_dir_path, project_dir_path)?;
	for i in all_path {
		print!("{}!{},", i.first, i.second);
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
