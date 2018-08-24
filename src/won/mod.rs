use GameBoard;
use Players;
use TileStatus;
use Winner;

pub fn has_someone_won(current_player: Players, game_board: GameBoard) -> Winner {
    match current_player {
        Players::Nought => nought(game_board),
        Players::Cross => cross(game_board),
    }
}

fn nought(game_board: GameBoard) -> Winner {
    match nought_first_row(game_board) {
        Winner::Nought => Winner::Nought,
        _ => match nought_second_row(game_board) {
            Winner::Nought => Winner::Nought,
            _ => match nought_third_row(game_board) {
                Winner::Nought => Winner::Nought,
                _ => match nought_first_col(game_board) {
                    Winner::Nought => Winner::Nought,
                    _ => match nought_second_col(game_board) {
                        Winner::Nought => Winner::Nought,
                        _ => match nought_third_col(game_board) {
                            Winner::Nought => Winner::Nought,
                            _ => match nought_diag_one(game_board) {
                                Winner::Nought => Winner::Nought,
                                _ => match nought_diag_two(game_board) {
                                    Winner::Nought => Winner::Nought,
                                    _ => Winner::None,
                                },
                            },
                        },
                    },
                },
            },
        },
    }
}

fn nought_first_row(game_board: GameBoard) -> Winner {
    match game_board.row_one[0] {
        TileStatus::Nought(_cursor) => match game_board.row_one[1] {
            TileStatus::Nought(_cursor) => match game_board.row_one[2] {
                TileStatus::Nought(_cursor) => Winner::Nought,
                _ => Winner::None,
            },
            _ => Winner::None,
        },
        _ => Winner::None,
    }
}

fn nought_second_row(game_board: GameBoard) -> Winner {
    match game_board.row_two[0] {
        TileStatus::Nought(_cursor) => match game_board.row_two[1] {
            TileStatus::Nought(_cursor) => match game_board.row_two[2] {
                TileStatus::Nought(_cursor) => Winner::Nought,
                _ => Winner::None,
            },
            _ => Winner::None,
        },
        _ => Winner::None,
    }
}

fn nought_third_row(game_board: GameBoard) -> Winner {
    match game_board.row_three[0] {
        TileStatus::Nought(_cursor) => match game_board.row_three[1] {
            TileStatus::Nought(_cursor) => match game_board.row_three[2] {
                TileStatus::Nought(_cursor) => Winner::Nought,
                _ => Winner::None,
            },
            _ => Winner::None,
        },
        _ => Winner::None,
    }
}

fn nought_first_col(game_board: GameBoard) -> Winner {
    match game_board.row_one[0] {
        TileStatus::Nought(_cursor) => match game_board.row_two[0] {
            TileStatus::Nought(_cursor) => match game_board.row_three[0] {
                TileStatus::Nought(_cursor) => Winner::Nought,
                _ => Winner::None,
            },
            _ => Winner::None,
        },
        _ => Winner::None,
    }
}

fn nought_second_col(game_board: GameBoard) -> Winner {
    match game_board.row_one[1] {
        TileStatus::Nought(_cursor) => match game_board.row_two[1] {
            TileStatus::Nought(_cursor) => match game_board.row_three[1] {
                TileStatus::Nought(_cursor) => Winner::Nought,
                _ => Winner::None,
            },
            _ => Winner::None,
        },
        _ => Winner::None,
    }
}

fn nought_third_col(game_board: GameBoard) -> Winner {
    match game_board.row_one[2] {
        TileStatus::Nought(_cursor) => match game_board.row_two[2] {
            TileStatus::Nought(_cursor) => match game_board.row_three[2] {
                TileStatus::Nought(_cursor) => Winner::Nought,
                _ => Winner::None,
            },
            _ => Winner::None,
        },
        _ => Winner::None,
    }
}

fn nought_diag_one(game_board: GameBoard) -> Winner {
    match game_board.row_one[0] {
        TileStatus::Nought(_cursor) => match game_board.row_two[1] {
            TileStatus::Nought(_cursor) => match game_board.row_three[2] {
                TileStatus::Nought(_cursor) => Winner::Nought,
                _ => Winner::None,
            },
            _ => Winner::None,
        },
        _ => Winner::None,
    }
}

fn nought_diag_two(game_board: GameBoard) -> Winner {
    match game_board.row_one[2] {
        TileStatus::Nought(_cursor) => match game_board.row_two[1] {
            TileStatus::Nought(_cursor) => match game_board.row_three[0] {
                TileStatus::Nought(_cursor) => Winner::Nought,
                _ => Winner::None,
            },
            _ => Winner::None,
        },
        _ => Winner::None,
    }
}

fn cross(game_board: GameBoard) -> Winner {
    match cross_first_row(game_board) {
        Winner::Cross => Winner::Cross,
        _ => match cross_second_row(game_board) {
            Winner::Cross => Winner::Cross,
            _ => match cross_third_row(game_board) {
                Winner::Cross => Winner::Cross,
                _ => match cross_first_col(game_board) {
                    Winner::Cross => Winner::Cross,
                    _ => match cross_second_col(game_board) {
                        Winner::Cross => Winner::Cross,
                        _ => match cross_third_col(game_board) {
                            Winner::Cross => Winner::Cross,
                            _ => match cross_diag_one(game_board) {
                                Winner::Cross => Winner::Cross,
                                _ => match cross_diag_two(game_board) {
                                    Winner::Cross => Winner::Cross,
                                    _ => Winner::None,
                                },
                            },
                        },
                    },
                },
            },
        },
    }
}

fn cross_first_row(game_board: GameBoard) -> Winner {
    match game_board.row_one[0] {
        TileStatus::Cross(_cursor) => match game_board.row_one[1] {
            TileStatus::Cross(_cursor) => match game_board.row_one[2] {
                TileStatus::Cross(_cursor) => Winner::Cross,
                _ => Winner::None,
            },
            _ => Winner::None,
        },
        _ => Winner::None,
    }
}

fn cross_second_row(game_board: GameBoard) -> Winner {
    match game_board.row_two[0] {
        TileStatus::Cross(_cursor) => match game_board.row_two[1] {
            TileStatus::Cross(_cursor) => match game_board.row_two[2] {
                TileStatus::Cross(_cursor) => Winner::Cross,
                _ => Winner::None,
            },
            _ => Winner::None,
        },
        _ => Winner::None,
    }
}

fn cross_third_row(game_board: GameBoard) -> Winner {
    match game_board.row_three[0] {
        TileStatus::Cross(_cursor) => match game_board.row_three[1] {
            TileStatus::Cross(_cursor) => match game_board.row_three[2] {
                TileStatus::Cross(_cursor) => Winner::Cross,
                _ => Winner::None,
            },
            _ => Winner::None,
        },
        _ => Winner::None,
    }
}

fn cross_first_col(game_board: GameBoard) -> Winner {
    match game_board.row_one[0] {
        TileStatus::Cross(_cursor) => match game_board.row_two[0] {
            TileStatus::Cross(_cursor) => match game_board.row_three[0] {
                TileStatus::Cross(_cursor) => Winner::Cross,
                _ => Winner::None,
            },
            _ => Winner::None,
        },
        _ => Winner::None,
    }
}

fn cross_second_col(game_board: GameBoard) -> Winner {
    match game_board.row_one[1] {
        TileStatus::Cross(_cursor) => match game_board.row_two[1] {
            TileStatus::Cross(_cursor) => match game_board.row_three[1] {
                TileStatus::Cross(_cursor) => Winner::Cross,
                _ => Winner::None,
            },
            _ => Winner::None,
        },
        _ => Winner::None,
    }
}

fn cross_third_col(game_board: GameBoard) -> Winner {
    match game_board.row_one[2] {
        TileStatus::Cross(_cursor) => match game_board.row_two[2] {
            TileStatus::Cross(_cursor) => match game_board.row_three[2] {
                TileStatus::Cross(_cursor) => Winner::Cross,
                _ => Winner::None,
            },
            _ => Winner::None,
        },
        _ => Winner::None,
    }
}

fn cross_diag_one(game_board: GameBoard) -> Winner {
    match game_board.row_one[0] {
        TileStatus::Cross(_cursor) => match game_board.row_two[1] {
            TileStatus::Cross(_cursor) => match game_board.row_three[2] {
                TileStatus::Cross(_cursor) => Winner::Cross,
                _ => Winner::None,
            },
            _ => Winner::None,
        },
        _ => Winner::None,
    }
}

fn cross_diag_two(game_board: GameBoard) -> Winner {
    match game_board.row_one[2] {
        TileStatus::Cross(_cursor) => match game_board.row_two[1] {
            TileStatus::Cross(_cursor) => match game_board.row_three[0] {
                TileStatus::Cross(_cursor) => Winner::Cross,
                _ => Winner::None,
            },
            _ => Winner::None,
        },
        _ => Winner::None,
    }
}
