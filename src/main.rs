use std::str::FromStr;
use std::io::{stdin,stdout,Write};
use std::fs;
use std::path::Path;

fn main() {
	let config_file = format!("{}/.config/{}", env!("HOME"), "TORustL.json");
	let mut todo: Vec<String> = Vec::new();
	if Path::new(&config_file).exists(){
		let tmp = json::parse(String::from_utf8(fs::read(&config_file).unwrap()).unwrap().as_str()).unwrap();
		for index in 0..tmp.len() {
			todo.push(tmp[index].to_string());
		}
	} else {
		let _=fs::create_dir_all(format!("{}/.config/", env!("HOME")));
	}
	print!(
r#"Welcome to TORustL (TODO List in rust)
Commands: (A)dd/New (adding with name), (R)m/Remove (removing by id or last), (L)s/List/(S)how (showing all or by id), (Q)uit.
"#);
	loop {
		print!("$ ");
		let mut input = String::new();
		let _=stdout().flush();
		let _=stdin().read_line(&mut input);
		if let Some('\n')=input.chars().next_back() {input.pop();}if let Some('\r')=input.chars().next_back() {input.pop();}
		let mut input: Vec<&str> = input.split(' ').collect();
		match input[0].to_lowercase().as_str() {
			"a"|"add"|"new" => {
				if !(input.len() >= 2) {
					println!("Error: you should give name.");
					continue;
				}	input.remove(0);
				let input = input.join(" ");
				todo.push(input.clone());
				println!("\"{}\" added to TODO List.", input);
			} "r"|"rm"|"remove" => {
				if todo.len() != 0 {
					if !(input.len() >= 2) {
						println!("Error: you should give id.");
						continue;
					} if i32::from_str(input[1]).is_err() {
						println!("Error: you should give id as decimal.");
						continue;
					}
					let index = usize::from_str(input[1]).unwrap() -1;
					//println!("l:{:?} | i:{:?} | t:{:?}", todo.len(), index+1, index+1 > todo.len());
					if index > todo.len()-1 {
						println!("Error, this id is bigger than TODO List length. (max: {})", todo.len());
						continue;
					}
					println!("removing \"{}\" from TODO List.", todo[index]);
					todo.remove(index);
				} else {
					println!("Error: TODO List is empty,");
				}
			} "s"|"l"|"ls"|"list"|"show" => {
				if todo.len() == 0 {
					println!("your TODO List is empty.");
					continue;
				}
				println!("{}", todo.join("\n"));
			} "q"|"quit" => {
				println!("saving to \"{}\" . . .", &config_file);
				let _=fs::write(config_file, json::from(todo.clone()).dump());
				println!("Goodbye!");
				break;
			} 
			_ => {}
		}
	}
}
