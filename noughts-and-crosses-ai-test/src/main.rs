#![cfg_attr(feature = "cargo-clippy", warn(clippy_pedantic))]
extern crate ai_graph;
use ai_graph::Gene;
fn main() {
    let test_gene = Gene::new_random_gene();
    println!("{:#?}", test_gene);
    println!(
        "{:#?}",
        test_gene.output([1, 0, 1, 1, 1, 0, 0, 1, 1].to_vec())
    );
    println!("Hello, world!");
}
