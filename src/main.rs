fn main() {
    println!("Welcome to my noughts and crosses game made in rust.");
    draw_game_board();
}

// Prints out the game board
fn draw_game_board() {
    let mut test = "x";
    println!("{}|{}|{}", test, test ,test);
    println!("_ _ _");
    println!("{}|{}|{}", test, test ,test);
    println!("_ _ _");
    println!("{}|{}|{}", test, test ,test);
}
