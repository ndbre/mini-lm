mod io;
mod ngram;
mod tokenizer;

use ngram::NgramModel;
use tokenizer::BPETokenizer;
use tokenizer::Id;

use std::env;
use std::fs;
use std::process;
use std::time;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }

    match args[1].as_str() {
        "train" => handle_train(&args),
        "generate" => handle_generate(&args),
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_usage();
            process::exit(1);
        }
    }
}

fn print_usage() {
    println!("Usage:");
    println!("\tmini-lm train    <input file> <output file> <num merges> <N>");
    println!("\tmini-lm generate <model file> <seed phrase> <num tokens>")
}

fn parse_usize(s: &str, name: &str) -> usize {
    s.parse::<usize>().unwrap_or_else(|_| {
        eprintln!("error: '{}' must be a positive integer, got {:?}", name, s);
        process::exit(1);
    })
}

fn handle_train(args: &[String]) {
    if args.len() < 6 {
        eprintln!("Error: 'train' requires 4 arguments.");
        print_usage();
        process::exit(1);
    }

    let input_file = &args[2];
    let model_output = &args[3];
    let num_merges: usize = parse_usize(&args[4], "num_merges");
    let n: usize = parse_usize(&args[5], "n");

    if n < 2 {
        eprintln!("Error: n must be at least 2.");
        process::exit(1);
    }

    println!("Reading input file: {}", input_file);
    let text = fs::read_to_string(input_file).unwrap_or_else(|e| {
        eprintln!("Error reading {}: {}", input_file, e);
        process::exit(1);
    });

    println!("Input: {} characters, {} unique chars", text.len(), {
        let mut chars: Vec<char> = text.chars().collect();
        chars.sort();
        chars.dedup();
        chars.len()
    });

    let mut start = time::Instant::now();
    println!(
        "\n\x1b[1mStep 1/3\x1b[0m - Training BPE tokenizer (num_merges={})...",
        num_merges
    );
    let tokenizer = BPETokenizer::train(&text, num_merges);
    let mut elapsed_time = start.elapsed();
    println!("Step 1 took {}s.", elapsed_time.as_secs());

    start = time::Instant::now();
    println!("\n\x1b[1mStep 2/3\x1b[0m - Encoding corpus with trained tokenizer...");
    let tokens = tokenizer.encode(&text);
    println!("  {} characters -> {} tokens", text.len(), tokens.len());
    println!(
        "  Compression ratio: {:.2}x",
        text.len() as f64 / tokens.len() as f64
    );
    elapsed_time = start.elapsed();
    println!("Step 2 took {}s.", elapsed_time.as_secs());

    start = time::Instant::now();
    println!(
        "\n\x1b[1mStep 3/3\x1b[0m - Training N-gram model (n={})...",
        n
    );
    let model = NgramModel::train(&tokens, n);
    elapsed_time = start.elapsed();
    println!("Step 3 took {}s.", elapsed_time.as_secs());

    println!("\nSaving model to: {}", model_output);
    io::save(model_output, &tokenizer, &model).unwrap_or_else(|e| {
        eprintln!("Error saving model: {}", e);
        process::exit(1);
    });

    println!("\nDone!");
    println!("  Vocab size:       {}", tokenizer.vocab.size());
    println!("  Merge rules:      {}", tokenizer.merges.len());
    println!("  N-gram order:     {}", model.n);
    println!("  Unique contexts:  {}", model.table.len());
}

fn handle_generate(args: &[String]) {
    if args.len() < 5 {
        eprintln!("Error: 'generate' requires 3 arguments.");
        print_usage();
        process::exit(1);
    }

    let model_file = &args[2];
    let seed_text = &args[3];
    let num_tokens: usize = parse_usize(&args[4], "num_tokens");

    println!("Loading model from: {}", model_file);
    let (tokenizer, model) = io::load(model_file).unwrap_or_else(|e| {
        eprintln!("Error loading model: {}", e);
        process::exit(1);
    });

    println!("Encoding seed: {:?}", seed_text);
    let seed_tokens = tokenizer.encode(seed_text);

    if seed_tokens.is_empty() {
        eprintln!(
            "Error: seed text produced no tokens. Make sure the seed characters exist in the training vocab."
        );
        process::exit(1);
    }

    println!("Generating {} tokens...\n", num_tokens);
    let output_tokens = model.generate(&seed_tokens, num_tokens);

    let output_text = tokenizer.decode(&output_tokens);

    println!("{}", "-".repeat(60));
    println!("{}", output_text);
    println!("{}", "-".repeat(60));
    println!(
        "\n({} tokens generated)",
        output_tokens.len().saturating_sub(seed_tokens.len())
    );
}
