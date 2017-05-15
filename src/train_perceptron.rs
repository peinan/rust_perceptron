/// Filename:   train_perceptron.rs
/// Author:     Peinan ZHANG
/// Created at: 2017-05-12

use std::io::{self, BufReader, Lines};
use std::io::prelude::*;
use std::fs::File;
use std::error::Error;
use std::path::Path;
use std::collections::HashMap;

static N_EPOCH: u32 = 10;

fn load_data(path: String) -> Lines<BufReader<File>> {
    let file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", &path, Error::description(&why)),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);

    return reader.lines();
}


fn main() {
    let in_fp = "titles-en-train.labeled";
    for line in load_data(in_fp.to_string()) {
        let line_str = line.unwrap();
        let splits: Vec<_> = line_str.trim_right().split('\t').collect();
    }
}
