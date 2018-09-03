#![cfg_attr(feature = "cargo-clippy", warn(clippy_pedantic))]
mod ai_graph {
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
    pub fn run(inputs: &[f64], output_num: u32) -> Vec<&f64> {
        let mut output = Vec::new();
        for input in inputs {
            output.push(input);
        }
        output
    }
}
