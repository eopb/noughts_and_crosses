// Prints out the game board
use std::io;
use GameBoard;
use TileStatus;
use Cursor;

#[derive(Copy, Clone)]
enum Movement {
    Left,
    Right,
    Up,
    Down,
    Place,
}

pub fn process_movement(game_board: GameBoard) -> GameBoard {
    let input = fetch_input();
    match input {
        Movement::Place => place_player(game_board),
        _ => move_cursor(game_board, input),
    }
    
}

fn fetch_input() -> Movement{
    let mut movement = String::new();
    let mut umovement = 0;
    loop {
        io::stdin().read_line(&mut movement)
            .expect("Failed to read line");
        umovement = match movement.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Plese Try again");
                continue;},
        };
        if (umovement == 4) ^
        (umovement == 6) ^
        (umovement == 8) ^
        (umovement == 2) ^
        (umovement == 5) {break;}
    };
    if umovement == 4 {
        println!("You have pressed 4 to go Left");
        Movement::Left
    }
    else if umovement == 6 {
        println!("You have pressed 6 to go Right");
        Movement::Right
    }
    else if umovement == 8 {
        println!("You have pressed 8 to go UP");
        Movement::Up
    }
    else if umovement == 2 {
        println!("You have pressed 2 to go Down");
        Movement::Down
    }
    else if umovement == 5 {
        println!("You have pressed 5 to place you peace");
        Movement::Place
    }
    else {
        panic!();
    }
}

fn place_player(game_board: GameBoard) -> GameBoard {
    game_board
} 

fn move_cursor(game_board: GameBoard, inputed_movement: Movement) -> GameBoard {
    if is_cursor(game_board.row_one[0]){
        println!("meh");
    }
    game_board
} 

fn is_cursor(tile: TileStatus) -> bool {
    match tile {
        TileStatus::Cursor => true,
        TileStatus::Nought(cursor) => match cursor {
            Cursor::True => true,
            Cursor::None => false,
            },
        TileStatus::Cross(cursor) => match cursor {
            Cursor::True => true,
            Cursor::None => false,
            },
        TileStatus::None => false
    }
}

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
