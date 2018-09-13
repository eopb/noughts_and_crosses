#[cfg(test)]
mod tests {
    use crate::Gene;
    #[test]
    fn random_gene_is_valid() {
        let test_gene = Gene::new_random_gene();
        if !test_gene.validate() {
            panic!("Gene is not valid")
        };
    }
}
