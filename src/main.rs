#[macro_use]
extern crate text_io;

fn main() {
    play_game();
}

fn play_game() {
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

    guess_amino_acids (&mut non_polar, "non-polar");
    guess_amino_acids (&mut polar, "polar");
    guess_amino_acids (&mut acidic, "acidid");
    guess_amino_acids (&mut basic, "basic");

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

    let mut input: String;

    while ! amino_acids.is_empty() {
        println! ("{}/{}", original_amount - amino_acids.len(), original_amount);

        input = read!();

        // Skip this group of amino acids
        if input == "skip" {
            println! ("The remaining {} amino acids are:", amino_type);
            print_acids (&amino_acids);
            return;
        }
        // Print first first letter of first element as a hint
        else if input == "hint" {
            println! ("{}", amino_acids[0].as_bytes()[0] as char);
        }

        // Check if guess is correct
        for x in 0..amino_acids.len() {
            if amino_acids[x] == input.to_lowercase() {
                amino_acids.remove (x);
                break;
            }
        }
    }
}

fn print_acids (vec: &[&str]) {
    for (iter_count, vec_item) in vec.iter().enumerate() {
        println! ("{}) {}", iter_count + 1, vec_item);
    }
}
