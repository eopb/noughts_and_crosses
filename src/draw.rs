// Prints out the game board

pub fn draw_game_board(
    row1: &[&str; 3],
    row2: &[&str; 3],
    row3: &[&str; 3]
    ) {
    println!("{}|{}|{}", row1[0], row1[1] ,row1[2]);
    println!("_ _ _");
    println!("{}|{}|{}", row2[0], row2[1] ,row2[2]);
    println!("_ _ _");
    println!("{}|{}|{}", row3[0], row3[1] ,row3[2]);
}
