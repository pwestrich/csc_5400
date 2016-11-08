#![allow(non_snake_case)]

#[macro_use]
extern crate nom;
extern crate num_complex;
extern crate rand;

mod poly;

use std::io;
use std::io::Write;

use poly::{Polynomial, Poly};

fn main(){

	let mut polynomial = Polynomial::new();

	loop {

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

			"1" 	=> polynomial = generateRandomPolynomial(),
			"2" 	=> polynomial = readFromFile(),
			"3" 	=> writeToFile(&polynomial),
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

fn generateRandomPolynomial() -> Polynomial {

	let mut input = String::new();

	print!("Enter the degree of polynomial you'd like: ");
	let _ = io::stdout().flush();
	io::stdin().read_line(&mut input).expect("stdin is broken.");

	let degree = input.trim().parse();
	if let Err(e) = degree {

		println!("Please enter an integer above zero: {:?}", e);
		return generateRandomPolynomial();

	}

	let degree: i32 = degree.unwrap();

	input.clear();
	print!("Enter the bounds of the coefficients: ");
	let _ = io::stdout().flush();
	io::stdin().read_line(&mut input).expect("stdin is broken.");

	let bounds = input.trim().parse();
	if let Err(_) = bounds {

		println!("Please enter a decimal number above zero.");
		return generateRandomPolynomial();

	}

	let bounds: f64 = bounds.unwrap();

	let poly = Polynomial::random(degree, bounds);
	if let Err(e) = poly {

		println!("{}", e);
		return generateRandomPolynomial();

	} else {

		return poly.unwrap();

	}

}

fn readFromFile() -> Polynomial {

	let mut filename = String::new();
	print!("Enter a filename to read: ");
	let _ = io::stdout().flush();
	io::stdin().read_line(&mut filename).expect("stdin is broken.");

	let poly = Polynomial::readFromFile(&filename);

	if let Err(e) = poly {

		println!("Error reading file: {}", e);
		return readFromFile();

	} else {

		return poly.unwrap();

	}

}

fn writeToFile(poly: &Polynomial){

	let mut filename = String::new();
	print!("Enter a filename to write to: ");
	let _ = io::stdout().flush();
	io::stdin().read_line(&mut filename).expect("stdin is broken.");

	let result = poly.writeToFile(&filename);

	if let Err(e) = result {

		println!("File not written: {}", e);

	} else {

		println!("File written successfully to \"{}\"", filename);

	}

}
