use std::collections::HashMap;

use crate::dna::Dna;

pub struct GeneticAlgorithm<T> {
    global_id: i32,
    pub dna_by_id: HashMap<i32, Dna<T>>,
    pub generation_number: i32,
}

impl<T> GeneticAlgorithm<T> {
    pub fn new() -> GeneticAlgorithm<T> {
        return GeneticAlgorithm {
            dna_by_id: HashMap::new(),
            global_id: 0,
            generation_number: 0,
        };
    }

    pub fn add_dna(&mut self, dna: Dna<T>) {
        self.dna_by_id.insert(self.global_id, dna);
        self.global_id += 1;
    }

    pub fn sort_by_score(v: &mut Vec<(i32, f64)>) {
        v.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        v.reverse();
    }

    pub fn compute_fitness(
        &self,
        compute_fitness_score_for_dna: fn(&i32, &Dna<T>) -> f64,
    ) -> Vec<(i32, f64)> {
        let mut list_of_id_and_score = vec![];

        for dna in &self.dna_by_id {
            let id_and_score = compute_fitness_score_for_dna(dna.0, dna.1);
            list_of_id_and_score.push((*dna.0, id_and_score));
        }

        Self::sort_by_score(&mut list_of_id_and_score);

        return list_of_id_and_score;
    }

    pub fn select_next_generation(
        &mut self,
        ids_and_scores: &Vec<(i32, f64)>,
        number_of_survivor: &i32,
    ) {
        let len_of_ids_and_scores: i32 = ids_and_scores.len() as i32;
        if len_of_ids_and_scores <= *number_of_survivor {
            return;
        }

        for index in *number_of_survivor..len_of_ids_and_scores {
            let id_and_score = ids_and_scores.get(index as usize).unwrap();
            self.dna_by_id.remove(&id_and_score.0);
        }
    }

    pub fn populate(
        &mut self,
        population_number: &i32,
        select_parent: fn(&HashMap<i32, Dna<T>>) -> &Dna<T>,
        create_new_gene_from_parents: fn(&T, &T) -> T,
    ) {
        let number_of_new_child = population_number - self.dna_by_id.len() as i32;

        for _ in 0..number_of_new_child {
            let parent1 = select_parent(&self.dna_by_id);
            let parent2 = select_parent(&self.dna_by_id);
            let child = Dna::crossover(parent1, parent2, create_new_gene_from_parents);
            self.add_dna(child);
        }
    }

    pub fn compute_next_generation(
        &mut self,
        compute_fitness_score_for_dna: fn(&i32, &Dna<T>) -> f64,
        number_of_survivor: &i32,
        population_number: &i32,
        select_parent: fn(&HashMap<i32, Dna<T>>) -> &Dna<T>,
        create_new_gene_from_parents: fn(&T, &T) -> T,
    ) {
        let ids_and_scores = self.compute_fitness(compute_fitness_score_for_dna);
        self.select_next_generation(&ids_and_scores, number_of_survivor);
        self.populate(
            population_number,
            select_parent,
            create_new_gene_from_parents,
        );
        self.generation_number += 1;
    }
}
