use std::collections::HashMap;

fn main() {
    type MyMap = HashMap<String, i8>;
    let mut word_model: MyMap = HashMap::new();
    word_model.insert("Key".to_string(), 1);
    let x: MyMap = HashMap::new();
    let word: &str = "Key";
    let word2: &str = "Key2";

    *word_model.entry(word.to_string()).or_insert(0) += 1;
    *word_model.entry(word2.to_string()).or_insert(0) += 1;

    for (k, v) in word_model.iter() {
        println!("{}: {}", k, v);
    }
}
