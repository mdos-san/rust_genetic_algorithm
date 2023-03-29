pub mod agent;
pub mod evaluator;
pub mod mutator;
pub mod population;
pub mod selector;

use agent::Agent;
use evaluator::Evaluator;
use mutator::Mutator;
use population::Population;
use selector::Selector;

pub struct GeneticAlgorithm<T, K> {
    pub population: Population<T>,
    pub evaluator: Evaluator<T, K>,
    pub selector: Selector,
    pub mutator: Mutator<T>,
}

impl<T, K> GeneticAlgorithm<T, K>
where
    T: Copy,
{
    pub fn new(
        population_size: usize,
        generate_dna: fn() -> Vec<T>,
        evaluate_agent: fn(agent: &mut Agent<T>, context: &mut K),
        mutate_agent: fn(agent: &mut Agent<T>),
    ) -> GeneticAlgorithm<T, K> {
        let population = Population::new(population_size, generate_dna);
        let evaluator = Evaluator::new(evaluate_agent);
        let selector = Selector::new(population_size / 2);
        let mutator = Mutator::new(mutate_agent, 0.1);

        GeneticAlgorithm {
            population,
            evaluator,
            selector,
            mutator,
        }
    }

    pub fn compute_next_generation(&mut self, context: &mut K) {
        let population = &mut self.population;
        self.evaluator.evaluate_population(population, context);
        self.selector.select_from_population(population);
        self.population.repopulate();

        for i in 0..self.population.agents.len() {
            let agent = self.population.agents.get_mut(i).unwrap();
            self.mutator.mutate(agent);
        }
    }
}
