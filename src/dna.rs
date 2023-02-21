pub struct Dna<T> {
    pub values: Vec<T>,
}

impl<T> Dna<T> {
    pub fn crossover(
        dna1: &Dna<T>,
        dna2: &Dna<T>,
        create_new_gene_from_parents: fn(&T, &T) -> T,
    ) -> Dna<T> {
        let mut child: Dna<T> = Dna { values: vec![] };

        for counter in 0..dna1.values.len() {
            let v1 = dna1.values.get(counter).unwrap();
            let v2 = dna2.values.get(counter).unwrap();

            let child_gene = create_new_gene_from_parents(v1, v2);
            child.values.push(child_gene);
        }

        return child;
    }

    pub fn mutate(dna1: &mut Dna<T>, mutate_current_value: &mut dyn FnMut(&T) -> T) {
        for dna_value in &mut dna1.values {
            *dna_value = mutate_current_value(dna_value);
        }
    }
}
