mod tokenizer;

use tokenizer::Tokenizer;

fn main() {
    let mut tokenizer = Tokenizer::new("local object12 = 1444".to_string());
    let tokens = tokenizer.capture();

    println!("{:?}", tokens);
}
