/// Filename:   test_perceptron.rs
/// Author:     Peinan ZHANG
/// Created at: 2017-05-19

use std::io::{BufReader, BufWriter, Lines};
use std::io::prelude::*;
use std::fs::File;
use std::error::Error;
use std::collections::HashMap;


static N_EPOCH: u32 = 10;

/// Load inputs to lines
fn load_data(path: &str) -> Lines<BufReader<File>> {
    let file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", &path, Error::description(&why)),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);

    return reader.lines();
}


/// Load model file
fn load_model(path: &str) -> HashMap<String, i32> {
    let mut model = HashMap::new();
    for line in load_data(path) {
        let l = line.unwrap();
        let splits: Vec<_> = l.split('\t').collect();
        let word = splits[0].to_string();
        let weight: i32 = splits[1].to_string().parse().unwrap();

        model.insert(word, weight);
    }

    model
}


/// Write model to file
fn dump_model(model: &HashMap<String, i32>, path: &str) {
    let file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", &path, Error::description(&why)),
        Ok(file) => file,
    };

    let mut writer = BufWriter::new(file);

    for (word, weight) in model.iter() {
        write!(&mut writer, "{}\t{}\n", word, weight.to_string());
    }
}


/// Parse one line
fn parse_line(line: &String) -> (i8, HashMap<String, u8>) {
    let mut x_wordcount = HashMap::new();
    let splits: Vec<_> = line.trim_right().split('\t').collect();
    let y: i8 = splits[0].to_string().parse().unwrap();
    let words: Vec<&str> = splits[1].split(' ').collect();
    for word in words {
        *x_wordcount.entry(word.to_string()).or_insert(1) += 1;
    }

    (y, x_wordcount)
}


/// Predict one set
fn predict_one(x: &HashMap<String, u8>, w: &HashMap<String, i32>) -> i8 {
    let mut wx: i32 = 0;
    for (word, _) in x.iter() {
        wx += match w.contains_key(word) {
            true => w[word] * (x[word] as i32),
            false => 0,
        };
    }
    let y_ = sign((wx as f64));

    y_
}


fn sign(x: f64) -> i8 {
    match x < 0. {
        true => -1,
        false => 1,
    }
}


fn main() {
    let in_fp = "data/titles-en-test.small.labeled";
    let model_fp = "model/titles-en-train.small.model";

    let mut word_model = load_model(model_fp);
    let mut n_total = 0;
    let mut n_correct = 0;

    for (k, v) in word_model.iter() {
        println!("{}: {}", k, v);
    }

    for line in load_data(in_fp) {
        let (y, x) = parse_line(&line);
        let y_pred = predict_one(&x, &word_model);

        n_total += 1;
        n_correct += match y_pred == y {
            true => 1,
            _ => 0,
        };

        // println!("[{}] {}\nPRED: {}, TRUE: {}", y_pred == y, l, y_pred, y);
    }
}
