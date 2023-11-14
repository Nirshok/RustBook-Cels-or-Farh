// Fix somehow that output is too big to display correctly
pub fn calc(str: &str, numb: f64) {
    match str {
        "C" => {
            let result = numb / 5.0 * 9.0 + 32.0;
            println!("{numb} Celsius is {result} Fahrenheit");
        },
        "F" => {
            let result = ((numb - 32.0) * 5.0) / 9.0;
            println!("{numb} Fahrenheit is {result} Celsius");
        }
        // Should not return in in normal circumstances
        // Since it's compared prior in check_c_or_f
        _ => println!("Error, string is corrupted."),
    }
    
}

pub fn check_c_or_f(input: &str) -> &str {
    match input {
        "C" => {
            println!("\nCelsius\n");
            input
        },
        "F" => {
            println!("\nFahrenheit\n");
            input
        },
        _ => {
            println!("\nWrong input!\n");
            // Returns "Error" for outside manipulation
            let error = "Error";
            error
        },
            
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn working_calc() {
        let name = "C";
        let number: f64 = 102.0;

        calc(name, number)
    }
    
    #[test]
    fn weird_err_calc() {
        let name = "GG";
        let number: f64 = 55.0;

        calc(name, number)
    }
// Doesn't work as intended
// Somehow fix big number inputs and outputs
  /*   #[test]
    fn failing_calc() {
        let name = "F";
        let number = 123142354353465463423534636345777.0;

        calc(name, number)
    }
    */
    #[test]
    fn checked() {
        let word = "C";

        assert_eq!("C", check_c_or_f(word));
    }

    #[test]
    fn wrong_input() {
        let word = "Mom's spaghetti";
        let numb = "1";

        assert_eq!("Error", check_c_or_f(word));
        assert_eq!("Error", check_c_or_f(numb));
    }
}
