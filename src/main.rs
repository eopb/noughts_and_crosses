fn main() {
    println!("Welcome to my noughts and crosses game made in rust.");
    enum PartStatus {
        PlayerX,
        PlayerY,
        Selected,
        Empty,
    }
    struct row1 {
        coloum1: PartStatus,
        coloum2: PartStatus,
        coloum3: PartStatus,
    }
    struct row2 {
        coloum1: PartStatus,
        coloum2: PartStatus,
        coloum3: PartStatus,
    
    }
    struct row3 {
        coloum1: PartStatus,
        coloum2: PartStatus,
        coloum3: PartStatus,
    
}

    struct Board {
        row1: row1,
        row2: row2,
        row3: row3,
    }
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
