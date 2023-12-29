use std::fs::File;
use std::io::Read;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);
    let mut hashlist = "hashlist.txt";
    let mut wordlist = "default.txt";
    let mut algorithm = "SHA256";
    if args.len() < 7 {
        panic!("Missing arguments");
    } else {
        for i in 1..(args.len() - 1) {
            if args[i] == "-w" {
                wordlist = &args[i + 1];
            }
            if args[i] == "-h" {
                hashlist = &args[i + 1];
            }
            if args[i] == "-a" {
                algorithm = &args[i + 1];
            }
        }
    }
    
    let hash_list = lines_from_file(hashlist);
    println!("{:?},{:?},{:?},{:?}", hash_list, hashlist, wordlist, algorithm);
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
    let mut lines: Vec<String> = file_contents.split("\n")
        .map(|s: &str| s.to_string())
        .collect();
    lines.remove(lines.len() - 1);
    lines
}

/*
    basic functionality:
        1. the user supplies arguments about hashing algorithm, hashlist or hash, and wordlist
        2. read from wordlist
        3. hash contents of wordlist and compare hashes with contents of hashlist
        4. Store the cracked hashes in a file
    
    neccesary functions:
        1. Common hashing algorithms (MD5, SHAx)
        2. file operations

    future features:
        1. multiple hashes in one file
        2. multithreading
        3. implement parsing for /etc/shadow
*/
