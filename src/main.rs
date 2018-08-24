use std::io;
mod draw;
// mod tests;

pub struct GameBoard {
    row_one: [TileStatus; 3],
    row_two: [TileStatus; 3],
    row_three: [TileStatus; 3],
}

pub enum TileStatus {
    Nought(Cursor),
    Cross(Cursor),
    Cursor,
    None,
}

enum Players {
    Nought,
    Cross,
}

pub enum Cursor {
    True,
    None,
}

fn main() {
    println!("Welcome to my noughts and crosses game made in rust.");

    let mut game_board = GameBoard {
        row_one: [
            TileStatus::Cursor,
            TileStatus::Nought(Cursor::True),
            TileStatus::None
            ],
        row_two: [
            TileStatus::None,
            TileStatus::Nought(Cursor::None),
            TileStatus::None
            ],
        row_three: [
            TileStatus::None,
            TileStatus::Cross(Cursor::None),
            TileStatus::None
            ],
    };
    let mut current_player = Players::Cross;

    println!("Crosses goes first.");
    println!("The board looks like this.");
    draw::draw_game_board(&game_board);
    println!("You are the *");
}



// fn main() {
//     println!("Welcome to my noughts and crosses game made in rust.");
//     let mut row1 = [" "," "," "];
//     let mut row2 = [" "," "," "];
//     let mut row3 = [" "," "," "];

//     let mut player_position: [i32; 2] = [1,1];

//     let mut current_player = "0";

//     println!("Crosses goes first.");

//     println!("The board looks like this.");
//     draw::draw_game_board(&row1, &row2, &row3, &player_position);
//     println!("You are the *");
//     loop {
//         if current_player == "0" {
//             current_player = "x";
//         }
//         else if current_player == "x" {
//             current_player = "0";
//         }
//         draw::draw_game_board(&row1, &row2, &row3, &player_position);
//         let winner = tests::has_someone_won(&row1, &row2, &row3);
//         println!("winner {}", winner);
//         loop {
//             println!("The player is {}", current_player);
//             println!("To move the star left type 4 and hit enter");
//             println!("To move the star right type 6 and hit enter");
//             println!("To move the star up type 8 and hit enter");
//             println!("To move the star down type 2 and hit enter");
//             println!("To place your cross type 5 and hit enter");
//             let mut movement = String::new();

//             io::stdin().read_line(&mut movement)
//                 .expect("Failed to read line");
//             let movement: u32 = match movement.trim().parse() {
//                 Ok(num) => num,
//                 Err(_) => continue,
//             };
//             if (movement == 4) ^ (movement == 2) ^ (movement == 6) ^ (movement == 8) {
//                 player_position = move_player(movement, player_position);
//             }
//             else if movement == 5 {
//                 if player_position[0] == 1 {
//                     if player_position[1] == 1 {
//                         if row1[0] == " " {
//                             row1 = [current_player, row1[1], row1[2]];
//                             break;
//                         }
//                     }
//                     if player_position[1] == 2 {
//                         if row1[1] == " " {
//                             row1 = [row1[0], current_player, row1[2]];
//                             break;
//                         }
//                     }
//                     if player_position[1] == 3 {
//                         if row1[2] == " " {
//                             row1 = [row1[0], row1[1], current_player];
//                             break;
//                         }
//                     }
//                 }
//                 if player_position[0] == 2 {
//                     if player_position[1] == 1 {
//                         if row2[0] == " " {
//                             row2 = [current_player, row2[1], row2[2]];
//                             break;
//                         }
//                     }
//                     if player_position[1] == 2 {
//                         if row2[1] == " " {
//                             row2 = [row2[0], current_player, row2[2]];
//                             break;
//                         }
//                     }
//                     if player_position[1] == 3 {
//                         if row2[2] == " " {
//                             row2 = [row2[0], row2[1], current_player];
//                             break;
//                         }
//                     }
//                 }
//                 if player_position[0] == 3 {
//                     if player_position[1] == 1 {
//                         if row3[0] == " " {
//                             row3 = [current_player, row3[1], row3[2]];
//                             break;
//                         }
//                     }
//                     if player_position[1] == 2 {
//                         if row3[1] == " " {
//                             row3 = [row3[0], current_player, row3[2]];
//                             break;
//                         }
//                     }
//                     if player_position[1] == 3 {
//                         if row3[2] == " " {
//                             row3 = [row3[0], row3[1], current_player];
//                             break;
//                         }
//                     }
//                 }

                
//             }
//             else if movement == 0 {
//                 panic!();
//             }
//             else {
//                 println!("That move is invalid please try again.");
//                 continue; 
//             }
//             draw::draw_game_board(&row1, &row2, &row3, &player_position);
//             let winner = tests::has_someone_won(&row1, &row2, &row3);
//             println!("winner {}", winner)
//         }
//     }

//     // draw::draw_game_board(&row1, &row2, &row3, &player_position);
//     // let winner = tests::has_someone_won(&row1, &row2, &row3);
//     // println!("winner {}", winner)
// }

// fn move_player(movement: u32, mut player_position: [i32; 2]) -> [i32; 2]{
//     if movement == 4 {
//         if player_position[1] == 1 {
//             println!("That move is invalid please try again.");
//         }
//         else {
//             player_position = [player_position[0] , player_position[1] - 1];
//         }
//     }
//     else if movement == 6 {
//         if player_position[1] == 3 {
//             println!("That move is invalid please try again.");
//         }
//         else {
//             player_position = [player_position[0] , player_position[1] + 1];
//         }
//     }
//     else if movement == 8 {
//         if player_position[0] == 1 {
//             println!("That move is invalid please try again.");
//         }
//         else {
//             player_position = [player_position[0] - 1 , player_position[1]];
//         }
//     }
//     else if movement == 2 {
//         if player_position[0] == 3 {
//             println!("That move is invalid please try again.");
//         }
//         else {
//             player_position = [player_position[0] + 1, player_position[1]];
//         }
//     }
//     player_position
// }
