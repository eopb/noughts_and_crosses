pub mod won;
use GameBoard;
use TileStatus;

impl GameBoard {
    pub fn is_board_full(self) -> bool {
        !(self.board[0][0].no_player())
            && !(self.board[0][1].no_player())
            && !(self.board[0][2].no_player())
            && !(self.board[1][0].no_player())
            && !(self.board[1][1].no_player())
            && !(self.board[1][2].no_player())
            && !(self.board[2][0].no_player())
            && !(self.board[2][1].no_player())
            && !(self.board[2][2].no_player())
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
