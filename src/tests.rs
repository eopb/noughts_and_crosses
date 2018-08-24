use GameBoard;
use Players;
use TileStatus;
use Winner;

pub fn has_someone_won(current_player: Players, game_board: GameBoard) -> Winner {
    let player_being_tested = [Players::Nought, Players::Cross];
    match current_player {
        Players::Nought => nought(game_board),
        Players::Cross => Winner::None,
    }
}

fn nought(game_board: GameBoard) -> Winner {
    match nought_first_row(game_board) {
        Winner::Nought => Winner::Nought,
        _ => match nought_second_row(game_board) {
            Winner::Nought => Winner::Nought,
            _ => match nought_third_row(game_board) {
                Winner::Nought => Winner::Nought,
                _ => match nought_first_col(game_board) {
                    Winner::Nought => Winner::Nought,
                    _ => match nought_second_col(game_board) {
                        Winner::Nought => Winner::Nought,
                        _ => match nought_third_col(game_board) {
                            Winner::Nought => Winner::Nought,
                            _ => match nought_diag_one(game_board) {
                                Winner::Nought => Winner::Nought,
                                _ => match nought_diag_two(game_board) {
                                    Winner::Nought => Winner::Nought,
                                    _ => Winner::None,
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn nought_first_row(game_board: GameBoard) -> Winner{
        match game_board.row_one[0] {
        TileStatus::Nought(_cursor) => match game_board.row_one[1] {
            TileStatus::Nought(_cursor) => match game_board.row_one[2] {
                TileStatus::Nought(_cursor) => Winner::Nought,
                _ => Winner::None
            },
            _ => Winner::None,
        },
        _ => Winner::None,
    }
}

fn nought_second_row(game_board: GameBoard) -> Winner{
        match game_board.row_two[0] {
        TileStatus::Nought(_cursor) => match game_board.row_two[1] {
            TileStatus::Nought(_cursor) => match game_board.row_two[2] {
                TileStatus::Nought(_cursor) => Winner::Nought,
                _ => Winner::None
            },
            _ => Winner::None,
        },
        _ => Winner::None,
    }
}

fn nought_third_row(game_board: GameBoard) -> Winner{
        match game_board.row_three[0] {
        TileStatus::Nought(_cursor) => match game_board.row_three[1] {
            TileStatus::Nought(_cursor) => match game_board.row_three[2] {
                TileStatus::Nought(_cursor) => Winner::Nought,
                _ => Winner::None
            },
            _ => Winner::None,
        },
        _ => Winner::None,
    }
}

fn nought_first_col(game_board: GameBoard) -> Winner{
        match game_board.row_one[0] {
        TileStatus::Nought(_cursor) => match game_board.row_two[0] {
            TileStatus::Nought(_cursor) => match game_board.row_three[0] {
                TileStatus::Nought(_cursor) => Winner::Nought,
                _ => Winner::None
            },
            _ => Winner::None,
        },
        _ => Winner::None,
    }
}

fn nought_second_col(game_board: GameBoard) -> Winner{
        match game_board.row_one[1] {
        TileStatus::Nought(_cursor) => match game_board.row_two[1] {
            TileStatus::Nought(_cursor) => match game_board.row_three[1] {
                TileStatus::Nought(_cursor) => Winner::Nought,
                _ => Winner::None
            },
            _ => Winner::None,
        },
        _ => Winner::None,
    }
}

fn nought_third_col(game_board: GameBoard) -> Winner{
        match game_board.row_one[2] {
        TileStatus::Nought(_cursor) => match game_board.row_two[2] {
            TileStatus::Nought(_cursor) => match game_board.row_three[2] {
                TileStatus::Nought(_cursor) => Winner::Nought,
                _ => Winner::None
            },
            _ => Winner::None,
        },
        _ => Winner::None,
    }
}

fn nought_diag_one(game_board: GameBoard) -> Winner{
        match game_board.row_one[0] {
        TileStatus::Nought(_cursor) => match game_board.row_two[1] {
            TileStatus::Nought(_cursor) => match game_board.row_three[2] {
                TileStatus::Nought(_cursor) => Winner::Nought,
                _ => Winner::None
            },
            _ => Winner::None,
        },
        _ => Winner::None,
    }
}

fn nought_diag_two(game_board: GameBoard) -> Winner{
        match game_board.row_one[2] {
        TileStatus::Nought(_cursor) => match game_board.row_two[1] {
            TileStatus::Nought(_cursor) => match game_board.row_three[0] {
                TileStatus::Nought(_cursor) => Winner::Nought,
                _ => Winner::None
            },
            _ => Winner::None,
        },
        _ => Winner::None,
    }
}



// pub fn has_someone_won(
//     row1: &[&str; 3],
//     row2: &[&str; 3],
//     row3: &[&str; 3]
//     ) -> String {
    
//     let player = ["x", "0"] ;
//     let mut winner = "none".to_string();
//     for &player in player.iter() {
//         if (row1[0] == player) && (row1[1] == player) && (row1[2] == player){
//             mention_the_winner(player.to_string());
//             winner = player.to_string();
//         }
//         else if (row2[0] == player) && (row2[1] == player) && (row2[2] == player){
//             mention_the_winner(player.to_string());
//             winner = player.to_string();
//         }
//         else if (row3[0] == player) && (row3[1] == player) && (row3[2] == player){
//             mention_the_winner(player.to_string());
//             winner = player.to_string();
//         }



//         else if (row1[0] == player) && (row2[0] == player) && (row3[0] == player){
//             mention_the_winner(player.to_string());
//             winner = player.to_string();
//         }
//         else if (row1[1] == player) && (row2[1] == player) && (row3[1] == player){
//             mention_the_winner(player.to_string());
//             winner = player.to_string();
//         }
//         else if (row1[2] == player) && (row2[2] == player) && (row3[2] == player){
//             mention_the_winner(player.to_string());
//             winner = player.to_string();
//         }


//         else if (row1[0] == player) && (row2[1] == player) && (row3[2] == player){
//             mention_the_winner(player.to_string());
//             winner = player.to_string();
//         }
//         else if (row3[0] == player) && (row2[1] == player) && (row1[2] == player){
//             mention_the_winner(player.to_string());
//             winner = player.to_string();
//         }
//     }
//     fn mention_the_winner(winner: String) {
//         println!("The winner is {}", winner);
//     }
//     winner.to_string()
// }
