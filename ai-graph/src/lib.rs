#![feature(tool_lints)]
#![warn(clippy::pedantic)]
mod breed;
mod new_genes;
mod output;
mod tests;
mod validate;
#[derive(Clone, Copy, Debug)]
enum MutationLine {
    Pass,
    Reset,
    Multiply(i8),
    Divide(i8),
    Add(i8),
}
#[derive(Clone, Copy, Debug)]
enum MutationNode {
    Multiply,
    Divide,
    Add,
}

#[derive(Clone, Debug)]
pub struct Gene {
    line_dna: Vec<Vec<Vec<MutationLine>>>,
    node_dna: Vec<Vec<MutationNode>>,
}
// mod ai_graph {
//     use Gene;
// }
