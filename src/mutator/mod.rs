use crate::agent::Agent;
use rand::Rng;

pub struct Mutator<T> {
    mutate_agent: fn(agent: &mut Agent<T>),
    mutation_rate: f64,
}

impl<T> Mutator<T> {
    pub fn new(mutate_agent: fn(agent: &mut Agent<T>), mutation_rate: f64) -> Mutator<T> {
        Mutator {
            mutate_agent,
            mutation_rate,
        }
    }

    pub fn mutate(&self, agent: &mut Agent<T>) {
        let mut rng = rand::thread_rng();
        let should_mutate = rng.gen_bool(self.mutation_rate);

        if should_mutate {
            (self.mutate_agent)(agent);
        }
    }
}

#[cfg(test)]
pub mod unit_tests {
    use crate::{agent::Agent, mutator::Mutator};

    fn mutate_agent(agent: &mut Agent<i32>) {
        for i in 0..agent.dna.len() {
            *agent.dna.get_mut(i).unwrap() = 1;
        }
    }

    #[test]
    fn can_mutate_an_agent_with_1_mutation_rate() {
        let mut agent = Agent::new(vec![0, 0, 0], 0.0);
        let mutator = Mutator::new(mutate_agent, 1.0);

        // Act
        mutator.mutate(&mut agent);

        // Assert
        assert!(agent.dna.iter().all(|gene| *gene == 1));
    }

    #[test]
    fn can_mutate_an_agent_0_mutation_rate() {
        let mut agent = Agent::new(vec![0, 0, 0], 0.0);
        let mutator = Mutator::new(mutate_agent, 0.0);

        // Act
        mutator.mutate(&mut agent);

        // Assert
        assert!(agent.dna.iter().all(|gene| *gene == 0));
    }
}
