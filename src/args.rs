use clap::{Arg, command, ArgMatches};

pub fn get_arguments() -> ArgMatches {

    let match_result = command!().arg(
        Arg::new("hashlist")
            .short('f')
            .required(true)
            .help("The file of hashes to be cracked.")
    )
    .arg(
        Arg::new("wordlist")
            .short('w')
            .required(true)
            .help("The file of words to be used to crack the hashes.")
    )
    .arg(
        Arg::new("algorithm")
            .short('a')
            .required(true)
            .help("The hashing algorithm in use.")
    )
    .arg(
        Arg::new("shadow")
            .long("shadow")
            .help("Tells CrabCracker that the supplied file is an /etc/shadow file. An argument is required but will not be processed, pass whatever you want.")
    )
    .about("Barebones password cracking tool written entirely in Rust.")
    .get_matches();

    return match_result
}
