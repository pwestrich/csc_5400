#![allow(non_snake_case)]

extern crate num_complex;

mod poly;

use std::io;
use std::io::Write;

use poly::{Polynomial, Poly};

fn main(){

	loop {

		let mut polynomial = Polynomial::new();
		let mut choice = String::new();

		println!("Menu");
		println!("---------------------------------------------------------------------");
		println!("1.  Generate a random polynomial with complex coefficients.");
		println!("2.  Read coefficients from a file.");
		println!("3.  Write coefficients to a file.");
		println!("4.  Evaluate the current polynomial using the naive method.");
		println!("5.  Evaluate the current polynomial using Horner's method.");
		println!("6.  Evaluate the current polynomial using the improved naive method");
		println!("7.  Evaluate the current polynomial using the Fast Fourier Transform.");
		println!("8.  Display run times for the above four algorithms.");
		println!("9.  Display the multiplpication count for the above four algorithms.");
		println!("10. Quit the program.\n");

		println!("The current polynomial is: {}", polynomial.print());
		print!("Your choice: ");
		let _ = io::stdout().flush();
		io::stdin().read_line(&mut choice).expect("stdin is broken.");

		match choice.trim() {

			"1" 	=> println!("Not implemented."),
			"2" 	=> println!("Not implemented."),
			"3" 	=> println!("Not implemented."),
			"4" 	=> println!("Not implemented."),
			"5" 	=> println!("Not implemented."),
			"6" 	=> println!("Not implemented."),
			"7" 	=> println!("Not implemented."),
			"8" 	=> println!("Not implemented."),
			"9" 	=> println!("Not implemented."),
			"10"	=> return,
			_   	=> println!("Invalid choice. Please try again.")

		}

	}

}
