fn main() {
    println!("Welcome to my noughts and crosses game made in rust.");
    let mut row1 = ["1","2","3"];
    let mut row2 = ["4","5","6"];
    let mut row3 = ["7","8","9"];


    draw_game_board(
        row1[0].to_string(),
        row1[1].to_string(),
        row1[2].to_string(),
        row2[0].to_string(),
        row2[1].to_string(),
        row2[2].to_string(),
        row3[0].to_string(),
        row3[1].to_string(),
        row3[2].to_string(),);
}

// Prints out the game board
fn draw_game_board(
        row1col1: String,
        row1col2: String,
        row1col3: String,
        row2col1: String,
        row2col2: String,
        row2col3: String,
        row3col1: String,
        row3col2: String,
        row3col3: String,
    ) {
    println!("{}|{}|{}", row1col1, row1col2 ,row1col3);
    println!("_ _ _");
    println!("{}|{}|{}", row2col1, row2col2 ,row2col3);
    println!("_ _ _");
    println!("{}|{}|{}", row3col1, row3col2 ,row3col3);
}