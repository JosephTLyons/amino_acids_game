use std::io;
use std::time::SystemTime;

struct AminoGroup<'a> {
    items: Vec<&'a str>,
    amino_type: &'a str,
}

fn main() {
    game();
}

fn game() {
    print_rules();

    let mut non_polar = AminoGroup {
        items: vec![
            "glycine",
            "alanine",
            "valine",
            "leucine",
            "isoleucine",
            "methionine",
            "tryptophan",
            "phenylalanine",
            "proline",
        ],
        amino_type: "Non-Polar",
    };

    let mut polar = AminoGroup {
        items: vec![
            "serine",
            "threonine",
            "cysteine",
            "tyrosine",
            "asparagine",
            "glutamine",
        ],
        amino_type: "Polar",
    };

    let mut acidic = AminoGroup {
        items: vec!["aspartic", "glutamic"],
        amino_type: "Acidic",
    };

    let mut basic = AminoGroup {
        items: vec!["lysine", "arginine", "histidine"],
        amino_type: "Basic",
    };

    let start_time = SystemTime::now();

    for amino_group in [&mut non_polar, &mut polar, &mut acidic, &mut basic] {
        guess_amino_acids(amino_group)
    }

    let duration = SystemTime::now()
        .duration_since(start_time)
        .expect("Error timing run.");
    println!("This run took: {:?}", duration);

    let number_guessed: usize =
        20 - non_polar.items.len() - polar.items.len() - acidic.items.len() - basic.items.len();

    println!("Amino acids guessed correctly: {}", number_guessed);

    if number_guessed > 0 {
        println!(
            "Average time of each guess: {:?}",
            duration / number_guessed as u32
        );
    }

    println!("The game is over!");
}

fn print_rules() {
    println!("What can be typed:");
    println!("-The name of an amino acid");
    println!("-\"hint\"");
    println!("-\"skip\"");
    println!();
}

fn guess_amino_acids(amino_acids: &mut AminoGroup) {
    let original_amount = amino_acids.items.len();
    println!(
        "Guess the {} {} amino acids:",
        original_amount, amino_acids.amino_type
    );

    let mut input: String = String::new();

    while !amino_acids.items.is_empty() {
        println!(
            "{}/{} ({}%)",
            original_amount - amino_acids.items.len(),
            original_amount,
            ((original_amount - amino_acids.items.len()) as f32 / original_amount as f32) * 100.0
        );

        io::stdin()
            .read_line(&mut input)
            .expect("Couldn't read input.");

        // Remove newline from stdin and any other whitespace
        input = input.trim().to_string();

        // Skip this group of amino acids
        if input.to_lowercase() == "skip" {
            println!("The remaining {} amino acids are:", amino_acids.amino_type);
            print_acids(&amino_acids.items);
            return;
        }
        // Print first first letter of first element as a hint
        else if input.to_lowercase() == "hint" {
            println!("{}", amino_acids.items[0].as_bytes()[0] as char);
        }

        // Check if guess is correct
        for x in 0..amino_acids.items.len() {
            if amino_acids.items[x] == input.to_lowercase() {
                amino_acids.items.remove(x);
                break;
            }
        }

        input.clear();
    }
}

fn print_acids(vec: &[&str]) {
    for (iter_count, vec_item) in vec.iter().enumerate() {
        println!("{}) {}", iter_count + 1, vec_item);
    }
}
