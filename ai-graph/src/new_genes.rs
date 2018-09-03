extern crate rand;
use self::rand::Rng;
use Gene;
use MutationLine;
use MutationNode;
impl Gene {
    pub fn new_gene() -> Gene {
        Gene {
            LineDna: vec![vec![vec![MutationLine::None; 9]; 9]; 2],
            NodeDna: vec![vec![MutationNode::Add; 9]; 2],
        }
    }
    pub fn new_random_gene() -> Gene {
        Gene {
            LineDna: vec![
                vec![
                    vec![
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                    ],
                    vec![
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                    ],
                    vec![
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                    ],
                    vec![
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                    ],
                    vec![
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                    ],
                    vec![
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                    ],
                    vec![
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                    ],
                    vec![
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                    ],
                    vec![
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                    ],
                ],
                vec![
                    vec![
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                    ],
                    vec![
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                    ],
                    vec![
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                    ],
                    vec![
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                    ],
                    vec![
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                    ],
                    vec![
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                    ],
                    vec![
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                    ],
                    vec![
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                    ],
                    vec![
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                        MutationLine::rand_mut(),
                    ],
                ],
            ],
            NodeDna: vec![
                vec![
                    MutationNode::rand_mut(),
                    MutationNode::rand_mut(),
                    MutationNode::rand_mut(),
                    MutationNode::rand_mut(),
                    MutationNode::rand_mut(),
                    MutationNode::rand_mut(),
                    MutationNode::rand_mut(),
                    MutationNode::rand_mut(),
                    MutationNode::rand_mut(),
                ],
                vec![
                    MutationNode::rand_mut(),
                    MutationNode::rand_mut(),
                    MutationNode::rand_mut(),
                    MutationNode::rand_mut(),
                    MutationNode::rand_mut(),
                    MutationNode::rand_mut(),
                    MutationNode::rand_mut(),
                    MutationNode::rand_mut(),
                    MutationNode::rand_mut(),
                ],
            ],
        }
    }
}

impl MutationLine {
    fn rand_mut() -> Self {
        let mut rng = rand::thread_rng();
        let line_types = [
            MutationLine::None,
            MutationLine::Multiply(rand_i64()),
            MutationLine::Add(rand_i64()),
            MutationLine::Subtract(rand_i64()),
            MutationLine::Divide(rand_i64()),
            MutationLine::Power(rand_i64()),
            MutationLine::Root(rand_i64()),
        ];
        *rng.choose(&line_types).unwrap()
    }
}

impl MutationNode {
    fn rand_mut() -> Self {
        let mut rng = rand::thread_rng();
        let node_types = [
            MutationNode::Multiply,
            MutationNode::Add,
            MutationNode::Divide,
            MutationNode::Subtract,
        ];
        *rng.choose(&node_types).unwrap()
    }
}

fn rand_i64() -> i64 {
    rand::thread_rng().gen_range(i64::min_value(), i64::max_value())
}
