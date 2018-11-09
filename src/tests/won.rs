use GameBoard;
use TileStatus;
use Winner;

impl GameBoard {
    pub fn has_someone_won(self) -> Winner {
        match cross(self) {
            Winner::Nought | Winner::Cross => Winner::Cross,
            Winner::None => nought(self),
        }
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
    match game_board.board[0][0] {
        TileStatus::Nought(_cursor) => match game_board.board[0][1] {
            TileStatus::Nought(_cursor) => match game_board.board[0][2] {
                TileStatus::Nought(_cursor) => Winner::Nought,
                _ => Winner::None,
            },
            _ => Winner::None,
        },
        _ => Winner::None,
    }
}

fn nought_second_row(game_board: GameBoard) -> Winner {
    match game_board.board[1][0] {
        TileStatus::Nought(_cursor) => match game_board.board[1][1] {
            TileStatus::Nought(_cursor) => match game_board.board[1][2] {
                TileStatus::Nought(_cursor) => Winner::Nought,
                _ => Winner::None,
            },
            _ => Winner::None,
        },
        _ => Winner::None,
    }
}

fn nought_third_row(game_board: GameBoard) -> Winner {
    match game_board.board[2][0] {
        TileStatus::Nought(_cursor) => match game_board.board[2][1] {
            TileStatus::Nought(_cursor) => match game_board.board[2][2] {
                TileStatus::Nought(_cursor) => Winner::Nought,
                _ => Winner::None,
            },
            _ => Winner::None,
        },
        _ => Winner::None,
    }
}

fn nought_first_col(game_board: GameBoard) -> Winner {
    match game_board.board[0][0] {
        TileStatus::Nought(_cursor) => match game_board.board[1][0] {
            TileStatus::Nought(_cursor) => match game_board.board[2][0] {
                TileStatus::Nought(_cursor) => Winner::Nought,
                _ => Winner::None,
            },
            _ => Winner::None,
        },
        _ => Winner::None,
    }
}

fn nought_second_col(game_board: GameBoard) -> Winner {
    match game_board.board[0][1] {
        TileStatus::Nought(_cursor) => match game_board.board[1][1] {
            TileStatus::Nought(_cursor) => match game_board.board[2][1] {
                TileStatus::Nought(_cursor) => Winner::Nought,
                _ => Winner::None,
            },
            _ => Winner::None,
        },
        _ => Winner::None,
    }
}

fn nought_third_col(game_board: GameBoard) -> Winner {
    match game_board.board[0][2] {
        TileStatus::Nought(_cursor) => match game_board.board[1][2] {
            TileStatus::Nought(_cursor) => match game_board.board[2][2] {
                TileStatus::Nought(_cursor) => Winner::Nought,
                _ => Winner::None,
            },
            _ => Winner::None,
        },
        _ => Winner::None,
    }
}

fn nought_diag_one(game_board: GameBoard) -> Winner {
    match game_board.board[0][0] {
        TileStatus::Nought(_cursor) => match game_board.board[1][1] {
            TileStatus::Nought(_cursor) => match game_board.board[2][2] {
                TileStatus::Nought(_cursor) => Winner::Nought,
                _ => Winner::None,
            },
            _ => Winner::None,
        },
        _ => Winner::None,
    }
}

fn nought_diag_two(game_board: GameBoard) -> Winner {
    match game_board.board[0][2] {
        TileStatus::Nought(_cursor) => match game_board.board[1][1] {
            TileStatus::Nought(_cursor) => match game_board.board[2][0] {
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
    match game_board.board[0][0] {
        TileStatus::Cross(_cursor) => match game_board.board[0][1] {
            TileStatus::Cross(_cursor) => match game_board.board[0][2] {
                TileStatus::Cross(_cursor) => Winner::Cross,
                _ => Winner::None,
            },
            _ => Winner::None,
        },
        _ => Winner::None,
    }
}

fn cross_second_row(game_board: GameBoard) -> Winner {
    match game_board.board[1][0] {
        TileStatus::Cross(_cursor) => match game_board.board[1][1] {
            TileStatus::Cross(_cursor) => match game_board.board[1][2] {
                TileStatus::Cross(_cursor) => Winner::Cross,
                _ => Winner::None,
            },
            _ => Winner::None,
        },
        _ => Winner::None,
    }
}

fn cross_third_row(game_board: GameBoard) -> Winner {
    match game_board.board[2][0] {
        TileStatus::Cross(_cursor) => match game_board.board[2][1] {
            TileStatus::Cross(_cursor) => match game_board.board[2][2] {
                TileStatus::Cross(_cursor) => Winner::Cross,
                _ => Winner::None,
            },
            _ => Winner::None,
        },
        _ => Winner::None,
    }
}

fn cross_first_col(game_board: GameBoard) -> Winner {
    match game_board.board[0][0] {
        TileStatus::Cross(_cursor) => match game_board.board[1][0] {
            TileStatus::Cross(_cursor) => match game_board.board[2][0] {
                TileStatus::Cross(_cursor) => Winner::Cross,
                _ => Winner::None,
            },
            _ => Winner::None,
        },
        _ => Winner::None,
    }
}

fn cross_second_col(game_board: GameBoard) -> Winner {
    match game_board.board[0][1] {
        TileStatus::Cross(_cursor) => match game_board.board[1][1] {
            TileStatus::Cross(_cursor) => match game_board.board[2][1] {
                TileStatus::Cross(_cursor) => Winner::Cross,
                _ => Winner::None,
            },
            _ => Winner::None,
        },
        _ => Winner::None,
    }
}

fn cross_third_col(game_board: GameBoard) -> Winner {
    match game_board.board[0][2] {
        TileStatus::Cross(_cursor) => match game_board.board[1][2] {
            TileStatus::Cross(_cursor) => match game_board.board[2][2] {
                TileStatus::Cross(_cursor) => Winner::Cross,
                _ => Winner::None,
            },
            _ => Winner::None,
        },
        _ => Winner::None,
    }
}

fn cross_diag_one(game_board: GameBoard) -> Winner {
    match game_board.board[0][0] {
        TileStatus::Cross(_cursor) => match game_board.board[1][1] {
            TileStatus::Cross(_cursor) => match game_board.board[2][2] {
                TileStatus::Cross(_cursor) => Winner::Cross,
                _ => Winner::None,
            },
            _ => Winner::None,
        },
        _ => Winner::None,
    }
}

fn cross_diag_two(game_board: GameBoard) -> Winner {
    match game_board.board[0][2] {
        TileStatus::Cross(_cursor) => match game_board.board[1][1] {
            TileStatus::Cross(_cursor) => match game_board.board[2][0] {
                TileStatus::Cross(_cursor) => Winner::Cross,
                _ => Winner::None,
            },
            _ => Winner::None,
        },
        _ => Winner::None,
    }
}
