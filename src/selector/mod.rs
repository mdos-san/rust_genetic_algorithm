use crate::population::Population;

pub struct Selector {
    selection_size: usize,
}

impl Selector {
    pub fn new(selection_size: usize) -> Selector {
        Selector { selection_size }
    }

    pub fn select_from_population<T>(self, population: &mut Population<T>) {
        Selector::sort_agents_by_fitness_desc(population);

        while population.agents.len() > self.selection_size {
            population.agents.remove(population.agents.len() - 1);
        }
    }

    fn sort_agents_by_fitness_desc<T>(population: &mut Population<T>) {
        population
            .agents
            .sort_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap());
        population.agents.reverse();
    }
}

#[cfg(test)]
pub mod unit_tests {
    use super::Selector;
    use crate::{agent::Agent, evaluator::Evaluator, population::Population};

    fn generate_dna() -> Vec<i32> {
        return vec![1, 2, 3];
    }
    struct TestContext {
        counter: i32,
    }

    fn evaluate_agent(agent: &mut Agent<i32>, context: &mut TestContext) {
        agent.fitness = context.counter.into();
        context.counter += 1;
    }

    #[test]
    fn can_select_best_agents_of_a_population() {
        // Arrange
        let mut population = Population::new(1024, generate_dna);
        let mut test_context = TestContext { counter: 1 };
        let evaluator = Evaluator::new(evaluate_agent);
        evaluator.evaluate_population(&mut population, &mut test_context);
        let selector = Selector::new(512);

        // Act
        selector.select_from_population(&mut population);

        // Assert
        assert_eq!(population.agents.len(), 512);
        assert_eq!(population.agents.get(0).unwrap().fitness, 1024.0);
        assert_eq!(population.agents.get(1).unwrap().fitness, 1023.0);
        assert_eq!(population.agents.get(2).unwrap().fitness, 1022.0);
        assert_eq!(population.agents.get(511).unwrap().fitness, 513.0);
    }
}
