#[macro_use] extern crate text_io;

fn main() {
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

    println!("Guess the non-polar amino acids:");
    guess_amino_acids(&mut non_polar);

    println!("Guess the polar amino acids:");
    guess_amino_acids(&mut polar);

    println!("Guess the acidic amino acids:");
    guess_amino_acids(&mut acidic);

    println!("Guess the basic amino acids:");
    guess_amino_acids(&mut basic);
}

fn guess_amino_acids(acids: &mut Vec<&str>) {
    let mut input: String;
    let original_amount = acids.len();

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
fn print_array(array: &[&str]) {
    for x in array {
        println!("    -{}", x);
    }
}
