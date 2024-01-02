use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufRead, Read};
use ntlm_hash::*;
use encoding_rs::ISO_8859_10;
use encoding_rs_io::DecodeReaderBytesBuilder;

mod args;

fn main() -> Result<(), Box<dyn Error>> {

    let arguments = args::get_arguments();

    let hashlist_file = arguments.get_one::<String>("hashlist").unwrap();
    let wordlist_file = arguments.get_one::<String>("wordlist").unwrap();
    let algorithm_type = arguments.get_one::<String>("algorithm").unwrap();

    let mut cracked_passwords: Vec<String> = Vec::new();

    // if arguments.get_one::<String>("shadow").unwrap(); != None {
    //     parse_shadow();
    // }

    println!("\nGenerating hashlist...");
    let mut hashes = lines_from_file(hashlist_file);

    println!("\nGenerating wordlist...");
    let mut words: Vec<String> = Vec::new();
    let file = File::open(wordlist_file)?;
    let mut reader = BufReader::new(
        DecodeReaderBytesBuilder::new()
            .encoding(Some(ISO_8859_10))
            .build(file));
    for line in reader.lines() {
        match line {
            Ok(word) => words.push(word),
            Err(..) => continue,
        };
    }

    println!("\nCracking...\n");

    for word in &words {
        if word.len() > 15 {
            continue
        }

        for j in 0..(hashes.len()) {
            if hashes[j] == ntlm_hash(&word) {
                cracked_passwords.push(String::from(format!("{}: {}", word, hashes[j])));
                hashes.remove(j);
                break
            }
        }
    }

    println!("Cracked Hashes: \n{:#?}", cracked_passwords);
    println!("\nUncracked Hashes: \n{:#?}", hashes);

    return Ok(());
}


fn lines_from_file(filename: &str) -> Vec<String> {
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => panic!("no such file"),
    };
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .ok()
        .expect("failed to read!");
    let lines: Vec<String> = file_contents.split("\n")
        .map(|s: &str| s.to_string())
        .collect();
    lines
}
