mod move_cursor;
use std::io;
use Cursor;
use GameBoard;
use MovementReturn;
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

pub fn process(game_board: GameBoard, current_player: Players) -> MovementReturn {
    let input = fetch_input();
    match input {
        Movement::Place => MovementReturn {
            game_board: place_player_on_cursor(game_board, current_player),
            placed: true,
        },
        Movement::None => MovementReturn {
            game_board: Option::None,
            placed: false,
        },
        _ => MovementReturn {
            game_board: game_board.move_cursor(input),
            placed: false,
        },
    }
}

fn fetch_input() -> Movement {
    let mut movement = String::new();
    #[allow(unused_assignments)]
    let mut movement_num = 0;

    io::stdin()
        .read_line(&mut movement)
        .expect("Failed to read line");
    movement_num = match movement.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Plese Try again");
            return Movement::None;
        }
    };

    if movement_num == 4 {
        println!("You have pressed 4 to go Left");
        Movement::Left
    } else if movement_num == 6 {
        println!("You have pressed 6 to go Right");
        Movement::Right
    } else if movement_num == 8 {
        println!("You have pressed 8 to go UP");
        Movement::Up
    } else if movement_num == 2 {
        println!("You have pressed 2 to go Down");
        Movement::Down
    } else if movement_num == 5 {
        println!("You have pressed 5 to place you peace");
        Movement::Place
    } else {
        Movement::None
    }
}

fn place_player_on_cursor(game_board: GameBoard, current_player: Players) -> Option<GameBoard> {
    if game_board.board[0][0].is_cursor() {
        match game_board.board[0][0] {
            TileStatus::Cursor => Option::Some(GameBoard {
                board: [
                    [
                        game_board.board[0][0].place_player(current_player),
                        game_board.board[0][1],
                        game_board.board[0][2],
                    ],
                    game_board.board[1],
                    game_board.board[2],
                ],
            }),
            _ => Option::None,
        }
    } else if game_board.board[0][1].is_cursor() {
        match game_board.board[0][1] {
            TileStatus::Cursor => Option::Some(GameBoard {
                board: [
                    [
                        game_board.board[0][0],
                        game_board.board[0][1].place_player(current_player),
                        game_board.board[0][2],
                    ],
                    game_board.board[1],
                    game_board.board[2],
                ],
            }),
            _ => Option::None,
        }
    } else if game_board.board[0][2].is_cursor() {
        match game_board.board[0][2] {
            TileStatus::Cursor => Option::Some(GameBoard {
                board: [
                    [
                        game_board.board[0][0],
                        game_board.board[0][1],
                        game_board.board[0][2].place_player(current_player),
                    ],
                    game_board.board[1],
                    game_board.board[2],
                ],
            }),
            _ => Option::None,
        }
    } else if game_board.board[1][0].is_cursor() {
        match game_board.board[1][0] {
            TileStatus::Cursor => Option::Some(GameBoard {
                board: [
                    game_board.board[0],
                    [
                        game_board.board[1][0].place_player(current_player),
                        game_board.board[1][1],
                        game_board.board[1][2],
                    ],
                    game_board.board[2],
                ],
            }),
            _ => Option::None,
        }
    } else if game_board.board[1][1].is_cursor() {
        match game_board.board[1][1] {
            TileStatus::Cursor => Option::Some(GameBoard {
                board: [
                    game_board.board[0],
                    [
                        game_board.board[1][0],
                        game_board.board[1][1].place_player(current_player),
                        game_board.board[1][2],
                    ],
                    game_board.board[2],
                ],
            }),
            _ => Option::None,
        }
    } else if game_board.board[1][2].is_cursor() {
        match game_board.board[1][2] {
            TileStatus::Cursor => Option::Some(GameBoard {
                board: [
                    game_board.board[0],
                    [
                        game_board.board[1][0],
                        game_board.board[1][1],
                        game_board.board[1][2].place_player(current_player),
                    ],
                    game_board.board[2],
                ],
            }),
            _ => Option::None,
        }
    } else if game_board.board[2][0].is_cursor() {
        match game_board.board[2][0] {
            TileStatus::Cursor => Option::Some(GameBoard {
                board: [
                    game_board.board[0],
                    game_board.board[1],
                    [
                        game_board.board[2][0].place_player(current_player),
                        game_board.board[2][1],
                        game_board.board[2][2],
                    ],
                ],
            }),
            _ => Option::None,
        }
    } else if game_board.board[2][1].is_cursor() {
        match game_board.board[2][1] {
            TileStatus::Cursor => Option::Some(GameBoard {
                board: [
                    game_board.board[0],
                    game_board.board[1],
                    [
                        game_board.board[2][0],
                        game_board.board[2][1].place_player(current_player),
                        game_board.board[2][2],
                    ],
                ],
            }),
            _ => Option::None,
        }
    } else if game_board.board[2][2].is_cursor() {
        match game_board.board[2][2] {
            TileStatus::Cursor => Option::Some(GameBoard {
                board: [
                    game_board.board[0],
                    game_board.board[1],
                    [
                        game_board.board[2][0],
                        game_board.board[2][1],
                        game_board.board[2][2].place_player(current_player),
                    ],
                ],
            }),
            _ => Option::None,
        }
    } else {
        Option::None
    }
}

impl TileStatus {
    fn is_cursor(self) -> bool {
        match self {
            TileStatus::Cursor => true,
            TileStatus::Nought(cursor) | TileStatus::Cross(cursor) => match cursor {
                Cursor::True => true,
                Cursor::None => false,
            },
            TileStatus::None => false,
        }
    }
}
