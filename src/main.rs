use std::io;
mod draw;
mod tests;

fn main() {
    println!("Welcome to my noughts and crosses game made in rust.");
    let mut row1 = [" "," "," "];
    let mut row2 = [" "," "," "];
    let mut row3 = [" "," "," "];
    println!("Crosses goes first.");

    println!("The board looks like this.");
    draw::draw_game_board(&row1, &row2, &row3);
    println!("You are the *");
    println!("To move the star left type a and hit enter");
    println!("To move the star right type d and hit enter");
    println!("To move the star up type w and hit enter");
    println!("To move the star down type s and hit enter");
    println!("To place your cross type q and hit enter");
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    let winner = tests::has_someone_won(&row1, &row2, &row3);
    println!("winner {}", winner)
}
