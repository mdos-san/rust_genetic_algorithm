use rand::Rng;

use crate::agent::Agent;

pub struct Population<T> {
    pub agents: Vec<Agent<T>>,
    pub population_size: usize,
}

impl<T> Population<T>
where
    T: Copy,
{
    pub fn new(size: usize, generate_dna: fn() -> Vec<T>) -> Population<T> {
        let mut new_population = Population {
            agents: vec![],
            population_size: size,
        };

        for _ in 0..size {
            new_population.agents.push(Agent::new(generate_dna(), 0.0));
        }

        return new_population;
    }

    pub fn repopulate(&mut self) {
        let mut rng = rand::thread_rng();

        while self.agents.len() < self.population_size {
            let index_1 = rng.gen_range(0..(self.agents.len() - 1));
            let index_2 = rng.gen_range(0..(self.agents.len() - 1));

            let agent_1 = self.agents.get(index_1).unwrap();
            let agent_2 = self.agents.get(index_2).unwrap();

            let new_agent = Agent::from_parents(agent_1, agent_2);

            self.agents.push(new_agent);
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::population::Population;

    fn generate_dna() -> Vec<i32> {
        return vec![1, 2, 3];
    }

    #[test]
    fn can_create_a_population() {
        let population = Population::new(1024, generate_dna);
        assert_eq!(population.agents.len(), 1024);
        assert_eq!(population.agents.get(0).unwrap().dna.len(), 3);
        assert_eq!(population.agents.get(1023).unwrap().dna.len(), 3);
    }

    #[test]
    fn can_repopulate() {
        // Arrange
        let mut population = Population::new(1024, generate_dna);
        for _ in 0..512 {
            population.agents.remove(population.agents.len() - 1);
        }

        // Act
        population.repopulate();

        // Assert
        assert_eq!(population.agents.len(), 1024);
        assert_eq!(population.agents.get(0).unwrap().dna.len(), 3);
        assert_eq!(population.agents.get(1023).unwrap().dna.len(), 3);
    }
}
