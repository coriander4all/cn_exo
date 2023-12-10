use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use rand::Rng;
use std::process;


fn examine(line: &String, version: usize)->(){
	let sections: Vec<String> = line.split(";").map(|s| s.to_string()).collect();
	println!("");

	println!("{}", &sections[0]);

	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("Failed to read line");
	input = input.trim().to_string();
	if input == "quit" {
		println!("再见！");
		process::exit(0);
	}

	println!("correct answer: {}", &sections[version]);
	let correct: bool = 
		if version == 2 {
			let meanings: Vec<String> = sections[version].split("/").map(|s| s.to_string()).collect();
			meanings.contains(&input)
		} else if version == 1 {
			input == sections[version]
		} else {
			eprintln!("Exercise type number is incorrect: either 1 or 2");
			process::exit(1);
		};
	if correct{
		print!("\x1b[38;5;46m");
	} else {
		print!("\x1b[38;5;1m");
	}
	for section in &sections {
		print!("{: <20}", section);
	}
	println!("\x1b[38;5;255m");
}

fn main() -> io::Result<()> {
	//env::set_var("RUST_BACKTRACE", "1");	
	
	let args: Vec<String> = env::args().collect();
	let version: usize = if args.len()<2 {1} else {args[1].parse().expect("1 or 2 as cli arguments")};

	let mut rng = rand::thread_rng();	

	//println!("Current directory: {:?}", env::current_dir()); 
	let	file_path = Path::new("./assets/cn.csv");
	let file = File::open(file_path)?;
	
	let lines: Vec<String> = io::BufReader::new(file)
			.lines()
			.filter_map(|line| line.ok())
			.collect();

	loop {
		let random_idx = rng.gen_range(0..lines.len());
		if let Some(random_line) = lines.get(random_idx) {
			examine(random_line, version);
		} else {
			eprintln!("issue getting random line");
		}
	}
}

