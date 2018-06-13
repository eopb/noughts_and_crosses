fn main() {
    println!("Welcome to my noughts and crosses game made in rust.");
    let mut row1 = ["x","x","x"];
    let mut row2 = [" "," "," "];
    let mut row3 = [" "," "," "];
    println!("Crosses goes first.");

    draw_game_board(&row1, &row2, &row3);
    let winner = has_someone_won(&row1, &row2, &row3);
}

// Prints out the game board
fn draw_game_board(
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

fn has_someone_won(
    row1: &[&str; 3],
    row2: &[&str; 3],
    row3: &[&str; 3]
    ) -> std::string::String {
    
    let player = ["x", "0"] ;
    let mut winner = "none".to_string();
    for &player in player.iter() {
        if (row1[0] == player) && (row1[1] == player) && (row1[2] == player){
            mention_the_winner(player.to_string());
            winner = player.to_string();
            winner
        }
        else if (row2[0] == player) && (row2[1] == player) && (row2[2] == player){
            mention_the_winner(player.to_string());
        }
        else if (row3[0] == player) && (row3[1] == player) && (row3[2] == player){
            mention_the_winner(player.to_string());
        }



        else if (row1[0] == player) && (row2[0] == player) && (row3[0] == player){
            mention_the_winner(player.to_string());
        }
        else if (row1[1] == player) && (row2[1] == player) && (row3[1] == player){
            mention_the_winner(player.to_string());
        }
        else if (row1[2] == player) && (row2[2] == player) && (row3[2] == player){
            mention_the_winner(player.to_string());
        }


        else if (row1[0] == player) && (row2[1] == player) && (row3[2] == player){
            mention_the_winner(player.to_string());
        }
        else if (row3[0] == player) && (row2[1] == player) && (row1[2] == player){
            mention_the_winner(player.to_string());
        }
    }
    fn mention_the_winner(winner: String) {
        println!("The winner is {}", winner);
    }
    winner.to_string()
}

