use std::{fs::File, io::{BufRead, BufReader}};
use rand::Rng;

fn t_read(filename: &str) -> Result<String, String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let split_collection: Vec<Vec<u8>> = reader.split(b'%').map(|l| l.unwrap()).collect();
    let index = rand::thread_rng().gen_range(0..split_collection.len());

    let result = String::from_utf8(split_collection.get(index).unwrap().to_vec()).unwrap().trim().to_string();
        
    Ok(result)
}

fn main() {
    let line = t_read("test.txt").unwrap();
    println!("{}", line);
}
