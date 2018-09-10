use crate::Gene;
use crate::MutationNode;
use rand::Rng;
impl Gene {
    // TODO breed lines
    pub fn breed(&self, second_gene: &Self) -> Self {
        let mut new_values = self.clone();
        // Merge nodes.
        for (node_line_index, node_line) in self.node_dna.iter().enumerate() {
            for (node_index, node) in node_line.iter().enumerate() {
                new_values.node_dna[node_line_index][node_index] =
                    node.node_merge(&second_gene.node_dna[node_line_index][node_index]);
            }
        }
        new_values.clone()
    }
}

impl MutationNode {
    fn node_merge(&self, second_node: &Self) -> Self {
        let mut rng = rand::thread_rng();
        let node_types = [self, &second_node];
        **rng.choose(&node_types).unwrap()
    }
}
