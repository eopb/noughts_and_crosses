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
    None,
}

pub fn process_movement(game_board: GameBoard, current_player: Players) -> Option<GameBoard> {
    let input = fetch_input();
    match input {
        Movement::Place => place_player(game_board, current_player),
        Movement::None => Option::None,
        _ => move_cursor(game_board, input),
    }
}

fn fetch_input() -> Movement {
    let mut movement = String::new();
    let mut umovement = 0;

    io::stdin()
        .read_line(&mut movement)
        .expect("Failed to read line");
    umovement = match movement.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Plese Try again");
            return Movement::None;
        }
    };

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
        Movement::None
    }
}

fn place_player(game_board: GameBoard, current_player: Players) -> Option<GameBoard> {
    if is_cursor(game_board.row_one[0]) {
        match game_board.row_one[0] {
            TileStatus::Cursor => match current_player {
                Players::Cross => Option::Some(GameBoard {
                    row_one: [
                        TileStatus::Cross(Cursor::True),
                        game_board.row_one[1],
                        game_board.row_one[2],
                    ],
                    ..game_board
                }),
                Players::Nought => Option::Some(GameBoard {
                    row_one: [
                        TileStatus::Nought(Cursor::True),
                        game_board.row_one[1],
                        game_board.row_one[2],
                    ],
                    ..game_board
                }),
            },
            _ => Option::None,
        }
    } else if is_cursor(game_board.row_one[1]) {
        match game_board.row_one[1] {
            TileStatus::Cursor => match current_player {
                Players::Cross => Option::Some(GameBoard {
                    row_one: [
                        game_board.row_one[0],
                        TileStatus::Cross(Cursor::True),
                        game_board.row_one[2],
                    ],
                    ..game_board
                }),
                Players::Nought => Option::Some(GameBoard {
                    row_one: [
                        game_board.row_one[0],
                        TileStatus::Nought(Cursor::True),
                        game_board.row_one[2],
                    ],
                    ..game_board
                }),
            },
            _ => Option::None,
        }
    } else if is_cursor(game_board.row_one[2]) {
        match game_board.row_one[2] {
            TileStatus::Cursor => match current_player {
                Players::Cross => Option::Some(GameBoard {
                    row_one: [
                        game_board.row_one[0],
                        game_board.row_one[1],
                        TileStatus::Cross(Cursor::True),
                    ],
                    ..game_board
                }),
                Players::Nought => Option::Some(GameBoard {
                    row_one: [
                        game_board.row_one[0],
                        game_board.row_one[1],
                        TileStatus::Nought(Cursor::True),
                    ],
                    ..game_board
                }),
            },
            _ => Option::None,
        }
    } else if is_cursor(game_board.row_two[0]) {
        match game_board.row_two[0] {
            TileStatus::Cursor => match current_player {
                Players::Cross => Option::Some(GameBoard {
                    row_two: [
                        TileStatus::Cross(Cursor::True),
                        game_board.row_two[1],
                        game_board.row_two[2],
                    ],
                    ..game_board
                }),
                Players::Nought => Option::Some(GameBoard {
                    row_two: [
                        TileStatus::Nought(Cursor::True),
                        game_board.row_two[1],
                        game_board.row_two[2],
                    ],
                    ..game_board
                }),
            },
            _ => Option::None,
        }
    } else if is_cursor(game_board.row_two[1]) {
        match game_board.row_two[1] {
            TileStatus::Cursor => match current_player {
                Players::Cross => Option::Some(GameBoard {
                    row_two: [
                        game_board.row_two[0],
                        TileStatus::Cross(Cursor::True),
                        game_board.row_two[2],
                    ],
                    ..game_board
                }),
                Players::Nought => Option::Some(GameBoard {
                    row_two: [
                        game_board.row_two[0],
                        TileStatus::Nought(Cursor::True),
                        game_board.row_two[2],
                    ],
                    ..game_board
                }),
            },
            _ => Option::None,
        }
    } else if is_cursor(game_board.row_two[2]) {
        match game_board.row_two[2] {
            TileStatus::Cursor => match current_player {
                Players::Cross => Option::Some(GameBoard {
                    row_two: [
                        game_board.row_two[0],
                        game_board.row_two[1],
                        TileStatus::Cross(Cursor::True),
                    ],
                    ..game_board
                }),
                Players::Nought => Option::Some(GameBoard {
                    row_two: [
                        game_board.row_two[0],
                        game_board.row_two[1],
                        TileStatus::Nought(Cursor::True),
                    ],
                    ..game_board
                }),
            },
            _ => Option::None,
        }
    } else if is_cursor(game_board.row_three[0]) {
        match game_board.row_three[0] {
            TileStatus::Cursor => match current_player {
                Players::Cross => Option::Some(GameBoard {
                    row_three: [
                        TileStatus::Cross(Cursor::True),
                        game_board.row_three[1],
                        game_board.row_three[2],
                    ],
                    ..game_board
                }),
                Players::Nought => Option::Some(GameBoard {
                    row_three: [
                        TileStatus::Nought(Cursor::True),
                        game_board.row_three[1],
                        game_board.row_three[2],
                    ],
                    ..game_board
                }),
            },
            _ => Option::None,
        }
    } else if is_cursor(game_board.row_three[1]) {
        match game_board.row_three[1] {
            TileStatus::Cursor => match current_player {
                Players::Cross => Option::Some(GameBoard {
                    row_three: [
                        game_board.row_three[0],
                        TileStatus::Cross(Cursor::True),
                        game_board.row_three[2],
                    ],
                    ..game_board
                }),
                Players::Nought => Option::Some(GameBoard {
                    row_three: [
                        game_board.row_three[0],
                        TileStatus::Nought(Cursor::True),
                        game_board.row_three[2],
                    ],
                    ..game_board
                }),
            },
            _ => Option::None,
        }
    } else if is_cursor(game_board.row_three[2]) {
        match game_board.row_three[2] {
            TileStatus::Cursor => match current_player {
                Players::Cross => Option::Some(GameBoard {
                    row_three: [
                        game_board.row_three[0],
                        game_board.row_three[1],
                        TileStatus::Cross(Cursor::True),
                    ],
                    ..game_board
                }),
                Players::Nought => Option::Some(GameBoard {
                    row_three: [
                        game_board.row_three[0],
                        game_board.row_three[1],
                        TileStatus::Nought(Cursor::True),
                    ],
                    ..game_board
                }),
            },
            _ => Option::None,
        }
    } else {
        Option::None
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
