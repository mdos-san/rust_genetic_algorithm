use crate::{agent::Agent, population::Population};

pub struct Evaluator<T> {
    evaluate_agent: fn(agent: &mut Agent<T>),
}

impl<T> Evaluator<T> {
    pub fn new(evaluate_agent: fn(agent: &mut Agent<T>)) -> Evaluator<T> {
        return Evaluator { evaluate_agent };
    }

    pub fn evaluate_population(self, population: &mut Population<T>) {
        for agent in population.agents.iter_mut() {
            (self.evaluate_agent)(agent);
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::{agent::Agent, evaluator::Evaluator, population::Population};

    fn generate_dna() -> Vec<i32> {
        return vec![1, 2, 3];
    }

    fn evaluate_agent(agent: &mut Agent<i32>) {
        agent.fitness = 4.2;
    }

    #[test]
    fn can_evaluate_fitness_of_a_population() {
        let mut population = Population::new(1024, generate_dna);
        let evaluator = Evaluator::new(evaluate_agent);
        evaluator.evaluate_population(&mut population);
        assert_eq!(population.agents.get(0).unwrap().fitness, 4.2);
        assert_eq!(population.agents.get(1023).unwrap().fitness, 4.2);
    }
}
