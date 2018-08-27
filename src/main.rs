use ai::no_player;
use ai::process_ai;
use draw::draw_game_board;
use movement::process_movement;
use won::has_someone_won;
mod ai;
mod draw;
mod movement;
mod won;
use std::io;
const IS_DEBUG: bool = false;
#[derive(Copy, Clone, Debug)]
pub struct GameBoard {
    row_one: [TileStatus; 3],
    row_two: [TileStatus; 3],
    row_three: [TileStatus; 3],
}

#[derive(Copy, Clone, Debug)]
pub enum TileStatus {
    Nought(Cursor),
    Cross(Cursor),
    Cursor,
    None,
}

#[derive(Copy, Clone, Debug)]
pub enum Cursor {
    True,
    None,
}

#[derive(Copy, Clone)]
pub enum Players {
    Nought,
    Cross,
}

enum GameStatus {
    Playing,
    Finished,
}

pub enum Winner {
    Nought,
    Cross,
    None,
}

enum GameMode {
    TwoPlayer,
    SinglePlayer,
}
#[derive(Copy, Clone)]
pub enum AiMode {
    Random,
    SmartRandom,
    None,
}

pub struct MovementReturn {
    game_board: Option<GameBoard>,
    placed: bool,
}
fn main() {
    println!("Welcome to my noughts and crosses game made in rust.");
    print_instructions();
    let mut ai_mode = AiMode::None;
    let game_mode = game_mode_choice();
    match game_mode {
        GameMode::SinglePlayer => {
            ai_mode = ai_mode_choice();
        }
        _ => (),
    };
    let mut current_player = Players::Cross;
    let mut game_status = GameStatus::Playing;
    let mut game_board = GameBoard {
        row_one: [TileStatus::Cursor, TileStatus::None, TileStatus::None],
        row_two: [TileStatus::None, TileStatus::None, TileStatus::None],
        row_three: [TileStatus::None, TileStatus::None, TileStatus::None],
    };

    println!("Crosses goes first.");
    println!("The board looks like this.");
    draw_game_board(game_board);

    while match game_status {
        GameStatus::Playing => true,
        GameStatus::Finished => false,
    } {
        let mut movement_return = process_movement(game_board, current_player);
        loop {
            game_board = match movement_return.game_board {
                Some(game_board) => game_board,
                None => {
                    println!("That did not work");
                    movement_return = process_movement(game_board, current_player);
                    continue;
                }
            };
            break;
        }
        if movement_return.placed {
            match has_someone_won(game_board) {
                Winner::Cross => {
                    match game_mode {
                        GameMode::SinglePlayer => {
                            println!("You won");
                        }
                        GameMode::TwoPlayer => {
                            println!("Crosses won");
                        }
                    }
                    game_status = GameStatus::Finished;
                    continue;
                }
                Winner::Nought => {
                    println!("Noughts won");
                    game_status = GameStatus::Finished;
                    continue;
                }
                Winner::None => {
                    if IS_DEBUG {
                        println!("No one has won");
                    };
                    if is_board_full(game_board) {
                        println!("It is a tie!");
                        game_status = GameStatus::Finished;
                        continue;
                    };
                }
            };
            match game_mode {
                GameMode::TwoPlayer => {
                    current_player = switch_player(current_player);
                }
                GameMode::SinglePlayer => {
                    game_board = process_ai(game_board, ai_mode);
                    match has_someone_won(game_board) {
                        Winner::Cross => {
                            println!("You won");
                            game_status = GameStatus::Finished;
                            continue;
                        }
                        Winner::Nought => {
                            println!("Noughts won");
                            game_status = GameStatus::Finished;
                            continue;
                        }
                        Winner::None => {
                            if IS_DEBUG {
                                println!("No one has won");
                            };
                            if is_board_full(game_board) {
                                println!("It is a tie!");
                                game_status = GameStatus::Finished;
                                continue;
                            };
                        }
                    };
                }
            };
            draw_game_board(game_board);
            continue;
        };
        draw_game_board(game_board);
    }
}

fn switch_player(current_player: Players) -> Players {
    match current_player {
        Players::Cross => {
            if IS_DEBUG {
                println!("Current player was switched to Nought");
            };
            Players::Nought
        }
        Players::Nought => {
            if IS_DEBUG {
                println!("Current player was switched to Cross");
            };
            Players::Cross
        }
    }
}

fn game_mode_choice() -> GameMode {
    println!("Input the number of players you want to play.");
    loop {
        let mut inputed_choice = String::new();
        io::stdin()
            .read_line(&mut inputed_choice)
            .expect("Failed to read line");
        let inputed_choice: u32 = match inputed_choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("I did not understand that number. Plese Try again");
                continue;
            }
        };
        if inputed_choice == 1 {
            println!("Welcome to single player mode.");
            return GameMode::SinglePlayer;
        } else if inputed_choice == 2 {
            println!("Welcome to two player mode.");
            return GameMode::TwoPlayer;
        } else {
            println!("This game only works with 1 or 2 players. Please try again.");
            continue;
        }
    }
}

fn ai_mode_choice() -> AiMode {
    println!("Input the ai mode you want to play against. One or town?");
    loop {
        let mut inputed_choice = String::new();
        io::stdin()
            .read_line(&mut inputed_choice)
            .expect("Failed to read line");
        let inputed_choice: u32 = match inputed_choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("I did not understand that number. Plese Try again");
                continue;
            }
        };
        if inputed_choice == 1 {
            println!("Welcome to Random mode.");
            return AiMode::Random;
        } else if inputed_choice == 2 {
            println!("Welcome to SmartRandom mode.");
            return AiMode::SmartRandom;
        } else {
            println!("This game only works with ai 1 or 2");
            continue;
        }
    }
}

fn print_instructions() {
    println!("To move the star left type 4 and hit enter");
    println!("To move the star right type 6 and hit enter");
    println!("To move the star up type 8 and hit enter");
    println!("To move the star down type 2 and hit enter");
    println!("To place your cross type 5 and hit enter");
}

fn is_board_full(game_board: GameBoard) -> bool {
    if !(no_player(game_board.row_one[0]))
        && !(no_player(game_board.row_one[1]))
        && !(no_player(game_board.row_one[2]))
        && !(no_player(game_board.row_two[0]))
        && !(no_player(game_board.row_two[1]))
        && !(no_player(game_board.row_two[2]))
        && !(no_player(game_board.row_three[0]))
        && !(no_player(game_board.row_three[1]))
        && !(no_player(game_board.row_three[2]))
    {
        true
    } else {
        false
    }
}
