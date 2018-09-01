use movement::Movement;
use Cursor;
use GameBoard;
use TileStatus;

impl GameBoard {
    pub fn move_cursor(self, inputed_movement: Movement) -> Option<GameBoard> {
        if self.row_one[0].is_cursor() {
            match inputed_movement {
                Movement::Right => Option::Some(GameBoard {
                    row_one: [
                        self.row_one[0].remove_cursor(),
                        self.row_one[1].add_cursor(),
                        self.row_one[2],
                    ],
                    ..self
                }),
                Movement::Down => Option::Some(GameBoard {
                    row_one: [
                        self.row_one[0].remove_cursor(),
                        self.row_one[1],
                        self.row_one[2],
                    ],
                    row_two: [
                        self.row_two[0].add_cursor(),
                        self.row_two[1],
                        self.row_two[2],
                    ],
                    ..self
                }),
                _ => {
                    println!("This can not be done");
                    Option::None
                }
            }
        } else if self.row_one[1].is_cursor() {
            match inputed_movement {
                Movement::Right => Option::Some(GameBoard {
                    row_one: [
                        self.row_one[0],
                        self.row_one[1].remove_cursor(),
                        self.row_one[2].add_cursor(),
                    ],
                    ..self
                }),
                Movement::Left => Option::Some(GameBoard {
                    row_one: [
                        self.row_one[0].add_cursor(),
                        self.row_one[1].remove_cursor(),
                        self.row_one[2],
                    ],
                    ..self
                }),
                Movement::Down => Option::Some(GameBoard {
                    row_one: [
                        self.row_one[0],
                        self.row_one[1].remove_cursor(),
                        self.row_one[2],
                    ],
                    row_two: [
                        self.row_two[0],
                        self.row_two[1].add_cursor(),
                        self.row_two[2],
                    ],
                    ..self
                }),
                _ => {
                    println!("This can not be done");
                    Option::None
                }
            }
        } else if self.row_one[2].is_cursor() {
            match inputed_movement {
                Movement::Left => Option::Some(GameBoard {
                    row_one: [
                        self.row_one[0],
                        self.row_one[1].add_cursor(),
                        self.row_one[2].remove_cursor(),
                    ],
                    ..self
                }),
                Movement::Down => Option::Some(GameBoard {
                    row_one: [
                        self.row_one[0],
                        self.row_one[1],
                        self.row_one[2].remove_cursor(),
                    ],
                    row_two: [
                        self.row_two[0],
                        self.row_two[1],
                        self.row_two[2].add_cursor(),
                    ],
                    ..self
                }),
                _ => {
                    println!("This can not be done");
                    Option::None
                }
            }
        } else if self.row_two[0].is_cursor() {
            match inputed_movement {
                Movement::Right => Option::Some(GameBoard {
                    row_two: [
                        self.row_two[0].remove_cursor(),
                        self.row_two[1].add_cursor(),
                        self.row_two[2],
                    ],
                    ..self
                }),
                Movement::Down => Option::Some(GameBoard {
                    row_two: [
                        self.row_two[0].remove_cursor(),
                        self.row_two[1],
                        self.row_two[2],
                    ],
                    row_three: [
                        self.row_three[0].add_cursor(),
                        self.row_three[1],
                        self.row_three[2],
                    ],
                    ..self
                }),
                Movement::Up => Option::Some(GameBoard {
                    row_one: [
                        self.row_one[0].add_cursor(),
                        self.row_one[1],
                        self.row_one[2],
                    ],
                    row_two: [
                        self.row_two[0].remove_cursor(),
                        self.row_two[1],
                        self.row_two[2],
                    ],
                    ..self
                }),
                _ => {
                    println!("This can not be done");
                    Option::None
                }
            }
        } else if self.row_two[1].is_cursor() {
            match inputed_movement {
                Movement::Right => Option::Some(GameBoard {
                    row_two: [
                        self.row_two[0],
                        self.row_two[1].remove_cursor(),
                        self.row_two[2].add_cursor(),
                    ],
                    ..self
                }),
                Movement::Left => Option::Some(GameBoard {
                    row_two: [
                        self.row_two[0].add_cursor(),
                        self.row_two[1].remove_cursor(),
                        self.row_two[2],
                    ],
                    ..self
                }),
                Movement::Down => Option::Some(GameBoard {
                    row_two: [
                        self.row_two[0],
                        self.row_two[1].remove_cursor(),
                        self.row_two[2],
                    ],
                    row_three: [
                        self.row_three[0],
                        self.row_three[1].add_cursor(),
                        self.row_three[2],
                    ],
                    ..self
                }),
                Movement::Up => Option::Some(GameBoard {
                    row_one: [
                        self.row_one[0],
                        self.row_one[1].add_cursor(),
                        self.row_one[2],
                    ],
                    row_two: [
                        self.row_two[0],
                        self.row_two[1].remove_cursor(),
                        self.row_two[2],
                    ],
                    ..self
                }),
                _ => {
                    println!("This can not be done");
                    Option::None
                }
            }
        } else if self.row_two[2].is_cursor() {
            match inputed_movement {
                Movement::Left => Option::Some(GameBoard {
                    row_two: [
                        self.row_two[0],
                        self.row_two[1].add_cursor(),
                        self.row_two[2].remove_cursor(),
                    ],
                    ..self
                }),
                Movement::Down => Option::Some(GameBoard {
                    row_two: [
                        self.row_two[0],
                        self.row_two[1],
                        self.row_two[2].remove_cursor(),
                    ],
                    row_three: [
                        self.row_three[0],
                        self.row_three[1],
                        self.row_three[2].add_cursor(),
                    ],
                    ..self
                }),
                Movement::Up => Option::Some(GameBoard {
                    row_one: [
                        self.row_one[0],
                        self.row_one[1],
                        self.row_one[2].add_cursor(),
                    ],
                    row_two: [
                        self.row_two[0],
                        self.row_two[1],
                        self.row_two[2].remove_cursor(),
                    ],
                    ..self
                }),
                _ => {
                    println!("This can not be done");
                    Option::None
                }
            }
        } else if self.row_three[0].is_cursor() {
            match inputed_movement {
                Movement::Right => Option::Some(GameBoard {
                    row_three: [
                        self.row_three[0].remove_cursor(),
                        self.row_three[1].add_cursor(),
                        self.row_three[2],
                    ],
                    ..self
                }),
                Movement::Up => Option::Some(GameBoard {
                    row_three: [
                        self.row_three[0].remove_cursor(),
                        self.row_three[1],
                        self.row_three[2],
                    ],
                    row_two: [
                        self.row_two[0].add_cursor(),
                        self.row_two[1],
                        self.row_two[2],
                    ],
                    ..self
                }),
                _ => {
                    println!("This can not be done");
                    Option::None
                }
            }
        } else if self.row_three[1].is_cursor() {
            match inputed_movement {
                Movement::Right => Option::Some(GameBoard {
                    row_three: [
                        self.row_three[0],
                        self.row_three[1].remove_cursor(),
                        self.row_three[2].add_cursor(),
                    ],
                    ..self
                }),
                Movement::Left => Option::Some(GameBoard {
                    row_three: [
                        self.row_three[0].add_cursor(),
                        self.row_three[1].remove_cursor(),
                        self.row_three[2],
                    ],
                    ..self
                }),
                Movement::Up => Option::Some(GameBoard {
                    row_three: [
                        self.row_three[0],
                        self.row_three[1].remove_cursor(),
                        self.row_three[2],
                    ],
                    row_two: [
                        self.row_two[0],
                        self.row_two[1].add_cursor(),
                        self.row_two[2],
                    ],
                    ..self
                }),
                _ => {
                    println!("This can not be done");
                    Option::None
                }
            }
        } else if self.row_three[2].is_cursor() {
            match inputed_movement {
                Movement::Left => Option::Some(GameBoard {
                    row_three: [
                        self.row_three[0],
                        self.row_three[1].add_cursor(),
                        self.row_three[2].remove_cursor(),
                    ],
                    ..self
                }),
                Movement::Up => Option::Some(GameBoard {
                    row_three: [
                        self.row_three[0],
                        self.row_three[1],
                        self.row_three[2].remove_cursor(),
                    ],
                    row_two: [
                        self.row_two[0],
                        self.row_two[1],
                        self.row_two[2].add_cursor(),
                    ],
                    ..self
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
}

impl TileStatus {
    fn remove_cursor(self) -> TileStatus {
        match self {
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

    fn add_cursor(self) -> TileStatus {
        match self {
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
}
