extern crate regex;

// for command line args
extern crate getopts;
use getopts::Options;
use getopts::Matches;
use std::env;
// reading files
use std::io::BufReader;
use std::fs::File;
use regex::Regex;
use std::io::BufRead;

extern crate time;
use time::PreciseTime;


fn main() {
    let dictionary = "/Users/montejohnston/bin/de-en.txt";

    // Parsing args
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optflag("a",
                 "all",
                 "include all occurances of word, not just definition");
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("u",
                 "usage",
                 "shows different usages of the word after the definition");
    opts.optflag("o", "out", "output suitable for appending to a csv");
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

    let start = PreciseTime::now();

    // Create a path to the desired file
    let f = File::open(dictionary).unwrap();
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
            print_match(&l, &matches);
            continue;
        }

        if let &Some(ref e_re) = &example_re {
            if e_re.is_match(&l) {
                print_match(&l, &matches);
            }
        }
    }

    if !matches.opt_present("o") {
        println!("Finished Running");
        let end = PreciseTime::now();

        println!("It took {} seconds.", start.to(end));
    }
}

fn print_match(line: &str, matches: &Matches) {
    let more = matches.opt_present("m");
    let all = matches.opt_present("a");
    let csv = matches.opt_present("o");
    //  let result = String::new();
    if line.is_empty() {
        return;
    }
    let ger_eng: Vec<&str> = line.trim().split("::").collect();

    let eng = ger_eng[1].split(" | ");
    let ger = ger_eng[0].split(" | ");
    let mut pairs = ger.zip(eng);
    let mut subsequent = false;
    while let Some((g, e)) = pairs.next() {
        // only show first result unless more or all
        if (!more && subsequent) && !all {
            break;
        }
        if csv {
            println!("{}, {}", g.trim(), e.trim());
        } else {
            let g = if subsequent {
                format!("– {}", g.trim())
            } else {
                subsequent = true;
                g.to_string()
            };

            println!("{0: <50} – {1: <50}", g, e.trim());
        }
    }
}


fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}
