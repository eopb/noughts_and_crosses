// Prints out the game board

pub fn draw_game_board(
    row1: &[&str; 3],
    row2: &[&str; 3],
    row3: &[&str; 3],
    player_position: &[i32; 2],
) {
    let mut player11 = String::new();
    let mut player12 = String::new();
    let mut player13 = String::new();
    let mut player21 = String::new();
    let mut player22 = String::new();
    let mut player23 = String::new();
    let mut player31 = String::new();
    let mut player32 = String::new();
    let mut player33 = String::new();
    if player_position[0] == 1 {
        if player_position[1] == 1 {
            player11 = "*".to_string();
        } else if player_position[1] == 2 {
            player12 = "*".to_string();
        } else if player_position[1] == 3 {
            player13 = "*".to_string();
        }
    } else if player_position[0] == 2 {
        if player_position[1] == 1 {
            player21 = "*".to_string();
        } else if player_position[1] == 2 {
            player22 = "*".to_string();
        } else if player_position[1] == 3 {
            player23 = "*".to_string();
        }
    } else if player_position[0] == 3 {
        if player_position[1] == 1 {
            player31 = "*".to_string();
        } else if player_position[1] == 2 {
            player32 = "*".to_string();
        } else if player_position[1] == 3 {
            player33 = "*".to_string();
        }
    }
    println!(
        "{}{}|{}{}|{}{}",
        row1[0], player11, row1[1], player12, row1[2], player13
    );
    println!("_ _ _");
    println!(
        "{}{}|{}{}|{}{}",
        row2[0], player21, row2[1], player22, row2[2], player23
    );
    println!("_ _ _");
    println!(
        "{}{}|{}{}|{}{}",
        row3[0], player31, row3[1], player32, row3[2], player33
    );
}
