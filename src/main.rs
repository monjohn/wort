extern crate regex;

// for command line args
extern crate getopts;
use getopts::Options;
use std::env; 
// reading files
use std::io::BufReader;
use std::fs::File;
use regex::Regex;
use std::io::BufRead;

extern crate time;
use time::PreciseTime;


fn main() {

    // Parsing args
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optflag("a", "all",
                 "include all occurances of word, not just definition");
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("o", "out",
                 "output suitable for appendint to a csv");
    let matches = match opts.parse(&args[2..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    // Check for help flags or if first arg is a help flag
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    
    let word = match env::args().nth(1) {
        None => panic!("You must enter a word"),
        Some(s) => {
            if s == "help" || s == "-h" || s == "--help" {
                print_usage(&program, opts);
                return;
            } else {
                s.clone()
            }
        }
    };
    

    let csv: bool = if matches.opt_present("o") { true } else { false };

    let start = PreciseTime::now();

    // Create a path to the desired file
    let f = File::open("/Users/monte/.bin/de-en.txt").unwrap();
    let file = BufReader::new(&f);

    // regex for word
    let word_re = "^".to_string() + &word + "[\\s;]";
    let re = Regex::new(&word_re).unwrap();

    // create regex if option of examples 
    let example_re = if matches.opt_present("a") {
        Some(Regex::new(&word).unwrap())
    } else {
        None
    };

    for line in file.lines() {

        let l = match line {
            Err(_) => continue,
            Ok(l) => l,
        };

        if re.is_match(&l) {
            println!("");
            print_match(&l, csv);
            continue;
        }

        if let &Some(ref e_re) = &example_re {
            if e_re.is_match(&l) {
                print_match(&l, csv);
            }
        }
    }

    if !csv {
        println!("Finished Running");
        let end = PreciseTime::now();

        println!("It took {} seconds.", start.to(end));
    }
}

fn print_match(line: &str, csv: bool) {
    //  let result = String::new();
    let ger_eng: Vec<&str> = line.split("::").collect();
    let eng = ger_eng[1].split(" | ");
    let ger = ger_eng[0].split(" | ");
    let mut pairs = ger.zip(eng);
    while let Some((g, e)) = pairs.next() {
        if csv {
            println!("{},{}", g, e);
        } else {
            println!("{0: <50} – {1: <50}", g, e);
        }
    }
}


fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}


// Prints each argument on a separate line