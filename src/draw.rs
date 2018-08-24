// Prints out the game board
use GameBoard;
use TileStatus;
use Players;
use Cursor;

pub fn draw_game_board(game_board: &GameBoard) {
    println!("it works");
    println!("{}|{}|{}", x_or_0(&game_board.row_one[0]), x_or_0(&game_board.row_one[1]), x_or_0(&game_board.row_one[2]));
    println!("_ _ _");
    println!("{}|{}|{}", x_or_0(&game_board.row_two[0]), x_or_0(&game_board.row_two[1]), x_or_0(&game_board.row_two[2]));
    println!("_ _ _");
    println!("{}|{}|{}", x_or_0(&game_board.row_three[0]), x_or_0(&game_board.row_three[1]), x_or_0(&game_board.row_three[2]));
}

fn x_or_0(player: &TileStatus) -> String{
    match player {
        TileStatus::Player(Players) => {
            match Players {
                Players::Nought(cursor) => match cursor {
                    Cursor::True => ("0*").to_string(),
                    Cursor::None => ("0").to_string(),
                    },
                Players::Cross(cursor) => match cursor {
                    Cursor::True => ("X*").to_string(),
                    Cursor::None => ("X").to_string(),
                    },
            }
        },
        TileStatus::Cursor => ("*").to_string(),
        TileStatus::None => (" ").to_string(),
    }
}

// pub fn draw_game_board(
//     row1: &[&str; 3],
//     row2: &[&str; 3],
//     row3: &[&str; 3],
//     player_position: &[i32; 2]
//     ) {
//     let mut player11 = String::new();
//     let mut player12 = String::new();
//     let mut player13 = String::new();
//     let mut player21 = String::new();
//     let mut player22 = String::new();
//     let mut player23 = String::new();
//     let mut player31 = String::new();
//     let mut player32 = String::new();
//     let mut player33 = String::new();
//     if player_position[0] == 1 {
//         if player_position[1] == 1 {
//             player11 = "*".to_string();
//         }
//         else if player_position[1] == 2 {
//             player12 = "*".to_string();
//         }
//         else if player_position[1] == 3 {
//             player13 = "*".to_string();
//         }
//     }
//     else if player_position[0] == 2 {
//         if player_position[1] == 1 {
//             player21 = "*".to_string();
//         }
//         else if player_position[1] == 2 {
//             player22 = "*".to_string();
//         }
//         else if player_position[1] == 3 {
//             player23 = "*".to_string();
//         }
//     }
//     else if player_position[0] == 3 {
//         if player_position[1] == 1 {
//             player31 = "*".to_string();
//         }
//         else if player_position[1] == 2 {
//             player32 = "*".to_string();
//         }
//         else if player_position[1] == 3 {
//             player33 = "*".to_string();
//         }
//     }
//     println!("{}{}|{}{}|{}{}", row1[0], player11, row1[1], player12, row1[2], player13);
//     println!("_ _ _");
//     println!("{}{}|{}{}|{}{}", row2[0], player21, row2[1], player22, row2[2], player23);
//     println!("_ _ _");
//     println!("{}{}|{}{}|{}{}", row3[0], player31, row3[1], player32, row3[2], player33);
// }
