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
}
