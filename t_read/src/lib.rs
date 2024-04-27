use rand::Rng;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn t_read(filename: &str) -> Result<String, ()> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let string_collection: Vec<String> = reader
        .split(b'%')
        .map(|l| String::from_utf8(l.unwrap()).unwrap())
        .collect();
    let index = rand::thread_rng().gen_range(0..string_collection.len());

    let result = string_collection.get(index).unwrap().trim().to_string();

    Ok(result)
}