use crate::{agent::Agent, population::Population};

pub struct Evaluator<T, K> {
    evaluate_agent: fn(agent: &mut Agent<T>, context: &mut K),
}

impl<T, K> Evaluator<T, K> {
    pub fn new(evaluate_agent: fn(agent: &mut Agent<T>, context: &mut K)) -> Evaluator<T, K> {
        return Evaluator { evaluate_agent };
    }

    pub fn evaluate_population(&self, population: &mut Population<T>, context: &mut K) {
        for agent in population.agents.iter_mut() {
            (self.evaluate_agent)(agent, context);
        }
    }
}

#[cfg(test)]
pub mod unit_tests {
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
    fn can_evaluate_fitness_of_a_population() {
        // Arrange
        let mut population = Population::new(1024, generate_dna);
        let mut test_context = TestContext { counter: 1 };
        let evaluator = Evaluator::new(evaluate_agent);

        // Act
        evaluator.evaluate_population(&mut population, &mut test_context);

        // Assert
        assert_eq!(population.agents.get(0).unwrap().fitness, 1.0);
        assert_eq!(population.agents.get(1).unwrap().fitness, 2.0);
        assert_eq!(population.agents.get(2).unwrap().fitness, 3.0);
        assert_eq!(population.agents.get(1023).unwrap().fitness, 1024.0);
    }
}
