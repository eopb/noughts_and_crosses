#![cfg_attr(feature = "cargo-clippy", warn(clippy_pedantic))]
mod new_genes;
#[derive(Clone, Copy, Debug)]
enum MutationLine {
    None,
    Multiply(i8),
    Add(i8),
}
#[derive(Clone, Copy, Debug)]
enum MutationNode {
    Multiply,
    Add,
}
#[derive(Debug)]
struct MutationNodeStorage {
    node_type: MutationNode,
    stored_data: Option<f64>,
}
#[derive(Clone, Debug)]
pub struct Gene {
    line_dna: Vec<Vec<Vec<MutationLine>>>,
    node_dna: Vec<Vec<MutationNode>>,
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
}
impl Gene {
    pub fn output(self, input: Vec<i32>) -> Vec<f64> {
        let mut output = vec![0.0; 9];
        let mut node_values = node_value_calc(&self.node_dna);
        print!("node values {:#?}", node_values);
        if !self.validate(input.to_vec()) {
            panic!("Gene is not valid")
        };
        for (node_index, node_tree) in self.line_dna[1].iter().enumerate() {
            for (line_index, line) in node_tree.iter().enumerate() {
                node_values[0][line_index].stored_data =
                    match node_values[0][line_index].stored_data {
                        Some(_x) => Some(
                            node_values[0][line_index]
                                .calc_pass_value(line.calc_pass_value(input[node_index].into())),
                        ),
                        None => Some(
                            node_values[0][line_index]
                                .calc_pass_value(line.calc_pass_value(input[node_index].into())),
                        ),
                    };
            }
        }
        print!("node values updated {:#?}", node_values);
        output[2] = 2.34;
        output[7] = 3.52;
        output
    }
    fn validate(&self, input: Vec<i32>) -> bool {
        for value in &self.line_dna[0] {
            if value.len() == self.node_dna[0].len() {
                continue;
            }
            return false;
        }
        true
    }
}

fn node_value_calc(node_dna: &Vec<Vec<MutationNode>>) -> Vec<Vec<MutationNodeStorage>> {
    let mut output = Vec::new();
    for row in node_dna {
        output.push(node_value_calc_row(&row));
    }
    output
}
fn node_value_calc_row(row: &Vec<MutationNode>) -> Vec<MutationNodeStorage> {
    let mut output = Vec::new();
    for node in row {
        output.push(convert_mut_node_to_mut_node_store(&node));
    }
    output
}

fn convert_mut_node_to_mut_node_store(node: &MutationNode) -> MutationNodeStorage {
    MutationNodeStorage {
        node_type: *node,
        stored_data: None,
    }
}

impl MutationLine {
    fn calc_pass_value(&self, input_value: f64) -> f64 {
        match self {
            MutationLine::Multiply(x) => input_value * *x as f64,
            MutationLine::Add(x) => input_value + *x as f64,
            MutationLine::None => input_value,
        }
    }
}

impl MutationNodeStorage {
    fn calc_pass_value(&self, input_value: f64) -> f64 {
        match self.stored_data {
            Some(data) => match self.node_type {
                MutationNode::Add => data + input_value,
                MutationNode::Multiply => {
                    println!("data {}, inputvalue {}", data, input_value);
                    data * input_value
                }
            },
            None => input_value,
        }
    }
}
