use std::io;
use std::time::{SystemTime};

fn main() {
    game();
}

fn game() {
    print_rules();

    let mut non_polar: Vec<&str> = vec![
        "glycine",
        "alanine",
        "valine",
        "leucine",
        "isoleucine",
        "methionine",
        "tryptophan",
        "phenylalanine",
        "proline",
    ];

    let mut polar: Vec<&str> = vec![
        "serine",
        "threonine",
        "cysteine",
        "tyrosine",
        "asparagine",
        "glutamine",
    ];

    let mut acidic: Vec<&str> = vec![
        "aspartic",
        "glutamic"
    ];

    let mut basic: Vec<&str> = vec![
        "lysine",
        "arginine",
        "histidine"
    ];

    let start_time = SystemTime::now();

    guess_amino_acids (&mut non_polar, "non-polar");
    guess_amino_acids (&mut polar, "polar");
    guess_amino_acids (&mut acidic, "acidic");
    guess_amino_acids (&mut basic, "basic");

    let end_time = SystemTime::now().duration_since (start_time).expect ("Error timing run.");

    println!("This run took: {:?}", end_time);
    println! ("The game is over!");
}

fn print_rules() {
    println! ("What can be typed:");
    println! ("-The name of an amino acid");
    println! ("-\"hint\"");
    println! ("-\"skip\"");
    println!();
}

fn guess_amino_acids (amino_acids: &mut Vec<&str>, amino_type: &str) {
    let original_amount = amino_acids.len();
    println! ("Guess the {} {} amino acids:", original_amount, amino_type);

    let mut input: String = String::new();

    while ! amino_acids.is_empty() {
        println! (
            "{}/{} ({}%)",
            original_amount - amino_acids.len(),
            original_amount,
            ((original_amount - amino_acids.len()) as f32 / original_amount as f32) * 100.0
        );

        io::stdin().read_line (&mut input).expect ("Couldn't read input.");

        // Remove newline from stdin and any other whitespace
        input = input.trim().to_string();

        // Skip this group of amino acids
        if input.to_lowercase() == "skip" {
            println! ("The remaining {} amino acids are:", amino_type);
            print_acids (&amino_acids);
            return;
        }

        // Print first first letter of first element as a hint
        else if input.to_lowercase() == "hint" {
            println! ("{}", amino_acids[0].as_bytes()[0] as char);
        }

        // Check if guess is correct
        for x in 0..amino_acids.len() {
            if amino_acids[x] == input.to_lowercase() {
                amino_acids.remove (x);
                break;
            }
        }

        input.clear();
    }
}

fn print_acids (vec: &[&str]) {
    for (iter_count, vec_item) in vec.iter().enumerate() {
        println! ("{}) {}", iter_count + 1, vec_item);
    }
}
