use std::fs::File;
use std::io::Read;

mod args;

fn main() {

    let arguments = args::get_arguments();

    let hashlist_file = arguments.get_one::<String>("hashlist").unwrap();
    let wordlist_file = arguments.get_one::<String>("wordlist").unwrap();
    let algorithm_type = arguments.get_one::<String>("algorithm").unwrap();

    // if arguments.get_one::<String>("wordlist").unwrap(); != None {
    //     parse_shadow();
    // }
    
    println!("{}", hashlist_file);
    println!("{}", wordlist_file);
    println!("{}", algorithm_type);
    
    // let hash_list = lines_from_file("hashlist.txt");
    // println!("{:?}", hash_list);
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
