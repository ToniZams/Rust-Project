//Main script
//Handles all commands
	//Command information and details

mod calculator;		 //Import the calculator module
mod file_reader;	//Import the file reader module
mod image_viewer;  //Import the image viewer module

use clap::{Arg, Command};
use file_reader::reading;
use calculator::calculate;
use image_viewer::viewing; 

fn main() {
	
    // Define the command-line interface using clap
    let matches = Command::new("RustProject")
        .version("1.0")
        .author("Jordan Maier")
        .about("Simple CLI Application")
        .subcommand(
			//Calculator Command
            Command::new("calc")
                .about("Performs a calculation")
                .arg(
                    Arg::new("num1")
                        .help("First value")
                        .required(true)
                        .value_parser(clap::value_parser!(f64)),
                )
                .arg(
                    Arg::new("operator")
						.help("+, -, *, /")
						.required(true)
						.value_parser(["+", "-", "*", "/", "x"]),
                )
                .arg(
                    Arg::new("num2")
                        .help("Second value")
                        .required(true)
                        .value_parser(clap::value_parser!(f64)),
                ),
        )
        .subcommand(
			//File Reader Command
            Command::new("read")
                .about("Reads a text file from the 'files' folder")
                .arg(
                    Arg::new("file")
                        .help("The name of the file to read")
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                ),
        )
        .subcommand(
			//Image Viewer Command
            Command::new("view")
                .about("Displays an image")
                .arg(
                    Arg::new("image")
                        .help("The name of the image file")
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                ),
        )
        .get_matches();


	//I suppose these could be put into a big block
	//But alas I think its more organized like this in my opinion

    //Handling the 'calc' command
    if let Some(cal_matches) = matches.subcommand_matches("calc") {
        calculate(cal_matches);
    }

    //Handling the 'read' command
    if let Some(read_matches) = matches.subcommand_matches("read") {
        if let Some(file_name) = read_matches.get_one::<String>("file") {
            reading(file_name);
        }
    }

    //Handling the 'view' command
    if let Some(view_matches) = matches.subcommand_matches("view") {
        if let Some(image_name) = view_matches.get_one::<String>("image") {
            viewing(image_name);
        }
    }
}
