#[cfg(test)]
mod genetic_algorithm {
    use rust_genetic::{dna::Dna, genetic_algorithm::GeneticAlgorithm};
    use std::{
        collections::HashMap,
        sync::atomic::{AtomicUsize, Ordering},
    };

    const POPULATION_SIZE: i32 = 4;
    const SELECTION_SIZE: i32 = 2;

    fn setup_genetic_algorithm() -> GeneticAlgorithm<i32> {
        // Arrange
        let mut genetic_algorithm: GeneticAlgorithm<i32> = GeneticAlgorithm::new();
        genetic_algorithm.add_dna(Dna { values: vec![1] });
        genetic_algorithm.add_dna(Dna { values: vec![3] });
        genetic_algorithm.add_dna(Dna { values: vec![-6] });
        genetic_algorithm.add_dna(Dna { values: vec![0] });
        return genetic_algorithm;
    }

    #[test]
    fn can_compute_fitness() {
        // Arrange
        let genetic_algorithm = setup_genetic_algorithm();

        // Act
        let ids_and_scores = genetic_algorithm.compute_fitness(compute_fitness_score_for_dna);

        // Assert
        assert_eq!(ids_and_scores.get(0).unwrap().0, 2);
        assert_eq!(ids_and_scores.get(0).unwrap().1, 36.0);

        assert_eq!(ids_and_scores.get(1).unwrap().0, 1);
        assert_eq!(ids_and_scores.get(1).unwrap().1, 9.0);

        assert_eq!(ids_and_scores.get(2).unwrap().0, 0);
        assert_eq!(ids_and_scores.get(2).unwrap().1, 1.0);

        assert_eq!(ids_and_scores.get(3).unwrap().0, 3);
        assert_eq!(ids_and_scores.get(3).unwrap().1, 0.0);
    }

    #[test]
    fn can_select_next_generation() {
        // Arrange
        let mut genetic_algorithm = setup_genetic_algorithm();
        let ids_and_scores = vec![(2, 36.0), (1, 9.0), (0, 1.0), (3, 0.0)];

        // Act
        genetic_algorithm.select_next_generation(&ids_and_scores, &SELECTION_SIZE);

        // Assert
        assert_eq!(genetic_algorithm.dna_by_id.len(), SELECTION_SIZE as usize);
        assert!(genetic_algorithm.dna_by_id.contains_key(&2));
        assert!(genetic_algorithm.dna_by_id.contains_key(&1));
    }

    #[test]
    fn can_populate() {
        // Arrange
        let mut genetic_algorithm = setup_genetic_algorithm();
        let ids_and_scores = vec![(2, 36.0), (1, 9.0), (0, 1.0), (3, 0.0)];

        // Act
        genetic_algorithm.select_next_generation(&ids_and_scores, &SELECTION_SIZE);
        genetic_algorithm.populate(
            &POPULATION_SIZE, // Since we have selected 2, it will create two child
            select_parent,
            create_new_gene_from_parents,
        );

        // Assert
        assert_eq!(genetic_algorithm.dna_by_id.len(), POPULATION_SIZE as usize);

        /*
         * These two should be selected
         */
        assert!(genetic_algorithm.dna_by_id.contains_key(&2)); 
        assert!(genetic_algorithm.dna_by_id.contains_key(&1));
    }

    #[test]
    fn when_a_new_generation_is_computed_should_increment_generation_counter() {
        // Arrange
        let mut genetic_algorithm = setup_genetic_algorithm();

        // Act
        genetic_algorithm.compute_next_generation(
            compute_fitness_score_for_dna,
            &SELECTION_SIZE,
            &POPULATION_SIZE,
            select_parent,
            create_new_gene_from_parents,
        );

        // Assert
        assert_eq!(genetic_algorithm.generation_number, 1);
    }

    fn select_parent(parents: &HashMap<i32, Dna<i32>>) -> &Dna<i32> {
        static COUNTER: AtomicUsize = AtomicUsize::new(2);

        if COUNTER.load(Ordering::Relaxed) == 2 {
            COUNTER.store(1, Ordering::Relaxed);
            return parents.get(&2).unwrap();
        }

        if COUNTER.load(Ordering::Relaxed) == 1 {
            return parents.get(&1).unwrap();
        }

        panic!("No element in hashmap");
    }

    fn create_new_gene_from_parents(v1: &i32, v2: &i32) -> i32 {
        return (v1 + v2) / 2;
    }

    fn compute_fitness_score_for_dna(_id: &i32, dna: &Dna<i32>) -> f64 {
        let value = *dna.values.get(0).unwrap();
        return (value * value) as f64;
    }
}
