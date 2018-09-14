use crate::Gene;

//TODO Not all checks are finnished.
impl Gene {
    pub fn validate(&self) -> bool {
        self.all_lines_per_row_equel() && self.all_output_avalible()
    }
    pub fn validate_two(&self, second_gene: &Self) -> bool {
        self.all_lines_per_row_equel() && self.all_output_avalible() && self.equel_size(second_gene)
    }

    fn all_lines_per_row_equel(&self) -> bool {
        for line_block in &self.line_dna {
            let number = line_block[0].len();
            for line_set in line_block {
                if !line_set.len() == number {
                    return false;
                }
            }
        }
        return true;
    }
    fn all_output_avalible(&self) -> bool {
        true
    }
    fn equel_size(&self, second_gene: &Self) -> bool {
        true
    }
}
