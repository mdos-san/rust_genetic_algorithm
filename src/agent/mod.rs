use std::collections::HashMap;

use rand::Rng;

pub struct Agent<T> {
    pub fitness: f64,
    pub dna: Vec<T>,
    pub metadata: HashMap<String, String>,
}

impl<T> Agent<T>
where
    T: Copy,
{
    pub fn new(dna: Vec<T>, fitness: f64) -> Agent<T> {
        Agent {
            dna,
            fitness,
            metadata: HashMap::new(),
        }
    }

    pub fn from_parents(
        agents: &mut Vec<Agent<T>>,
        index_parent_1: usize,
        index_parent_2: usize,
        index_child: usize,
    ) {
        let dna_size = agents.get(index_parent_1).unwrap().dna.len();

        let mut rng = rand::thread_rng();
        for index_dna in 0..dna_size {
            let selected_gene = match rng.gen_range(0..2) {
                0 => agents.get(index_parent_1).unwrap().dna.get(index_dna).unwrap(),
                _ => agents.get(index_parent_2).unwrap().dna.get(index_dna).unwrap(),
            };

            *agents.get_mut(index_child).unwrap().dna.get_mut(index_dna).unwrap() = *selected_gene;
        }

        let agent_3 = agents.get_mut(index_child).unwrap();
        agent_3.fitness = 0.0;
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
        let agent_3 = Agent::new(vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3], 4.0);
        let mut agents = vec![agent_1, agent_2, agent_3];

        Agent::from_parents(&mut agents, 0, 1, 2);

        assert_eq!(agents.get(2).unwrap().fitness, 0.0);
        assert_eq!(agents.get(2).unwrap().dna.len(), 10);
        assert!(agents.get(2).unwrap().dna.iter().any(|gene| *gene == 1));
        assert!(agents.get(2).unwrap().dna.iter().any(|gene| *gene == 2));
        assert!(!agents.get(2).unwrap().dna.iter().any(|gene| *gene == 3));
    }
}
