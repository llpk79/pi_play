use pi_play_lib::dot_matrix::DotMatrix;

fn main() {
    let mut dot_matrix = DotMatrix::new();
    println!("Here we go!");
    loop {
        let message = dot_matrix.get_user_input();
        match &message[..] {
            "x" => break,
            _ => dot_matrix.display_message(message),
        };
    }
}
