mod tokenizer;

use std::{env, fs};

use tokenizer::Tokenizer;

fn main() {
    let mut current_path = env::current_dir().unwrap();
    current_path.push("isolated.txt");

    let content_result = fs::read_to_string(current_path);
    let Ok(content_string) = content_result else {
        return;
    };

    let mut tokenizer = Tokenizer::new(content_string);
    let tokens = tokenizer.capture();

    println!("{:?}", tokens);
}
