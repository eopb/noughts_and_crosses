#![cfg_attr(feature = "cargo-clippy", warn(clippy_pedantic))]
#![allow(single_match_else)]
#![feature(type_ascription)]
extern crate ai_graph;
extern crate noughts_and_crosses_lib;

use ai_graph::Gene;
use noughts_and_crosses_lib::{GameBoard, GameMode, Players, Winner};

#[derive(Debug, Clone)]
struct GeneStorage {
    gene: Gene,
    score: f64,
}

fn main() {
    let mut scores = Vec::new();
    for _x in 0..5 {
        let gene_tested = Gene::new_random_gene();
        for _x in 0..30 {
            let mut game_board = GameBoard::empty_board();
            let mut score_values = Vec::new();
            loop {
                game_board = match game_board.random_placement(Players::Cross) {
                    Some(game_board) => game_board,
                    None => {
                        score_values.push(1);
                        break;
                    }
                };
                match game_board.has_someone_won() {
                    Winner::Nought => {
                        panic!("error 1");
                    }
                    Winner::Cross => {
                        score_values.push(0);
                        break;
                    }
                    Winner::None => (),
                };
                game_board.draw_game_board(&GameMode::Spectate);
                game_board = match game_board.place_largest_empty(
                    &gene_tested.clone().output(&[1, 0, 1, 1, 1, 0, 0, 1, 1]),
                    Players::Nought,
                ) {
                    Some(game_board) => game_board,
                    None => {
                        score_values.push(0);
                        break;
                    }
                };
                game_board.draw_game_board(&GameMode::Spectate);
                match game_board.has_someone_won() {
                    Winner::Nought => {
                        score_values.push(3);
                        break;
                    }
                    Winner::Cross => {
                        panic!("error 1");
                    }
                    Winner::None => (),
                };
            }
            scores.push(GeneStorage {
                gene: gene_tested.clone(),
                score: score_values
                    .iter()
                    .cloned()
                    .map(|val| val as f64)
                    .sum::<f64>() as f64
                    / score_values.len() as f64,
            });
        }
    }
    println!("{:#?}", scores);
    println!("Hello, world!");
}
