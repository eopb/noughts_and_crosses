use movement::is_cursor;
use movement::Movement;
use Cursor;
use GameBoard;
use TileStatus;

#[allow(cyclomatic_complexity)]
pub fn move_cursor(game_board: GameBoard, inputed_movement: Movement) -> Option<GameBoard> {
    if is_cursor(game_board.row_one[0]) {
        match inputed_movement {
            Movement::Right => Option::Some(GameBoard {
                row_one: [
                    remove_cursor(game_board.row_one[0]),
                    add_cursor(game_board.row_one[1]),
                    game_board.row_one[2],
                ],
                ..game_board
            }),
            Movement::Down => Option::Some(GameBoard {
                row_one: [
                    remove_cursor(game_board.row_one[0]),
                    game_board.row_one[1],
                    game_board.row_one[2],
                ],
                row_two: [
                    add_cursor(game_board.row_two[0]),
                    game_board.row_two[1],
                    game_board.row_two[2],
                ],
                ..game_board
            }),
            _ => {
                println!("This can not be done");
                Option::None
            }
        }
    } else if is_cursor(game_board.row_one[1]) {
        match inputed_movement {
            Movement::Right => Option::Some(GameBoard {
                row_one: [
                    game_board.row_one[0],
                    remove_cursor(game_board.row_one[1]),
                    add_cursor(game_board.row_one[2]),
                ],
                ..game_board
            }),
            Movement::Left => Option::Some(GameBoard {
                row_one: [
                    add_cursor(game_board.row_one[0]),
                    remove_cursor(game_board.row_one[1]),
                    game_board.row_one[2],
                ],
                ..game_board
            }),
            Movement::Down => Option::Some(GameBoard {
                row_one: [
                    game_board.row_one[0],
                    remove_cursor(game_board.row_one[1]),
                    game_board.row_one[2],
                ],
                row_two: [
                    game_board.row_two[0],
                    add_cursor(game_board.row_two[1]),
                    game_board.row_two[2],
                ],
                ..game_board
            }),
            _ => {
                println!("This can not be done");
                Option::None
            }
        }
    } else if is_cursor(game_board.row_one[2]) {
        match inputed_movement {
            Movement::Left => Option::Some(GameBoard {
                row_one: [
                    game_board.row_one[0],
                    add_cursor(game_board.row_one[1]),
                    remove_cursor(game_board.row_one[2]),
                ],
                ..game_board
            }),
            Movement::Down => Option::Some(GameBoard {
                row_one: [
                    game_board.row_one[0],
                    game_board.row_one[1],
                    remove_cursor(game_board.row_one[2]),
                ],
                row_two: [
                    game_board.row_two[0],
                    game_board.row_two[1],
                    add_cursor(game_board.row_two[2]),
                ],
                ..game_board
            }),
            _ => {
                println!("This can not be done");
                Option::None
            }
        }
    } else if is_cursor(game_board.row_two[0]) {
        match inputed_movement {
            Movement::Right => Option::Some(GameBoard {
                row_two: [
                    remove_cursor(game_board.row_two[0]),
                    add_cursor(game_board.row_two[1]),
                    game_board.row_two[2],
                ],
                ..game_board
            }),
            Movement::Down => Option::Some(GameBoard {
                row_two: [
                    remove_cursor(game_board.row_two[0]),
                    game_board.row_two[1],
                    game_board.row_two[2],
                ],
                row_three: [
                    add_cursor(game_board.row_three[0]),
                    game_board.row_three[1],
                    game_board.row_three[2],
                ],
                ..game_board
            }),
            Movement::Up => Option::Some(GameBoard {
                row_one: [
                    add_cursor(game_board.row_one[0]),
                    game_board.row_one[1],
                    game_board.row_one[2],
                ],
                row_two: [
                    remove_cursor(game_board.row_two[0]),
                    game_board.row_two[1],
                    game_board.row_two[2],
                ],
                ..game_board
            }),
            _ => {
                println!("This can not be done");
                Option::None
            }
        }
    } else if is_cursor(game_board.row_two[1]) {
        match inputed_movement {
            Movement::Right => Option::Some(GameBoard {
                row_two: [
                    game_board.row_two[0],
                    remove_cursor(game_board.row_two[1]),
                    add_cursor(game_board.row_two[2]),
                ],
                ..game_board
            }),
            Movement::Left => Option::Some(GameBoard {
                row_two: [
                    add_cursor(game_board.row_two[0]),
                    remove_cursor(game_board.row_two[1]),
                    game_board.row_two[2],
                ],
                ..game_board
            }),
            Movement::Down => Option::Some(GameBoard {
                row_two: [
                    game_board.row_two[0],
                    remove_cursor(game_board.row_two[1]),
                    game_board.row_two[2],
                ],
                row_three: [
                    game_board.row_three[0],
                    add_cursor(game_board.row_three[1]),
                    game_board.row_three[2],
                ],
                ..game_board
            }),
            Movement::Up => Option::Some(GameBoard {
                row_one: [
                    game_board.row_one[0],
                    add_cursor(game_board.row_one[1]),
                    game_board.row_one[2],
                ],
                row_two: [
                    game_board.row_two[0],
                    remove_cursor(game_board.row_two[1]),
                    game_board.row_two[2],
                ],
                ..game_board
            }),
            _ => {
                println!("This can not be done");
                Option::None
            }
        }
    } else if is_cursor(game_board.row_two[2]) {
        match inputed_movement {
            Movement::Left => Option::Some(GameBoard {
                row_two: [
                    game_board.row_two[0],
                    add_cursor(game_board.row_two[1]),
                    remove_cursor(game_board.row_two[2]),
                ],
                ..game_board
            }),
            Movement::Down => Option::Some(GameBoard {
                row_two: [
                    game_board.row_two[0],
                    game_board.row_two[1],
                    remove_cursor(game_board.row_two[2]),
                ],
                row_three: [
                    game_board.row_three[0],
                    game_board.row_three[1],
                    add_cursor(game_board.row_three[2]),
                ],
                ..game_board
            }),
            Movement::Up => Option::Some(GameBoard {
                row_one: [
                    game_board.row_one[0],
                    game_board.row_one[1],
                    add_cursor(game_board.row_one[2]),
                ],
                row_two: [
                    game_board.row_two[0],
                    game_board.row_two[1],
                    remove_cursor(game_board.row_two[2]),
                ],
                ..game_board
            }),
            _ => {
                println!("This can not be done");
                Option::None
            }
        }
    } else if is_cursor(game_board.row_three[0]) {
        match inputed_movement {
            Movement::Right => Option::Some(GameBoard {
                row_three: [
                    remove_cursor(game_board.row_three[0]),
                    add_cursor(game_board.row_three[1]),
                    game_board.row_three[2],
                ],
                ..game_board
            }),
            Movement::Up => Option::Some(GameBoard {
                row_three: [
                    remove_cursor(game_board.row_three[0]),
                    game_board.row_three[1],
                    game_board.row_three[2],
                ],
                row_two: [
                    add_cursor(game_board.row_two[0]),
                    game_board.row_two[1],
                    game_board.row_two[2],
                ],
                ..game_board
            }),
            _ => {
                println!("This can not be done");
                Option::None
            }
        }
    } else if is_cursor(game_board.row_three[1]) {
        match inputed_movement {
            Movement::Right => Option::Some(GameBoard {
                row_three: [
                    game_board.row_three[0],
                    remove_cursor(game_board.row_three[1]),
                    add_cursor(game_board.row_three[2]),
                ],
                ..game_board
            }),
            Movement::Left => Option::Some(GameBoard {
                row_three: [
                    add_cursor(game_board.row_three[0]),
                    remove_cursor(game_board.row_three[1]),
                    game_board.row_three[2],
                ],
                ..game_board
            }),
            Movement::Up => Option::Some(GameBoard {
                row_three: [
                    game_board.row_three[0],
                    remove_cursor(game_board.row_three[1]),
                    game_board.row_three[2],
                ],
                row_two: [
                    game_board.row_two[0],
                    add_cursor(game_board.row_two[1]),
                    game_board.row_two[2],
                ],
                ..game_board
            }),
            _ => {
                println!("This can not be done");
                Option::None
            }
        }
    } else if is_cursor(game_board.row_three[2]) {
        match inputed_movement {
            Movement::Left => Option::Some(GameBoard {
                row_three: [
                    game_board.row_three[0],
                    add_cursor(game_board.row_three[1]),
                    remove_cursor(game_board.row_three[2]),
                ],
                ..game_board
            }),
            Movement::Up => Option::Some(GameBoard {
                row_three: [
                    game_board.row_three[0],
                    game_board.row_three[1],
                    remove_cursor(game_board.row_three[2]),
                ],
                row_two: [
                    game_board.row_two[0],
                    game_board.row_two[1],
                    add_cursor(game_board.row_two[2]),
                ],
                ..game_board
            }),
            _ => {
                println!("This can not be done");
                Option::None
            }
        }
    } else {
        println!("here");
        panic!();
    }
}

fn remove_cursor(tile: TileStatus) -> TileStatus {
    match tile {
        TileStatus::Cross(cursor) => match cursor {
            Cursor::True => TileStatus::Cross(Cursor::None),
            Cursor::None => TileStatus::Cross(Cursor::None),
        },
        TileStatus::Nought(cursor) => match cursor {
            Cursor::True => TileStatus::Nought(Cursor::None),
            Cursor::None => TileStatus::Nought(Cursor::None),
        },
        TileStatus::Cursor => TileStatus::None,
        TileStatus::None => TileStatus::None,
    }
}

fn add_cursor(tile: TileStatus) -> TileStatus {
    match tile {
        TileStatus::Cross(cursor) => match cursor {
            Cursor::True => TileStatus::Cross(Cursor::True),
            Cursor::None => TileStatus::Cross(Cursor::True),
        },
        TileStatus::Nought(cursor) => match cursor {
            Cursor::True => TileStatus::Nought(Cursor::True),
            Cursor::None => TileStatus::Nought(Cursor::True),
        },
        TileStatus::Cursor => TileStatus::Cursor,
        TileStatus::None => TileStatus::Cursor,
    }
}
