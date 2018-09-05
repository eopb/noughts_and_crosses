#![cfg_attr(feature = "cargo-clippy", warn(clippy_pedantic))]
extern crate ai_graph;
use ai_graph::Gene;

#[derive(Debug)]
struct GeneStorage {
    gene: Gene,
    score: u8,
}

fn main() {
    let mut scores = Vec::new();
    for _x in 0..10 {
        let gene_tested = Gene::new_random_gene();
        scores.push(GeneStorage {
            gene: gene_tested,
            score: 0,
        });
    }
    println!("{:#?}", scores);
    println!("Hello, world!");
}
