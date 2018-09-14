use crate::Gene;

impl Gene {
    pub fn validate(&self) -> bool {
        self.all_lines_per_row_equel() && self.all_output_avalible()
    }
    pub fn validate_two(&self, second_gene: &Self) -> bool {
        self.all_lines_per_row_equel() && self.all_output_avalible() && self.equel_size(second_gene)
    }
    fn all_lines_per_row_equel(&self) -> bool {
        true
    }
    fn all_output_avalible(&self) -> bool {
        true
    }
    fn equel_size(&self, second_gene: &Self) -> bool {
        true
    }
}
