use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufRead};
use ntlm_hash::*;
use encoding_rs::ISO_8859_10;
use encoding_rs_io::DecodeReaderBytesBuilder;

mod args;
mod hash_algorithms;

fn main() {

    let arguments = args::get_arguments();

    let hashlist_file = arguments.get_one::<String>("hashlist").unwrap();
    let wordlist_file = arguments.get_one::<String>("wordlist").unwrap();
    let algorithm_type = arguments.get_one::<String>("algorithm").unwrap();

    let mut cracked_passwords: Vec<String> = Vec::new();

    // if arguments.get_one::<String>("shadow").unwrap(); != None {
    //     parse_shadow();
    // }

    let hash_fn = match algorithm_type.as_str() {
        "0" => ntlm_hash,
        "1" => hash_algorithms::sha256_hash,
        _ => panic!("not a recognized hash"),
    };

    println!("\nGenerating hashlist...");
    let mut hashes = match generate_list(hashlist_file) {
        Ok(items) => items,
        Err(_) => panic!("Error generating hashlist."),
    };

    println!("\nGenerating wordlist...");
    let words = match generate_list(wordlist_file) {
        Ok(items) => items,
        Err(_) => panic!("Error generating wordlist."),
    };

    println!("\nCracking...\n");

    for word in &words {
        if word.len() > 15 {
            continue
        }

        for j in 0..(hashes.len()) {
            if hashes[j] == hash_fn(&word) {
                cracked_passwords.push(String::from(format!("{}: {}", word, hashes[j])));
                hashes.remove(j);
                break
            }
        }
    }

    println!("Cracked Hashes: \n{:#?}", cracked_passwords);
    println!("\nUncracked Hashes: \n{:#?}", hashes);
}

fn generate_list(filename: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut items: Vec<String> = Vec::new();
    
    let file = File::open(filename)?;
    let reader = BufReader::new(
        DecodeReaderBytesBuilder::new()
            .encoding(Some(ISO_8859_10))
            .build(file)
    );

    for line in reader.lines() {
        match line {
            Ok(item) => items.push(item),
            Err(_) => continue,
        };
    }

    Ok(items)
}
