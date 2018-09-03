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
    fn output(input: Vec<i32>, gene: Gene) -> Vec<f64> {
        vec![0.0; 9]
    }
}
