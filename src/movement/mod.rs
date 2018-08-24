mod move_cursor;
use self::move_cursor::move_cursor;
use std::io;
use Cursor;
use GameBoard;
use Players;
use TileStatus;

#[derive(Copy, Clone)]
pub enum Movement {
    Left,
    Right,
    Up,
    Down,
    Place,
}

pub fn process_movement(game_board: GameBoard, current_player: Players) -> GameBoard {
    let input = fetch_input();
    match input {
        Movement::Place => place_player(game_board, current_player),
        _ => move_cursor(game_board, input),
    }
}

fn fetch_input() -> Movement {
    let mut movement = String::new();
    let mut umovement = 0;
    loop {
        io::stdin()
            .read_line(&mut movement)
            .expect("Failed to read line");
        umovement = match movement.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Plese Try again");
                continue;
            }
        };
        if (umovement == 4)
            ^ (umovement == 6)
            ^ (umovement == 8)
            ^ (umovement == 2)
            ^ (umovement == 5)
        {
            break;
        }
    }
    if umovement == 4 {
        println!("You have pressed 4 to go Left");
        Movement::Left
    } else if umovement == 6 {
        println!("You have pressed 6 to go Right");
        Movement::Right
    } else if umovement == 8 {
        println!("You have pressed 8 to go UP");
        Movement::Up
    } else if umovement == 2 {
        println!("You have pressed 2 to go Down");
        Movement::Down
    } else if umovement == 5 {
        println!("You have pressed 5 to place you peace");
        Movement::Place
    } else {
        panic!();
    }
}

fn place_player(game_board: GameBoard, current_player: Players) -> GameBoard {
    if is_cursor(game_board.row_one[0]) {
        match game_board.row_one[0] {
            TileStatus::Cursor => match current_player {
                Players::Cross | Players::Nought => GameBoard {
                    row_one: [
                        TileStatus::Cross(Cursor::True),
                        game_board.row_one[1],
                        game_board.row_one[2],
                    ],
                    ..game_board
                },
            },
            _ => game_board,
        }
    } else if is_cursor(game_board.row_one[1]) {
        match game_board.row_one[1] {
            TileStatus::Cursor => match current_player {
                Players::Cross | Players::Nought => GameBoard {
                    row_one: [
                        game_board.row_one[0],
                        TileStatus::Cross(Cursor::True),
                        game_board.row_one[2],
                    ],
                    ..game_board
                },
            },
            _ => game_board,
        }
    } else if is_cursor(game_board.row_one[2]) {
        match game_board.row_one[2] {
            TileStatus::Cursor => match current_player {
                Players::Cross | Players::Nought => GameBoard {
                    row_one: [
                        game_board.row_one[0],
                        game_board.row_one[1],
                        TileStatus::Cross(Cursor::True),
                    ],
                    ..game_board
                },
            },
            _ => game_board,
        }
    } else if is_cursor(game_board.row_one[0]) {
        match game_board.row_two[0] {
            TileStatus::Cursor => match current_player {
                Players::Cross | Players::Nought => GameBoard {
                    row_two: [
                        TileStatus::Cross(Cursor::True),
                        game_board.row_two[1],
                        game_board.row_two[2],
                    ],
                    ..game_board
                },
            },
            _ => game_board,
        }
    } else if is_cursor(game_board.row_one[1]) {
        match game_board.row_two[1] {
            TileStatus::Cursor => match current_player {
                Players::Cross | Players::Nought => GameBoard {
                    row_two: [
                        game_board.row_two[0],
                        TileStatus::Cross(Cursor::True),
                        game_board.row_two[2],
                    ],
                    ..game_board
                },
            },
            _ => game_board,
        }
    } else if is_cursor(game_board.row_one[2]) {
        match game_board.row_two[2] {
            TileStatus::Cursor => match current_player {
                Players::Cross | Players::Nought => GameBoard {
                    row_two: [
                        game_board.row_two[0],
                        game_board.row_two[1],
                        TileStatus::Cross(Cursor::True),
                    ],
                    ..game_board
                },
            },
            _ => game_board,
        }
    } else if is_cursor(game_board.row_one[0]) {
        match game_board.row_three[0] {
            TileStatus::Cursor => match current_player {
                Players::Cross | Players::Nought => GameBoard {
                    row_three: [
                        TileStatus::Cross(Cursor::True),
                        game_board.row_three[1],
                        game_board.row_three[2],
                    ],
                    ..game_board
                },
            },
            _ => game_board,
        }
    } else if is_cursor(game_board.row_one[1]) {
        match game_board.row_three[1] {
            TileStatus::Cursor => match current_player {
                Players::Cross | Players::Nought => GameBoard {
                    row_three: [
                        game_board.row_three[0],
                        TileStatus::Cross(Cursor::True),
                        game_board.row_three[2],
                    ],
                    ..game_board
                },
            },
            _ => game_board,
        }
    } else if is_cursor(game_board.row_one[2]) {
        match game_board.row_three[2] {
            TileStatus::Cursor => match current_player {
                Players::Cross | Players::Nought => GameBoard {
                    row_three: [
                        game_board.row_three[0],
                        game_board.row_three[1],
                        TileStatus::Cross(Cursor::True),
                    ],
                    ..game_board
                },
            },
            _ => game_board,
        }
    } else {
        game_board
    }
}

pub fn is_cursor(tile: TileStatus) -> bool {
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
        TileStatus::None => false,
    }
}
