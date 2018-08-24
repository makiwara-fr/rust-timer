use std::env;

fn main() {
	println!("My test timer in Rust");
	
	for argument in env::args() {
	    println!("{}", argument);
	}
}

fn usage(){
	println!("Launch the timer with foloowing arguments")
	println!("")
}


fn read_input(){
	;
}