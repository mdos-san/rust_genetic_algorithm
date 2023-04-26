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

        for index_child in (self.population_size / 2)..self.population_size {
            let index_parent_1 = rng.gen_range(0..(self.agents.len() - 1));
            let index_parent_2 = rng.gen_range(0..(self.agents.len() - 1));

            Agent::from_parents(&mut self.agents, index_parent_1, index_parent_2, index_child);
        }
    }
}

#[cfg(test)]
pub mod unit_tests {
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
        for i in 0..1024 {
            population.agents.get_mut(i).unwrap().fitness = 42.0;
        }

        // Act
        population.repopulate();

        // Assert
        assert_eq!(population.agents.len(), 1024);
        assert_eq!(population.agents.get(0).unwrap().fitness, 42.0);
        assert_eq!(population.agents.get(1023).unwrap().fitness, 0.0);
    }
}
