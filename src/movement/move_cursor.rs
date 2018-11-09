use movement::Movement;
use Cursor;
use GameBoard;
use TileStatus;

impl GameBoard {
    pub fn move_cursor(self, inputed_movement: Movement) -> Option<Self> {
        if self.board[0][0].is_cursor() {
            match inputed_movement {
                Movement::Right => Option::Some(Self {
                    board: [
                        [
                            self.board[0][0].remove_cursor(),
                            self.board[0][1].add_cursor(),
                            self.board[0][2],
                        ],
                        self.board[1],
                        self.board[2],
                    ],
                }),
                Movement::Down => Option::Some(Self {
                    board: [
                        [
                            self.board[0][0].remove_cursor(),
                            self.board[0][1],
                            self.board[0][2],
                        ],
                        [
                            self.board[1][0].add_cursor(),
                            self.board[1][1],
                            self.board[1][2],
                        ],
                        self.board[2],
                    ],
                }),
                _ => {
                    println!("This can not be done");
                    Option::None
                }
            }
        } else if self.board[0][1].is_cursor() {
            match inputed_movement {
                Movement::Right => Option::Some(Self {
                    board: [
                        [
                            self.board[0][0],
                            self.board[0][1].remove_cursor(),
                            self.board[0][2].add_cursor(),
                        ],
                        self.board[1],
                        self.board[2],
                    ],
                }),
                Movement::Left => Option::Some(Self {
                    board: [
                        [
                            self.board[0][0].add_cursor(),
                            self.board[0][1].remove_cursor(),
                            self.board[0][2],
                        ],
                        self.board[1],
                        self.board[2],
                    ],
                }),
                Movement::Down => Option::Some(Self {
                    board: [
                        [
                            self.board[0][0],
                            self.board[0][1].remove_cursor(),
                            self.board[0][2],
                        ],
                        [
                            self.board[1][0],
                            self.board[1][1].add_cursor(),
                            self.board[1][2],
                        ],
                        self.board[2],
                    ],
                }),
                _ => {
                    println!("This can not be done");
                    Option::None
                }
            }
        } else if self.board[0][2].is_cursor() {
            match inputed_movement {
                Movement::Left => Option::Some(Self {
                    board: [
                        [
                            self.board[0][0],
                            self.board[0][1].add_cursor(),
                            self.board[0][2].remove_cursor(),
                        ],
                        self.board[1],
                        self.board[2],
                    ],
                }),
                Movement::Down => Option::Some(Self {
                    board: [
                        [
                            self.board[0][0],
                            self.board[0][1],
                            self.board[0][2].remove_cursor(),
                        ],
                        [
                            self.board[1][0],
                            self.board[1][1],
                            self.board[1][2].add_cursor(),
                        ],
                        self.board[2],
                    ],
                }),
                _ => {
                    println!("This can not be done");
                    Option::None
                }
            }
        } else if self.board[1][0].is_cursor() {
            match inputed_movement {
                Movement::Right => Option::Some(Self {
                    board: [
                        self.board[0],
                        [
                            self.board[1][0].remove_cursor(),
                            self.board[1][1].add_cursor(),
                            self.board[1][2],
                        ],
                        self.board[2],
                    ],
                }),
                Movement::Down => Option::Some(Self {
                    board: [
                        self.board[0],
                        [
                            self.board[1][0].remove_cursor(),
                            self.board[1][1],
                            self.board[1][2],
                        ],
                        [
                            self.board[2][0].add_cursor(),
                            self.board[2][1],
                            self.board[2][2],
                        ],
                    ],
                }),
                Movement::Up => Option::Some(Self {
                    board: [
                        [
                            self.board[1][0].add_cursor(),
                            self.board[1][1],
                            self.board[1][2],
                        ],
                        [
                            self.board[2][0].remove_cursor(),
                            self.board[2][1],
                            self.board[2][2],
                        ],
                        self.board[2],
                    ],
                }),
                _ => {
                    println!("This can not be done");
                    Option::None
                }
            }
        } else if self.board[1][1].is_cursor() {
            match inputed_movement {
                Movement::Right => Option::Some(Self {
                    board: [
                        self.board[0],
                        [
                            self.board[1][0],
                            self.board[1][1].remove_cursor(),
                            self.board[1][2].add_cursor(),
                        ],
                        self.board[2],
                    ],
                }),
                Movement::Left => Option::Some(Self {
                    board: [
                        self.board[0],
                        [
                            self.board[1][0].add_cursor(),
                            self.board[1][1].remove_cursor(),
                            self.board[1][2],
                        ],
                        self.board[2],
                    ],
                }),
                Movement::Down => Option::Some(Self {
                    board: [
                        self.board[0],
                        [
                            self.board[1][0],
                            self.board[1][1].remove_cursor(),
                            self.board[1][2],
                        ],
                        [
                            self.board[2][0],
                            self.board[2][1].add_cursor(),
                            self.board[2][2],
                        ],
                    ],
                }),
                Movement::Up => Option::Some(Self {
                    board: [
                        [
                            self.board[0][0],
                            self.board[0][1].add_cursor(),
                            self.board[0][2],
                        ],
                        [
                            self.board[1][0],
                            self.board[1][1].remove_cursor(),
                            self.board[1][2],
                        ],
                        self.board[2],
                    ],
                }),
                _ => {
                    println!("This can not be done");
                    Option::None
                }
            }
        } else if self.board[1][2].is_cursor() {
            match inputed_movement {
                Movement::Left => Option::Some(Self {
                    board: [
                        self.board[0],
                        [
                            self.board[1][0],
                            self.board[1][1].add_cursor(),
                            self.board[1][2].remove_cursor(),
                        ],
                        self.board[2],
                    ],
                }),
                Movement::Down => Option::Some(Self {
                    board: [
                        self.board[0],
                        [
                            self.board[1][0],
                            self.board[1][1],
                            self.board[1][2].remove_cursor(),
                        ],
                        [
                            self.board[2][0],
                            self.board[2][1],
                            self.board[2][2].add_cursor(),
                        ],
                    ],
                }),
                Movement::Up => Option::Some(Self {
                    board: [
                        [
                            self.board[0][0],
                            self.board[0][1],
                            self.board[0][2].add_cursor(),
                        ],
                        [
                            self.board[1][0],
                            self.board[1][1],
                            self.board[1][2].remove_cursor(),
                        ],
                        self.board[2],
                    ],
                }),
                _ => {
                    println!("This can not be done");
                    Option::None
                }
            }
        } else if self.board[2][0].is_cursor() {
            match inputed_movement {
                Movement::Right => Option::Some(Self {
                    board: [
                        self.board[0],
                        self.board[1],
                        [
                            self.board[2][0].remove_cursor(),
                            self.board[2][1].add_cursor(),
                            self.board[2][2],
                        ],
                    ],
                }),
                Movement::Up => Option::Some(Self {
                    board: [
                        self.board[0],
                        [
                            self.board[1][0].add_cursor(),
                            self.board[1][1],
                            self.board[1][2],
                        ],
                        [
                            self.board[2][0].remove_cursor(),
                            self.board[2][1],
                            self.board[2][2],
                        ],
                    ],
                }),
                _ => {
                    println!("This can not be done");
                    Option::None
                }
            }
        } else if self.board[2][1].is_cursor() {
            match inputed_movement {
                Movement::Right => Option::Some(Self {
                    board: [
                        self.board[0],
                        self.board[1],
                        [
                            self.board[2][0],
                            self.board[2][1].remove_cursor(),
                            self.board[2][2].add_cursor(),
                        ],
                    ],
                }),
                Movement::Left => Option::Some(Self {
                    board: [
                        self.board[0],
                        self.board[1],
                        [
                            self.board[2][0].add_cursor(),
                            self.board[2][1].remove_cursor(),
                            self.board[2][2],
                        ],
                    ],
                }),
                Movement::Up => Option::Some(Self {
                    board: [
                        self.board[0],
                        [
                            self.board[1][0],
                            self.board[1][1].add_cursor(),
                            self.board[1][2],
                        ],
                        [
                            self.board[2][0],
                            self.board[2][1].remove_cursor(),
                            self.board[2][2],
                        ],
                    ],
                    ..self
                }),
                _ => {
                    println!("This can not be done");
                    Option::None
                }
            }
        } else if self.board[2][2].is_cursor() {
            match inputed_movement {
                Movement::Left => Option::Some(Self {
                    board: [
                        self.board[0],
                        self.board[1],
                        [
                            self.board[2][0],
                            self.board[2][1].add_cursor(),
                            self.board[2][2].remove_cursor(),
                        ],
                    ],
                }),
                Movement::Up => Option::Some(Self {
                    board: [
                        self.board[0],
                        [
                            self.board[1][0],
                            self.board[1][1],
                            self.board[1][2].add_cursor(),
                        ],
                        [
                            self.board[2][0],
                            self.board[2][1],
                            self.board[2][2].remove_cursor(),
                        ],
                    ],
                }),
                _ => {
                    println!("This can not be done");
                    Option::None
                }
            }
        } else {
            panic!("No cursor found!");
        }
    }
}

impl TileStatus {
    fn remove_cursor(self) -> Self {
        match self {
            TileStatus::Cross(cursor) => match cursor {
                Cursor::True | Cursor::None => TileStatus::Cross(Cursor::None),
            },
            TileStatus::Nought(cursor) => match cursor {
                Cursor::True | Cursor::None => TileStatus::Nought(Cursor::None),
            },
            TileStatus::Cursor | TileStatus::None => TileStatus::None,
        }
    }

    fn add_cursor(self) -> Self {
        match self {
            TileStatus::Cross(cursor) => match cursor {
                Cursor::True | Cursor::None => TileStatus::Cross(Cursor::True),
            },
            TileStatus::Nought(cursor) => match cursor {
                Cursor::True | Cursor::None => TileStatus::Nought(Cursor::True),
            },
            TileStatus::Cursor | TileStatus::None => TileStatus::Cursor,
        }
    }
}
