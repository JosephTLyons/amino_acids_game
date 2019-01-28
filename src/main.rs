#[macro_use] extern crate text_io;

fn main() {
    play_game();
}

fn play_game() {
    let mut non_polar: Vec<&str> = vec!["glycine",
                                        "alanine",
                                        "valine",
                                        "leucine",
                                        "isoleucine",
                                        "methionine",
                                        "tryptophan",
                                        "phenylalanine",
                                        "proline"];

    let mut polar: Vec<&str> = vec!["serine",
                                    "threonine",
                                    "cysteine",
                                    "tyrosine",
                                    "asparagine",
                                    "glutamine"];

    let mut acidic: Vec<&str> = vec!["aspartic",
                                     "glutamic"];

    let mut basic: Vec<&str> = vec!["lysine",
                                    "arginine",
                                    "histidine"];

    guess_amino_acids(&mut non_polar, String::from("non-polar"));
    guess_amino_acids(&mut polar, String::from("polar"));
    guess_amino_acids(&mut acidic, String::from("acidid"));
    guess_amino_acids(&mut basic, String::from("basic"));

    println!("You've guessed all 20 amino acids!  Congratulations!");
}

fn guess_amino_acids(acids: &mut Vec<&str>, amino_type: String) {
    let original_amount = acids.len();
    println!("Guess the {} {} amino acids:", original_amount, amino_type);

    let mut input: String;

    while ! acids.is_empty() {
        println!("{}/{}", original_amount - acids.len(), original_amount);
        input = read!();

        for x in 0..acids.len() {
            if acids[x] == input.to_lowercase() {
                acids.remove(x);
                break;
            }
        }
    }
}

#[allow(dead_code)]
fn print_vector(vec: &[&str]) {
    for x in vec {
        println!("    -{}", x);
    }
}
