use GameBoard;
use TileStatus;
use Cursor;
use movement::Movement;

#[allow(needless_return)]
pub fn move_cursor(game_board: GameBoard, inputed_movement: Movement) -> GameBoard {
    if is_cursor(game_board.row_one[0]){
        return match inputed_movement {
            Movement::Right => GameBoard {
                row_one: [
                    remove_cursor(game_board.row_one[0]),
                    add_cursor(game_board.row_one[1]),
                    game_board.row_one[2]
                ],
                ..game_board
            },
            Movement::Down => GameBoard {
                row_one: [
                    remove_cursor(game_board.row_one[0]),
                    game_board.row_one[1],
                    game_board.row_one[2]
                ],
                row_two: [
                    add_cursor(game_board.row_two[0]),
                    game_board.row_two[1],
                    game_board.row_two[2]
                ],
                ..game_board
            },
            _ => {
                println!("This can not be done");
                game_board
            },
        };
    }
    else if is_cursor(game_board.row_one[1]){
        return match inputed_movement {
            Movement::Right => GameBoard {
                row_one: [
                    game_board.row_one[0],
                    remove_cursor(game_board.row_one[1]),
                    add_cursor(game_board.row_one[2])
                ],
                ..game_board
            },
            Movement::Left => GameBoard {
                row_one: [
                    add_cursor(game_board.row_one[0]),
                    remove_cursor(game_board.row_one[1]),
                    game_board.row_one[2]
                ],
                ..game_board
            },
            Movement::Down => GameBoard {
                row_one: [
                    game_board.row_one[0],
                    remove_cursor(game_board.row_one[1]),
                    game_board.row_one[2]
                ],
                row_two: [
                    game_board.row_two[0],
                    add_cursor(game_board.row_two[1]),
                    game_board.row_two[2]
                ],
                ..game_board
            },
            _ => {
                println!("This can not be done");
                game_board
            },
        };
    }
    else if is_cursor(game_board.row_one[2]){
        return match inputed_movement {
            Movement::Left => GameBoard {
                row_one: [
                    game_board.row_one[0],
                    add_cursor(game_board.row_one[1]),
                    remove_cursor(game_board.row_one[2])
                ],
                ..game_board
            },
            Movement::Down => GameBoard {
                row_one: [
                    game_board.row_one[0],
                    game_board.row_one[1],
                    remove_cursor(game_board.row_one[2])
                ],
                row_two: [
                    game_board.row_two[0],
                    game_board.row_two[1],
                    add_cursor(game_board.row_two[2])
                ],
                ..game_board
            },
            _ => {
                println!("This can not be done");
                game_board
            },
        };
    }
    else if is_cursor(game_board.row_two[0]){
        return match inputed_movement {
            Movement::Right => GameBoard {
                row_two: [
                    remove_cursor(game_board.row_two[0]),
                    add_cursor(game_board.row_two[1]),
                    game_board.row_two[2]
                ],
                ..game_board
            },
            Movement::Down => GameBoard {
                row_two: [
                    remove_cursor(game_board.row_two[0]),
                    game_board.row_two[1],
                    game_board.row_two[2]
                ],
                row_three: [
                    add_cursor(game_board.row_three[0]),
                    game_board.row_three[1],
                    game_board.row_three[2]
                ],
                ..game_board
            },
            Movement::Up => GameBoard {
                row_one: [
                    add_cursor(game_board.row_one[0]),
                    game_board.row_one[1],
                    game_board.row_one[2]
                ],
                row_two: [
                    remove_cursor(game_board.row_two[0]),
                    game_board.row_two[1],
                    game_board.row_two[2]
                ],
                ..game_board
            },
            _ => {
                println!("This can not be done");
                game_board
            },
        };
    }
    else if is_cursor(game_board.row_two[1]){
        return match inputed_movement {
            Movement::Right => GameBoard {
                row_two: [
                    game_board.row_two[0],
                    remove_cursor(game_board.row_two[1]),
                    add_cursor(game_board.row_two[2])
                ],
                ..game_board
            },
            Movement::Left => GameBoard {
                row_two: [
                    add_cursor(game_board.row_two[0]),
                    remove_cursor(game_board.row_two[1]),
                    game_board.row_two[2]
                ],
                ..game_board
            },
            Movement::Down => GameBoard {
                row_two: [
                    game_board.row_two[0],
                    remove_cursor(game_board.row_two[1]),
                    game_board.row_two[2]
                ],
                row_three: [
                    game_board.row_three[0],
                    add_cursor(game_board.row_three[1]),
                    game_board.row_three[2]
                ],
                ..game_board
            },
            Movement::Up => GameBoard {
                row_one: [
                    game_board.row_one[0],
                    add_cursor(game_board.row_one[1]),
                    game_board.row_one[2]
                ],
                row_two: [
                    game_board.row_two[0],
                    remove_cursor(game_board.row_two[1]),
                    game_board.row_two[2]
                ],
                ..game_board
            },
            _ => {
                println!("This can not be done");
                game_board
            },
        };
    }
    else if is_cursor(game_board.row_two[2]){
        return match inputed_movement {
            Movement::Left => GameBoard {
                row_two: [
                    game_board.row_two[0],
                    add_cursor(game_board.row_two[1]),
                    remove_cursor(game_board.row_two[2])
                ],
                ..game_board
            },
            Movement::Down => GameBoard {
                row_two: [
                    game_board.row_two[0],
                    game_board.row_two[1],
                    remove_cursor(game_board.row_two[2])
                ],
                row_three: [
                    game_board.row_three[0],
                    game_board.row_three[1],
                    add_cursor(game_board.row_three[2])
                ],
                ..game_board
            },
            Movement::Up => GameBoard {
                row_one: [
                    game_board.row_one[0],
                    game_board.row_one[1],
                    add_cursor(game_board.row_one[2])
                ],
                row_two: [
                    game_board.row_two[0],
                    game_board.row_two[1],
                    remove_cursor(game_board.row_two[2])
                ],
                ..game_board
            },
            _ => {
                println!("This can not be done");
                game_board
            },
        };
    }
    else if is_cursor(game_board.row_three[0]){
        return match inputed_movement {
            Movement::Right => GameBoard {
                row_three: [
                    remove_cursor(game_board.row_three[0]),
                    add_cursor(game_board.row_three[1]),
                    game_board.row_three[2]
                ],
                ..game_board
            },
            Movement::Up => GameBoard {
                row_three: [
                    remove_cursor(game_board.row_three[0]),
                    game_board.row_three[1],
                    game_board.row_three[2]
                ],
                row_two: [
                    add_cursor(game_board.row_two[0]),
                    game_board.row_two[1],
                    game_board.row_two[2]
                ],
                ..game_board
            },
            _ => {
                println!("This can not be done");
                game_board
            },
        };
    }
    else {
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
