use crate::Gene;

impl Gene {
    pub fn validate(&self) -> bool {
        for value in &self.line_dna[0] {
            if value.len() == self.node_dna[0].len() {
                continue;
            }
            return false;
        }
        true
    }
}
