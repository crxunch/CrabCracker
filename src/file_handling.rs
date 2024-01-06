use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufRead};
use encoding_rs::ISO_8859_10;
use encoding_rs_io::DecodeReaderBytesBuilder;

pub fn generate_list(filename: &str) -> Result<Vec<String>, Box<dyn Error>> {
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
