pub mod won;
use GameBoard;
use TileStatus;

impl GameBoard {
    pub fn is_board_full(self) -> bool {
        !(self.row_one[0].no_player())
            && !(self.row_one[1].no_player())
            && !(self.row_one[2].no_player())
            && !(self.row_two[0].no_player())
            && !(self.row_two[1].no_player())
            && !(self.row_two[2].no_player())
            && !(self.row_three[0].no_player())
            && !(self.row_three[1].no_player())
            && !(self.row_three[2].no_player())
    }
}

impl TileStatus {
    pub fn no_player(self) -> bool {
        match self {
            TileStatus::Cursor | TileStatus::None => true,
            TileStatus::Nought(_cursor) | TileStatus::Cross(_cursor) => false,
        }
    }
}
