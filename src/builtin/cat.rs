impl {
    pub fn cat_command(out: String) -> Result<(), Box<dyn Error>> {
	match out.find(">") {
	    Some(index) => {
		if index == 0 {
		    let file_path = &out[index + 1..].trim_start();
		    match file_path.find("<<") {
			Some(index) => {
			    let end = &file_path[index + 2..].trim_start();
			    let path = &file_path[..index].trim_end();
			    loop {
				print!("> ");
				io::stdout().flush().unwrap();
				let mut input = String::new();
				let _ = io::stdin().read_line(&mut input).unwrap();
				let input = input.trim();
				let mut file = OpenOptions::new()
				    .create(true)
				    .append(true)
				    .open(path)
				    .unwrap();
				if input.contains(end) {
				    break;
				} else {
				    writeln!(file, "{}", input).unwrap();
				}
			    }
			}
			None => {
			    print!("> ");
			    io::stdout().flush().unwrap();
			    let mut input = String::new();
			    let _ = io::stdin().read_line(&mut input).unwrap();
			    let input = input.trim();
			    _ = fs::write(file_path, input);
			}
		    }
		}
	    }
	    None => {
		let message: String = fs::read_to_string(out)?;
		println!("{}", message);
	    }
	}
	return Ok(());
    }

}
