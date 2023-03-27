use crate::agent::Agent;

pub struct Population<T> {
    pub agents: Vec<Agent<T>>,
}

impl<T> Population<T>
where
    T: Copy,
{
    pub fn new(size: usize, generate_dna: fn() -> Vec<T>) -> Population<T> {
        let mut new_population = Population { agents: vec![] };

        for _ in 0..size {
            new_population.agents.push(Agent::new(generate_dna(), 0.0));
        }

        return new_population;
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
}
