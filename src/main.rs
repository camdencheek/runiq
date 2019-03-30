use std::io;
use std::io::prelude::*;
use std::io::BufWriter;
use std::io::{Read, Write};
use std::collections::HashSet;
use regex::Regex;
use clap::{Arg, App};

fn main() {
    let matches = App::new("runiq")
        .version("0.1.0")
        .author("Camden Cheek <ccheek22@gmail.com>")
        .arg(Arg::with_name("regex")
            .short("r")
            .long("regex")
            .takes_value(true)
            .help("A regex pattern whose capture groups are used to determine uniqueness. \
                   Any non-captured portion of the line will not be used to determine uniqueness.")
        ).get_matches();


    let stdin = io::stdin();
    let mut stdout = io::stdout();
    // TODO buffer output with reasonable flush interval

    match matches.value_of("regex") {
        Some(r) => {
            let mut regex_tester = RegexUniqueTester::new(r.to_string()).expect("Failed to make regex");
            filter_input(&stdin, &mut stdout, &mut regex_tester);
        },
        None => {
            let mut hash_tester = HashUniqueTester::new();
            filter_input(&stdin, &mut stdout, &mut hash_tester);
        },
    };


}

fn filter_input<U,V>(stdin: &io::Stdin, stdout: &mut U, tester: &mut V)
    where U: Write, V: UniqueTester
{

    for line in stdin.lock().lines() {
        let line = line.expect("Failed to unwrap line");

        if !tester.visit(&line) {
            writeln!(stdout, "{}", line);
        }
    }
}

trait UniqueTester {
    fn visit(&mut self, line: &String) -> bool;
}

struct HashUniqueTester {
    visited: HashSet<String>,
}

impl HashUniqueTester {
    fn new() -> Self {
        HashUniqueTester {
            visited: HashSet::new(),
        }
    }
}

impl UniqueTester for HashUniqueTester {
    fn visit(&mut self, line: &String) -> bool {
        match self.visited.contains(line) {
            true => true,
            false => {
                self.visited.insert(line.clone());
                false
            }
        }
    }
}

struct RegexUniqueTester {
    re: Regex,
    visited: HashSet<Vec<String>>,

}

impl RegexUniqueTester {
    fn new(regexp: String) -> Result<Self, regex::Error> {
        let tester = RegexUniqueTester {
            re: Regex::new(&regexp)?,
            visited: HashSet::new(),
        };

        Ok(tester)
    }
}

impl UniqueTester for RegexUniqueTester {
    fn visit(&mut self, line: &String) -> bool {
        let caps = self.re.captures(line);
        let caps: Vec<String> = match caps {
            None => return false,
            Some(c) => {
                c.iter()
                    .skip(1)
                    .map(|x| x.unwrap().as_str().to_string())
                    .collect()
            }
        };


        match self.visited.contains(&caps) {
            true => true,
            false => {
                self.visited.insert(caps);
                false
            }
        }
    }
}
