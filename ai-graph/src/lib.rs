#![cfg_attr(feature = "cargo-clippy", warn(clippy_pedantic))]
mod new_gene;
enum MutationLine {
    None,
    Multiply(i64),
    Add(i64),
    Subtract(i64),
    Divide(i64),
    Power(i64),
    Root(i64),
}
enum MutationNode {
    Multiply,
    Add,
    Divide,
    Subtract,
}
struct Gene {
    LineDna: [[[MutationLine; 9]; 9]; 2],
    NodeDna: [[MutationNode; 9]; 2],
}
mod ai_graph {
    pub fn run(inputs: &[f64], output_num: u32) -> Vec<&f64> {
        let mut output = Vec::new();
        for input in inputs {
            output.push(input);
        }
        output
    }
}
