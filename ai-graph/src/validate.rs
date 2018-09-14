use crate::Gene;

impl Gene {
    pub fn validate(&self) -> bool {
        self.all_lines_per_row_equel() && self.all_output_avalible()
    }
    fn all_lines_per_row_equel(&self) -> bool {
        true
    }
    fn all_output_avalible(&self) -> bool {
        true
    }
}
