mod draw;
mod tests;

fn main() {
    println!("Welcome to my noughts and crosses game made in rust.");
    let mut row1 = ["x","x"," "];
    let mut row2 = ["0","x","  "];
    let mut row3 = ["0","x","0"];
    println!("Crosses goes first.");

    draw::draw_game_board(&row1, &row2, &row3);
    let winner = tests::has_someone_won(&row1, &row2, &row3);
    println!("winner {}", winner)
}




