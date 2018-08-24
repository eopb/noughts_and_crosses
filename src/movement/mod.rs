mod move_cursor;
use self::move_cursor::move_cursor;
use std::io;
use GameBoard;

#[derive(Copy, Clone)]
pub enum Movement {
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

