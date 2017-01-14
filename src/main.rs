extern crate regex;

use std::env; // for command line args

use std::io::BufReader;
use std::fs::File;
use regex::Regex;
use std::io::BufRead;


fn main() {
    match env::args().nth(1) {
        None => println!("You must enter a word. "),
        Some(word) => println!("The definition for {} is", word)
    }

    // Create a path to the desired file
    let f = File::open("de-en.txt").unwrap();
    let file = BufReader::new(&f);
    let re = Regex::new(r"^.*Abakus.*$").unwrap();

    for line in file.lines().take(100) {
        
        let l = match line {
           Err(_) => "Problem".to_string(),
           Ok(l) => l
        }; 


        if re.is_match(&l) {
        println!("Line: {}", &l) 
        }    }
    println!("Finished Running"); 


}

// Prints each argument on a separate line