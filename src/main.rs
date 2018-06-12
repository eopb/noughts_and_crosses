fn main() {
    println!("Welcome to my noughts and crosses game made in rust.");

    enum PartStatus {
        PlayerX,
        PlayerY,
        Selected,
        Empty,
    }

    struct Row1 {
        Coloum1: PartStatus,
        Coloum2: PartStatus,
        Coloum3: PartStatus,
    }

    struct Row2 {
        Coloum1: PartStatus,
        Coloum2: PartStatus,
        Coloum3: PartStatus,
    
    }

    struct Row3 {
        Coloum1: PartStatus,
        Coloum2: PartStatus,
        Coloum3: PartStatus,
    
}

    struct Board {
        Row1: Row1,
        Row2: Row2,
        Row3: Row3,
    }
    let mut board_values = read_board(Board);
    draw_game_board();
}

// Prints out the game board
fn draw_game_board() {
    let mut test = "x";
    println!("{}|{}|{}", test, test ,test);
    println!("_ _ _");
    println!("{}|{}|{}", test, test ,test);
    println!("_ _ _");
    println!("{}|{}|{}", test, test ,test);
}

fn read_board(&Board:) {

}