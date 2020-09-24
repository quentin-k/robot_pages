mod urls;

use std::env;
use std::fs::create_dir_all;
use tokio;
use urls::urls as url_mod;

static OUTPUT_DIR: &str = "output";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create the tokio runtime
    let rt = &mut tokio::runtime::Runtime::new().unwrap();

    // Read the arguments passed to the program
    let args: Vec<String> = env::args().collect();

    // Make sure the user supplied only 1 additional argument
    if args.len() != 2 {
        println!("This program takes 1 argument, which is the path to the list of URLs!");
        return Ok(());
    }

    // Get the URLs and create output directory
    let urls: Vec<String> = url_mod::import(&*args[1]).unwrap_or_default();
    match create_dir_all(OUTPUT_DIR) {
        Err(e) => println!("Error: {}", e),
        _ => (),
    };

    // Iterate through the URLs and get the robots.txt files
    for url in urls.iter() {
        url_mod::download(OUTPUT_DIR.to_string(), url.to_owned(), rt);
    }

    // Exit program
    Ok(())
}
