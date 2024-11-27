//command line (cargo run -- calc <num1> <operator> <num2>)
//calc -h or --help for help

use clap::ArgMatches;

pub fn calculate(matches: &ArgMatches) {
    //Find arguments using `get_one`
    let num1: f64 = *matches.get_one::<f64>("num1").unwrap();
    let operator: String = matches.get_one::<String>("operator").unwrap().to_string();
    let num2: f64 = *matches.get_one::<f64>("num2").unwrap();

    //Calculate based on operator
    let result = match operator.as_str() {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" | "x" => num1 * num2,
        "/" => {
            if num2 != 0.0 {
                num1 / num2
            } else {
                println!("Error: Division by zero.");
                return;
            }
        }
        _ => {
            println!("Error: Invalid operator.");
            return;
        }
    };

    //Result time
    println!("{} {} {} = {}", num1, operator, num2, result);
}
