
//Problem        : Base Arithmetic
//Language       : Rust
//Compiled Using : rustc
//Version        : rustc 1.0.0
//Input for your program will be provided from STDIN
//Print out all output from your program to STDOUT
use std::io;
use std::io::prelude::*;

fn main() {
	let mut buf: String = "".to_string();
	io::stdin().read_line(&mut buf).ok().expect("reading input");
	//println!("{:?}", buf);
	let max = buf.chars().max().expect("fetching max char");
	println!("{:?}", max);
	// +1 for 0-based offset & +1 because base is always 1 higher than max number
	let base = "1234567890ABCDEF".find(lc).expect("expecting character to exist in hex list") + 2;
	println!("{}", base);
}

