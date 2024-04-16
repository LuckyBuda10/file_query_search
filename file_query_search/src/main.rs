use std::fs::File;
use std::{process, env};
use std::io::{self, Read};

fn main() {
    let args = env::args();
    
    let args = match UserArgs::new(args.collect()) {
        Ok(args) => args,
        Err(e) => {
            println!("Argument Error: {}", e);
            process::exit(1);
        }
    };

    let file = match try_open_file(&args.filename) {
        Ok(file) => file,
        Err(e) => {
            println!("Error opening file: {}", e);
            process::exit(1);
        }
    };

    let results = search_file(file, &args.search_word);

    if results.len() == 0 {
        println!("No results for given query!");
    } else {
        for line in results {
            println!("{}", line);
        }
    }
}


struct UserArgs {
    filename: String,
    search_word: String,
}

impl UserArgs {
    fn new(args: Vec<String>) -> Result<UserArgs, &'static str> {
        if args.len() != 3 {
            return Err("Must give 2 arguments!");
        }

        Ok(UserArgs {
            filename: args[1].clone(),
            search_word: args[2].clone()
        })
    }
}

fn try_open_file(filename: &str) -> Result<File, io::Error> {
    let file = File::open(filename)?;

    Ok(file)
}

fn search_file(mut file: File, search_word: &str) -> Vec<String> {
    let mut results = Vec::new();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap_or_else(|err| {
        println!("Error reading file: {}", err);
        process::exit(1);
    });

    for line in contents.lines() {
        if line.contains(search_word) {
            results.push(line.to_string());
        }
    }

    results
}