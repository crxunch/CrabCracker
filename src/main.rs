use ntlm_hash::*;
use std::thread::{self, JoinHandle};

mod args;
mod hash_algorithms;
mod file_handling;

fn main() {

    let arguments = args::get_arguments();

    let hashlist_file = arguments.get_one::<String>("hashlist").unwrap();
    let wordlist_file = arguments.get_one::<String>("wordlist").unwrap();
    let algorithm_type = arguments.get_one::<String>("algorithm").unwrap();

    let max_pass_len: usize = match algorithm_type.as_str() {
        "0" => 15,
        _ => 18_446_744_073_709_551_615,
    };

    // if arguments.get_one::<String>("shadow").unwrap(); != None {
    //     parse_shadow();
    // }

    let hash_fn = match algorithm_type.as_str() {
        "0" => ntlm_hash,
        "1" => hash_algorithms::sha256_hash,
        "11" => hash_algorithms::sha512_hash,
        "2" => hash_algorithms::md5_hash,
        _ => panic!("not a recognized hash"),
    };

    println!("\nGenerating hashlist...");
    let hashes = match file_handling::generate_list(hashlist_file) {
        Ok(items) => items,
        Err(_) => panic!("Error generating hashlist."),
    };

    println!("\nGenerating wordlist...");
    let words = match file_handling::generate_list(wordlist_file) {
        Ok(items) => items,
        Err(_) => panic!("Error generating wordlist."),
    };

    println!("\nCracking...\n");

    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    for hash in hashes {
        let words_clone = words.clone();
        
        let handle = thread::spawn(move || {
            for word in &words_clone {
                if word.len() > max_pass_len {
                    continue
                }
                if *hash == hash_fn(&word) {
                    println!("{}: {}", word, hash);
                    break
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Error joining thread");
    }
    
}
