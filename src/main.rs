// Bring into scope I/O from standart library
// And binary library
use std::io;
use::cel_or_farh::*;
fn main() {
    // Loop over all fuction for handling errors
    // without closing programm outright
    'outer: loop {
        println!("Celsius or Fahrenheit? (input C or F).\n");
        // Create new string
        let mut c_or_f = String::new();
        // Read input and bind it to c_or_f
        io::stdin()
            .read_line(&mut c_or_f)
            .expect("Failed to read the line.");
        
        let no_whitespace = c_or_f.trim();
        // Checking if input is either "C" or "F", returning them
        let checked = check_c_or_f(no_whitespace);
        // Else returns "Error" to check for directing back to new input
        if checked == "Error" {
            continue 'outer;
        }
        
        println!("How much?\n");

        let mut number_input= String::new();
  
        io::stdin()
            .read_line(&mut number_input)
            .expect("Failed to read the line.");
        // Parsing input into f64, returning error if it can't be done
        // Would like to make it to direct to second input rather than first
        let number_input: f64 = match number_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\nError, input is not a number.\n");
                continue;
            },
        };
        
        // Calculate Celsius or Farhenheit depending on the first input
        calc(checked, number_input);
        break;
        
    }
}
