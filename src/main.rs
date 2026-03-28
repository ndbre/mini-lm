mod tokenizer;

use tokenizer::BPETokenizer;

fn main() {
    let input = std::fs::read_to_string("example.txt").unwrap();

    let tokenizer = BPETokenizer::train(&input, 500);

    println!("{:#?}", tokenizer);

    let encoded = tokenizer.encode("Hello, World.");

    println!("Encoding 'Hello, World.': {:?}", encoded);
    println!("Decoded: {:?}", tokenizer.decode(&encoded));
}
