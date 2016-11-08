#![allow(non_snake_case)]

#[macro_use]
extern crate nom;
extern crate num_complex;
extern crate rand;

mod poly;

use std::io;
use std::io::Write;
use std::time::Instant;

use poly::{Polynomial, Poly, rootsOfUnity};

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
			"4" 	=> evaluateNaive(&polynomial),
			"5" 	=> evaluateHorner(&polynomial),
			"6" 	=> evaluateNaiveImproved(&polynomial),
			"7" 	=> evaluateFFT(&polynomial),
			"8" 	=> benchmarkAlgorithms(&polynomial),
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

fn evaluateNaive(poly: &Polynomial){

	//generate the n roots of unity for the polynomial
	println!("Generating roots of unity...");
	let roots = rootsOfUnity(poly.len() as i32);
	let mut values = Vec::new();

	println!("Evaluating polynomial using the naive method...");

	for root in roots {

		values.push(poly.evaluateAtNaive(root));

	}

	println!("Done. The values are: {}", values.print());

}

fn evaluateHorner(poly: &Polynomial){

	//generate the n roots of unity for the polynomial
	println!("Generating roots of unity...");
	let roots = rootsOfUnity(poly.len() as i32);
	let mut values = Vec::new();

	println!("Evaluating polynomial using Horner's method...");

	for root in roots {

		values.push(poly.evaluateAtHorner(root));

	}

	println!("Done. The values are: {}", values.print());

}

fn evaluateNaiveImproved(poly: &Polynomial){

	//generate the n roots of unity for the polynomial
	println!("Generating roots of unity...");
	let roots = rootsOfUnity(poly.len() as i32);
	let mut values = Vec::new();

	println!("Evaluating polynomial using the improved naive method...");

	for root in roots {

		values.push(poly.evaluateAtNaiveImproved(root));

	}

	println!("Done. The values are: {}", values.print());

}

fn evaluateFFT(poly: &Polynomial){

	println!("Evaluating polynomial using the FFT...");

	let values = poly.evaluateAtFFT();

	println!("Done. The values are: {}", values.print());

}

fn benchmarkAlgorithms(poly: &Polynomial){

	//generate the n roots of unity for the polynomial
	println!("Generating roots of unity...");
	let rootsNaive 			= rootsOfUnity(poly.len() as i32);
	let rootsHorner 		= rootsOfUnity(poly.len() as i32);
	let rootsNaiveImproved 	= rootsOfUnity(poly.len() as i32);

	println!("Benchmarking algorithms...");

	let mut naiveValues 		= Vec::with_capacity(poly.len());
	let mut hornerValues 		= Vec::with_capacity(poly.len());
	let mut naiveImprovedValues = Vec::with_capacity(poly.len());

	let naiveStart = Instant::now();

	for root in rootsNaive {

		naiveValues.push(poly.evaluateAtNaive(root));

	}

	let naiveEnd = Instant::now();

	for root in rootsHorner {

		hornerValues.push(poly.evaluateAtHorner(root));

	}

	let hornerEnd = Instant::now();

	for root in rootsNaiveImproved {

		naiveImprovedValues.push(poly.evaluateAtNaiveImproved(root));

	}

	let naiveImprovedEnd = Instant::now();

	let fftValues = poly.evaluateAtFFT();

	let fftEnd = Instant::now();

	let naiveElapsed 			= naiveEnd.duration_since(naiveStart);
	let hornerElapsed 			= hornerEnd.duration_since(naiveEnd);
	let naiveImprovedElapsed 	= naiveImprovedEnd.duration_since(hornerEnd);
	let fftElapsed 				= fftEnd.duration_since(naiveImprovedEnd);

	println!("Done. Results:");
	println!("Naive:          {}s {}ns", naiveElapsed.as_secs(), naiveElapsed.subsec_nanos());
	println!("Horner's:       {}s {}ns", hornerElapsed.as_secs(), hornerElapsed.subsec_nanos());
	println!("Naive Improved: {}s {}ns", naiveImprovedElapsed.as_secs(), naiveImprovedElapsed.subsec_nanos());
	println!("FFT:            {}s {}ns", fftElapsed.as_secs(), fftElapsed.subsec_nanos());
	println!("Values:");
	println!("Naive:          {}", naiveValues.print());
	println!("Horner's:       {}", hornerValues.print());
	println!("Naive Improved: {}", naiveImprovedValues.print());
	println!("FFT:            {}", fftValues.print());

}
