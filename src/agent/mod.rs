use std::collections::HashMap;

use rand::Rng;

pub struct Agent<T> {
    pub fitness: f64,
    pub dna: Vec<T>,
    pub metadata: HashMap<String, String>
}

impl<T> Agent<T>
where
    T: Copy,
{
    pub fn new(dna: Vec<T>, fitness: f64) -> Agent<T> {
        Agent { dna, fitness, metadata: HashMap::new() }
    }

    pub fn from_parents(agent_1: &Agent<T>, agent_2: &Agent<T>) -> Agent<T> {
        let mut child_dna = vec![];

        let mut rng = rand::thread_rng();
        for index in 0..agent_1.dna.len() {
            let selected_agent = match rng.gen_range(0..2) {
                0 => agent_1,
                _ => agent_2,
            };
            let gene = *selected_agent.dna.get(index).unwrap();

            child_dna.push(gene);
        }

        Agent {
            dna: child_dna,
            fitness: 0.0,
            metadata: HashMap::new()
        }
    }
}

#[cfg(test)]
pub mod unit_tests {
    use crate::agent::Agent;

    #[test]
    fn can_create_an_agent() {
        let agent = Agent::new(vec![1, 2, 3], 0.0);
        assert_eq!(*agent.dna.get(0).unwrap(), 1);
        assert_eq!(*agent.dna.get(1).unwrap(), 2);
        assert_eq!(*agent.dna.get(2).unwrap(), 3);
        assert_eq!(agent.fitness, 0.0);
    }

    #[test]
    fn can_create_agent_from_two_parents() {
        let agent_1 = Agent::new(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 0.0);
        let agent_2 = Agent::new(vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2], 0.0);
        let child_agent = Agent::from_parents(&agent_1, &agent_2);

        assert_eq!(child_agent.fitness, 0.0);
        assert_eq!(child_agent.dna.len(), 10);
        assert!(child_agent.dna.iter().any(|gene| *gene == 1));
        assert!(child_agent.dna.iter().any(|gene| *gene == 2));
    }
}
