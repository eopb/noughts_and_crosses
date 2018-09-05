#![cfg_attr(feature = "cargo-clippy", warn(clippy_pedantic))]
#![allow(single_match_else)]
#![feature(type_ascription)]
extern crate ai_graph;
extern crate noughts_and_crosses_lib;

use ai_graph::Gene;
use noughts_and_crosses_lib::{GameBoard, Players, Winner};

#[derive(Debug, Clone)]
struct GeneStorage {
    gene: Gene,
    score: u32,
}

fn main() {
    let mut scores = Vec::new();
    for _x in 0..10 {
        let gene_tested = Gene::new_random_gene();
        for _x in 0..10 {
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
            }
            scores.push(GeneStorage {
                gene: gene_tested.clone(),
                score: score_values.iter().sum(): u32 / score_values.len() as u32,
            });
        }
    }
    println!("{:#?}", scores);
    println!("Hello, world!");
}
