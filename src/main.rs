use tokio;
use reqwest;
use std::env;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::{create_dir_all, File};

static OUTPUT_DIR: &str = "output";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create the tokio runtime
    let mut rt = tokio::runtime::Runtime::new().unwrap();

    // Read the arguments passed to the program
    let args: Vec<String> = env::args().collect();

    // Make sure the user supplied only 1 additional argument
    if args.len() != 2 {
        println!("This program takes 1 argument, which is the path to the list of URLs!");
        return Ok(());
    }

    // Get the URLs and create output directory
    let urls: Vec<String> = import_urls(&*args[1]).unwrap_or_default();
    match create_dir_all(OUTPUT_DIR) {
        Err(e) => println!("Error: {}", e),
        _ => (),
    };

    // Iterate through the URLs and get the robots.txt files
    for url in urls.iter() {
        let hurl = String::from("http://") + &*url + "/robots.txt";
        let surl = String::from("https://") + &*url + "/robots.txt";
        let full_urls: [String; 2] = [hurl, surl];

        for url_t in full_urls.iter() {
            // Initialize the output file path
            let mut o_file: String = String::from(OUTPUT_DIR);

            // Add the http type to the output file path
            let url_p = (&*url_t).split("://");
            let v_url_p: Vec<&str> = url_p.collect();
            o_file += &["/", v_url_p[0]].concat();

            // Separate the file from the url
            let url_p = v_url_p[1].split("/");
            let v_url_p: Vec<&str> = url_p.collect();

            // Add each segment to the output file path
            for segment in v_url_p {
                o_file += &["_", segment].concat();
            }

            // Download robots.txt for url_t
            match rt.block_on(download_robots(&url_t, &o_file)) {
                Err(e) => println!("Error: {}", e),
                Ok(_) => (),
            };
        }
    }

    // Exit program
    Ok(())
}

// Import the urls from a file into the program
fn import_urls(file_path: &str) -> io::Result<Vec<String>> {
    // Create the output
    let mut output: Vec<String> = Vec::new();

    // Create a BufReader for the file at the specified path
    let f = File::open(file_path)?;
    let f = BufReader::new(f);

    // Get the lines from the file
    for line in f.lines() {
        output.push(line.unwrap_or_default());
    }

    // Return the vector of URLs
    Ok(output)
}

async fn download_robots(url: &str, dest: &str) -> Result<(), Box<dyn std::error::Error>> {
    let robots_content = reqwest::get(url)
        .await?
        .text()
        .await?;

    let mut file = match File::create(dest) {
        Err(e) => panic!("Unable to create {}\nError: {}", dest, e),
        Ok(file) => file,
    };

    match file.write_all(robots_content.as_bytes()) {
        Err(e) => panic!("Unable to write to {}\nError: {}", dest, e),
        Ok(_) => (),
    }

    Ok(())
}
