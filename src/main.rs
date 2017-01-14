extern crate regex;


extern crate time;
use time::PreciseTime;

use std::env; // for command line args

use std::io::BufReader;
use std::fs::File;
use regex::Regex;
use std::io::BufRead;


fn main() {
    let word = match env::args().nth(1) {
        None =>  "alabasterpolyaster".to_string(),
        Some(ref w) => w.to_string()
    };

   // a.unwrap_or("bar".to_string())

    let start = PreciseTime::now();
    // Create a path to the desired file
    let f = File::open("de-en.txt").unwrap();
    let file = BufReader::new(&f);
    
    let re = Regex::new(&word).unwrap();

    for line in file.lines() {
        
        let l = match line {
           Err(_) => "Problem".to_string(),
           Ok(l) => l
        }; 


        if re.is_match(&l) {
        println!("Line: {}", &l) 
        }    }
    println!("Finished Running"); 
    let end = PreciseTime::now();

    println!("It took {} seconds.", start.to(end));

}

// Prints each argument on a separate line