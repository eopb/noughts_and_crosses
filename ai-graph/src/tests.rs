#[cfg(test)]
use crate::Gene;
#[test]
fn random_gene_is_valid() {
    let test_gene = Gene::new_random_gene();
    if !test_gene.validate() {
        panic!("Gene is not valid")
    };
}

#[test]
fn random_breed_is_valid() {
    let mut test_gene = Gene::new_random_gene();
    let test_gene2 = Gene::new_random_gene();
    test_gene = test_gene.breed(&test_gene2);
    println!("{:#?}", test_gene);
    if test_gene.validate() {
        panic!("Gene is not valid")
    };
}
