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
            LineDna: MutationLine::rand_vec3(2, 9, 9),
            NodeDna: MutationNode::rand_vec2(2, 9),
        }
    }
}

impl MutationLine {
    fn rand_vec3(num: u32, num2: u32, num3: u32) -> Vec<Vec<Vec<Self>>> {
        let mut rand_vec = Vec::new();
        for _x in 0..num {
            rand_vec.push(Self::rand_vec2(num2, num3));
        }
        rand_vec
    }
    fn rand_vec2(num: u32, num2: u32) -> Vec<Vec<Self>> {
        let mut rand_vec = Vec::new();
        for _x in 0..num {
            rand_vec.push(Self::rand_vec(num2));
        }
        rand_vec
    }
    fn rand_vec(num: u32) -> Vec<Self> {
        let mut rand_vec = Vec::new();
        for _x in 0..num {
            rand_vec.push(Self::rand_mut());
        }
        rand_vec
    }
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
    fn rand_vec2(num: u32, num2: u32) -> Vec<Vec<Self>> {
        let mut rand_vec = Vec::new();
        for _x in 0..num {
            rand_vec.push(Self::rand_vec(num2));
        }
        rand_vec
    }
    fn rand_vec(num: u32) -> Vec<Self> {
        let mut rand_vec = Vec::new();
        for _x in 0..num {
            rand_vec.push(Self::rand_mut());
        }
        rand_vec
    }
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
