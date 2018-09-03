#![cfg_attr(feature = "cargo-clippy", warn(clippy_pedantic))]
mod new_genes;
#[derive(Clone)]
enum MutationLine {
    None,
    Multiply(i64),
    Add(i64),
    Subtract(i64),
    Divide(i64),
    Power(i64),
    Root(i64),
}
#[derive(Clone)]
enum MutationNode {
    Multiply,
    Add,
    Divide,
    Subtract,
}
#[derive(Clone)]
struct Gene {
    LineDna: Vec<Vec<Vec<MutationLine>>>,
    NodeDna: Vec<Vec<MutationNode>>,
}
mod ai_graph {
    use Gene;
    pub fn run(inputs: &[f64], output_num: u32) -> Vec<&f64> {
        let mut output = Vec::new();
        for input in inputs {
            output.push(input);
        }
        output
    }
    pub fn output(input: [i32; 9], gene: Gene) -> [i32; 9] {}
}
