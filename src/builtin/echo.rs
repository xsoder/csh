impl {
    let output = input.strip_prefix("echo").unwrap_or("").trim();
    match output.find("-"){
	Some(index){
	    let main_flag = &output[index + 1..].trim_start().chars().next();
	    let extra_flags = &output[index + 1..].trim_start().chars().next();
	    println("{}",main_flag);
	    println("{}",extra_flag);
	    match main_flag {
		Some("") => {todo!();},
		None => {todo!();}
	    }
	    println!("{}", output);
	}
    }
}
