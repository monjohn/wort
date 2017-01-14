extern crate regex;


extern crate time;
use time::PreciseTime;

use std::env; // for command line args

use std::io::BufReader;
use std::fs::File;
use regex::Regex;
use std::io::BufRead;


fn main() {
    let word = env::args().nth(1).expect("You must enter a word.");

    let word_re = "^".to_string() + &word + " ";
    // a.unwrap_or("bar".to_string())

    let start = PreciseTime::now();
    // Create a path to the desired file
    let f = File::open("de-en.txt").unwrap();
    let file = BufReader::new(&f);

    let re = Regex::new(&word_re).unwrap();

    for line in file.lines() {

        let l = match line {
            Err(_) => continue,
            Ok(l) => l,
        };

        if re.is_match(&l) {
            print_match(&l);
            break;
        }
    }
    println!("Finished Running");
    let end = PreciseTime::now();

    println!("It took {} seconds.", start.to(end));

}

fn print_match(line: &str) {
  //  let result = String::new();
    let ger_eng: Vec<&str> = line.split("::").collect();
    let eng = ger_eng[1].split(" | ");
    let ger = ger_eng[0].split(" | ");
    let mut pairs = ger.zip(eng);
    while let Some((g, e)) = pairs.next() {
        println!("{} – {}", g, e);
       
    }
  
}

// Prints each argument on a separate line