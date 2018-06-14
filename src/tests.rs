pub fn has_someone_won(
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
        }
        else if (row2[0] == player) && (row2[1] == player) && (row2[2] == player){
            mention_the_winner(player.to_string());
            winner = player.to_string();
        }
        else if (row3[0] == player) && (row3[1] == player) && (row3[2] == player){
            mention_the_winner(player.to_string());
            winner = player.to_string();
        }



        else if (row1[0] == player) && (row2[0] == player) && (row3[0] == player){
            mention_the_winner(player.to_string());
            winner = player.to_string();
        }
        else if (row1[1] == player) && (row2[1] == player) && (row3[1] == player){
            mention_the_winner(player.to_string());
            winner = player.to_string();
        }
        else if (row1[2] == player) && (row2[2] == player) && (row3[2] == player){
            mention_the_winner(player.to_string());
            winner = player.to_string();
        }


        else if (row1[0] == player) && (row2[1] == player) && (row3[2] == player){
            mention_the_winner(player.to_string());
            winner = player.to_string();
        }
        else if (row3[0] == player) && (row2[1] == player) && (row1[2] == player){
            mention_the_winner(player.to_string());
            winner = player.to_string();
        }
    }
    fn mention_the_winner(winner: String) {
        println!("The winner is {}", winner);
    }
    winner.to_string()
}
