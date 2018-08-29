use Cursor;
use Players;
use TileStatus;

impl TileStatus {
    pub fn place_player(self, player_to_place: Players) -> TileStatus {
        match self {
            TileStatus::Nought(_cursor) | TileStatus::Cross(_cursor) => {
                println!("Error can't place player. Going to painc");
                panic!();
            }
            TileStatus::Cursor => match player_to_place {
                Players::Cross => TileStatus::Cross(Cursor::True),
                Players::Nought => TileStatus::Nought(Cursor::True),
            },
            TileStatus::None => match player_to_place {
                Players::Cross => TileStatus::Cross(Cursor::None),
                Players::Nought => TileStatus::Nought(Cursor::None),
            },
        }
    }
}
