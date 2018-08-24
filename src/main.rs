use std::env;
use std::time::{Duration, Instant};


fn main() {
	println!("My test timer in Rust");
	
	
	let duration: u32 = 0;
	
	//reading the args
	for (index, argument) in env::args().enumerate() {
	    println!("Index: {}, Arg : {}", index, argument);
		
		if index == 1 {
			//duration read from args
			let duration: u32 = match argument.parse() {
				Ok(time)=>time,
				Err(e)=>panic!("Problem with your input {}", e)
			};
		}	
	}
	
	
	//iterating in seconds
	let time_span = Duration::new(duration as u64, 0);
	let time_origin = Instant::now();
	let mut time_step = time_origin;
	let mut time_spent = Duration::new(0, 0);
	
	while time_spent.as_secs() < time_span.as_secs() {
		if time_origin.duration_since(time_step).as_secs() >= 1 {
			println!("+1");
			time_step = Instant::now();
		}
		time_spent = time_origin.elapsed();
		//println!("Ligne");
	}

	println!("Timer stopped");

}

fn usage(){
	println!("Launch the timer with following arguments");
	println!("");
}


fn read_input(){
	;
}