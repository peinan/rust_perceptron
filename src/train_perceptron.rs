/// Filename:   train_perceptron.rs
/// Author:     Peinan ZHANG
/// Created at: 2017-05-12

use std::io::{BufReader, Lines};
use std::io::prelude::*;
use std::fs::File;
use std::error::Error;
use std::collections::HashMap;


static N_EPOCH: u32 = 10;

/// Load inputs to lines
fn load_data(path: String) -> Lines<BufReader<File>> {
    let file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", &path, Error::description(&why)),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);

    return reader.lines();
}


/// Parse one line
fn parse_line(line: String) -> (i8, HashMap<String, u8>) {
    let mut x_wordcount = HashMap::new();
    let splits: Vec<_> = line.trim_right().split('\t').collect();
    let y: i8 = splits[0].to_string().parse().unwrap();
    let words: Vec<&str> = splits[1].split(' ').collect();
    for word in words {
        if !x_wordcount.contains_key(word) {
            x_wordcount.insert(word.to_string(), 1);
        } else {
            *x_wordcount.get_mut(word).unwrap() += 1;
        }
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
    let mut f_x = 1;
    if x < 0. {
        f_x = -1;
    }

    f_x
}


fn main() {
    let mut word_model: HashMap<String, i32> = HashMap::new();
    let in_fp = "data/titles-en-train.small.labeled";

    for line in load_data(in_fp.to_string()) {
        let (y, x) = parse_line(line.unwrap());
        let y_pred = predict_one(&x, &word_model);
        if y_pred != y {
            match y_pred {
                1 => {
                    for (word, _) in x.iter() {
                        // println!("{}", word);
                        // let weight = word_model.entry(word).or_insert(0);
                        // *weight += -1;
                        word_model[word] = match x.contains_key(word) {
                            true => word_model[word] + -1,
                            false => 0,
                        }
                    }
                }
                -1 => {
                    for (word, _) in x.iter() {
                        println!("{}", word);
                        // let weight = word_model.entry(word).or_insert(0);
                        // *weight += 1;
                        word_model[word] = match x.contains_key(word) {
                            true => word_model[word] + 1,
                            false => 0,
                        }
                    }
                }
                _ => {}
            }
        }
    }

    for (k, v) in word_model.iter() {
        println!("{}: {}", k, v);
    }
}
