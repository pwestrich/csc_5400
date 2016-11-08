
use std::str;
use std::fs::File;
use std::io::{Read, Write, BufWriter};

use nom::digit;
use nom::IResult::*;
use nom::Err::*;
use num_complex::Complex64;
use rand::distributions::IndependentSample;
use rand::distributions::range::Range;
use rand::thread_rng;

///All a polynomial is is an array of its complex coefficients/
///poly[0] is the x^0 term, and so on
pub type Polynomial = Vec<Complex64>;

//<black_magic>
//Create a function named "fractional" that is looking for
//a period followed by a series of digits, returns the digits.
named!(fractional, complete!(chain!(
    tag!(".") ~
    val: digit ,
    || val
)));

//Create a function named "fp" that is looking for a floating point number,
//defined as an optional negative sign, followed by some digits, followed by
//an optional "fractional" part, which is the function defined above
//it creates a new string consisting of each of the pieces, which is not the most
//efficient way to solve this problem, but I haven't figured out how to do it in the
//zero-copy fashion. Once it has the string, it converts it to an f64 and returns it.
named!(fp<f64>, chain!(
    neg: opt!(tag!("-")) ~
    digits: digit ~
    fract: opt!(fractional) ,
    || {
        let mut numstr = String::new();
        if let Some(_) = neg { numstr.push_str("-") }
        numstr += str::from_utf8(digits).unwrap();
        if let Some(fract) = fract { numstr.push_str("."); numstr.push_str(str::from_utf8(fract).unwrap()); }
        let num: f64 = numstr.parse().unwrap();
        return num;
    }
));

//Create a function named "polyfile" that returns a tuple of an i64 and a Polynomial.
//It searches for a floating point number followed by at least one newline,
//then it looks for zero or more complex numbers. Each complex number is defined as
//zero or more newlines, followed by a floating point number, followed by a comma,
//followed by zero or more spaces or tabs, followed by another floating point number.
named!(polyfile <(i64, Polynomial)>,
    chain!(
        deg: fp ~
        many1!(tag!("\n")) ~
        cmplx: many0!(complete!(chain!(
            many0!(tag!("\n")) ~
            re: fp ~
            tag!(",") ~
            many0!(alt!(tag!(" ") | tag!("\t"))) ~
            im: fp ,
            || Complex64::new(re, im)
        ))) ,
        || {
            return (deg as i64, cmplx)
        })
);//</black_magic>

fn get_min_err(pos: &[u8]) -> &str {
    let len = if pos.len() > 7 { 7 } else { pos.len() };
    let errstr = str::from_utf8(&pos[0..len]).unwrap();
    let errstr_lines: Vec<&str> = errstr.split("\n").collect();
    let errstr_1line = errstr_lines[0];
    let errstr_final;
    if  errstr_1line.len() < 3 {
        errstr_final = errstr;
    } else {
        errstr_final = errstr_1line;
    }
    return errstr_final;
}

///This trait defines what we can do with a polynomial
pub trait Poly {

	///Reads the polynomial form a file.
	///Returns the read polynomial on success, or an error message on failure.
	fn readFromFile(filename: &String) -> Result<Polynomial, String>;

	///Generates a random polynomial with the given degree withing the given bounds
	fn random(degree: i32, bounds: f64) -> Result<Polynomial, String>;

	///Generates a pretty looking string to print to the consome.
	fn print(&self) -> String;

	///Writes the polynomial to a file.
	///Returns Ok(()) on success, or a nerror message on failure.
	fn writeToFile(&self, filename: &String) -> Result<(), String>;

}

impl Poly for Polynomial {

	fn readFromFile(filename: &String) -> Result<Polynomial, String> {

		let file = File::open(filename.trim());
	    if file.is_err() { return Err("Failed to open file.".to_string()); }

	    let mut all_text = String::new();
	    let _ = file.unwrap().read_to_string(&mut all_text);

	    let polyresults = polyfile(all_text.as_bytes());
	    let degrees: i64;
	    let poly: Polynomial;
	    match polyresults {
	        Done(extra, (deg, pol)) => {
	            //convert from raw bytes to a utf8 string slice
	            let extra_str = str::from_utf8(extra).unwrap();
	            //there should be nothing left in the file
	            if extra_str.trim().len() > 0 {
	                return Err(format!("Parsing failed. Invalid syntax '{}' in file.", get_min_err(extra)));
	            }
	            degrees = deg;
	            poly = pol;
	        },
	        Error(errco) => {
	            match errco {
	                Position(_, pos) => return Err(format!("Parsing failed. Invalid syntax '{}' in file.", get_min_err(pos))),
	                _ => return Err("Parsing failed, invalid format.".to_string())
	            }
	        }
	        Incomplete(_) => return Err("Parsing failed, invalid format.".to_string()),
	    }

	    if degrees < 0 {
	        return Err("Polynomial degree cannot be negative.".to_string());
	    } else if degrees != poly.len() as i64 {
	        println!("{:?} != {:?}\n{:?}", degrees, poly.len(), poly);
	        return Err(format!("Incorrect number of coefficients, {} instead of {}.", poly.len(), degrees));
	    }

	    return Ok(poly);

	}

	fn random(degree: i32, bounds: f64) -> Result<Polynomial, String> {

		if degree < 0 {

			return Err("You can't have a polynomial of degree less than zero.".to_string());

		} else if bounds <= 0.0 {

			return Err("Range is negative or zero.".to_string());

		}

		let mut poly = Polynomial::with_capacity(degree as usize);

		let range = Range::new(-1.0 * bounds, bounds);
		let mut rng = thread_rng();

		for _ in 0..degree {

			let real: f64 = range.ind_sample(&mut rng);
			let imag: f64 = range.ind_sample(&mut rng);

			poly.push(Complex64::new(real, imag));

		}

		return Ok(poly);

	}

	fn print(&self) -> String {

		let mut string = String::new();

		for term in 0..self.len() {

			let real = self[term].re;
			let imag = self[term].im;
			let sign;

			if imag < 0.0 {

				sign = "-";

			} else {

				sign = "+";

			}

			string.push_str(&format!("({:.5} {} {:.5}i)(x^{})", real, sign, imag.abs(), term));

			if term < self.len() - 1 {

				string.push_str(" + ");

			}

		}

		return string;

	}

	fn writeToFile(&self, filename: &String) -> Result<(), String> {

		//attempt to open the file
		let file = File::create(filename.trim());
		if file.is_err() { return Err("Failed to create file.".to_string()); }

		let mut outFile = BufWriter::new(file.unwrap());

		//first, we write the degree and a newline
		let result = outFile.write(format!("{}\n", self.len()).as_bytes());

		if let Err(_) = result {

			return Err("Could not write to file.".to_string());

		}

		//then, for each element, write "real,imag\n"
		for item in self {

			let result = outFile.write(format!("{},{}\n", item.re, item.im).as_bytes());

			if let Err(_) = result {

				return Err("Could not write to file.".to_string());

			}

		}

		return Ok(());

	}

}

#[cfg(test)]
mod tests {

	use super::*;
	use num_complex::Complex64;

	#[test]
	fn test_read_file(){

		let result = Polynomial::readFromFile(&"./data/file_valid.txt".to_string()).unwrap();
		assert_eq!(result, vec![Complex64::new(1.0,0.0), Complex64::new(2.0,3.25), Complex64::new(3.25,1.0)]);

	}

	#[test]
	fn test_read_file_nonexistant(){

		let result = Polynomial::readFromFile(&"./data/file_nonexistant.txt".to_string());
		assert_eq!(result, Err("Failed to open file.".to_string()));

	}

	#[test]
	fn test_read_file_zero_degree(){

		let result = Polynomial::readFromFile(&"./data/file_valid_zero_degree.txt".to_string()).unwrap();
		assert_eq!(result, Vec::new());

	}

	#[test]
	fn test_read_file_invalid_has_junk_degree(){

		let result = Polynomial::readFromFile(&"./data/file_invalid_has_junk_degree.txt".to_string());
		assert_eq!(result, Err("Parsing failed. Invalid syntax \'junk\' in file.".to_string()));

	}

	#[test]
	fn test_read_file_invalid_has_junk_coeffs(){

		let result = Polynomial::readFromFile(&"./data/file_invalid_has_junk.txt".to_string());
		assert_eq!(result, Err("Parsing failed. Invalid syntax \'junk\' in file.".to_string()));

	}

	#[test]
	fn test_read_file_invalid_has_more_coeffs(){

		let result = Polynomial::readFromFile(&"./data/file_invalid_more_coeffs.txt".to_string());
		assert_eq!(result, Err("Incorrect number of coefficients, 4 instead of 3.".to_string()));

	}

	#[test]
	fn test_random_poly(){

		let result = Polynomial::random(3, 5.0).unwrap();
		assert_eq!(result.len(), 3);

	}

	#[test]
	fn test_random_poly_zero_degree(){

		let result = Polynomial::random(-1, 5.0);
		assert_eq!(result, Err("You can't have a polynomial of degree less than zero.".to_string()));

	}

	#[test]
	fn test_random_poly_zero_range(){

		let result = Polynomial::random(3, -5.0);
		assert_eq!(result, Err("Range is negative or zero.".to_string()));

	}

}