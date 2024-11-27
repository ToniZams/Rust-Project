//command line (cargo run -- view {image name.type})
//view -h or --help for help

//Image crate defining our RGB values 
//Allows us to get the width and height of an image with GenericImageView
use image::{GenericImageView, Rgba};

//Minifb crate defining our window size controls
use minifb::{Window, WindowOptions, Key};

use std::path::Path;

pub fn viewing(file_name: &str) {
    
	//Find the whole image path
    let image_path = Path::new("images").join(file_name);
    
    //Debug line to see if image is looking for correct location
    //println!("DEBUG: Image Path: {}", image_path.display());

    //Load the image
    let img = match image::open(&image_path) {
        Ok(img) => img,
        Err(e) => {
            println!("Error loading image: {}", e);
            return;
        }
    };

    //Find the dimensions of the image
    let (width, height) = img.dimensions();

    //Create the window, but use the image's width and height as its window size(default)
    let mut window = match Window::new(file_name, width as usize, height as usize, WindowOptions::default()) {
        Ok(window) => window,
        Err(e) => {
            println!("Error creating window: {}", e);
            return;
        }
    };

    //Convert the image to pixels
	//This section was found in the github link for minifb
	//specifically in the examples on the image.rs script
	//I had partially done this using just the github Q/A stuff so its not exact 1:1
	//however it still kind of works? My head hurt going over this :D
	
	//I also didn't realize this could all be done in one crate which is why the image crate is also linked in this script
	//So I simply took the image crate and merged them together and also try to explain how each line works
	
	//convert the image we want into a rgb format
    let img = img.to_rgba8();
	
	//Using the buffer, take the rgb values and put them into a single value(u32)
    let buffer: Vec<u32> = img
	
		//Calls the pixels method that would normally take the rgb values from earlier and puts them into categories(r ,g, b, a)
        .pixels()
		
		//data is the rgb values together.
		//separating them into a map lets us change each value individually
        .map(|p| {
            let Rgba(data) = p; 
			
			//assign the rbga values to a data segment
            let (r, g, b, a) = (data[0], data[1], data[2], data[3]);
			
			//apply the rgb values 
            (u32::from(a) << 24) | (u32::from(r) << 16) | (u32::from(g) << 8) | u32::from(b)
        })
        .collect();

    // Display the image with the window
    while window.is_open() {
        window.update_with_buffer(&buffer, width as usize, height as usize).unwrap();
    }
}
