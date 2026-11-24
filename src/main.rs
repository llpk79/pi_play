use pi_play_lib::dot_matrix::DotMatrix;
use pi_play_lib::dot_matrix::Characters;

fn main() {
    let dot_matrix = DotMatrix::new();
    let characters = Characters::new();
    println!("Here we go!");
    dot_matrix.display_user_message(characters)
}
