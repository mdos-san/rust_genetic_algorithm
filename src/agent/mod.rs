pub struct Agent<T> {
    pub fitness: f64,
    pub dna: Vec<T>,
}

impl<T> Agent<T> {
    pub fn new(dna: Vec<T>, fitness: f64) -> Agent<T> {
        return Agent { dna, fitness };
    }
}

#[cfg(test)]
pub mod tests {
    use crate::agent::Agent;

    #[test]
    fn can_create_an_agent() {
        let agent = Agent::new(vec![1, 2, 3], 0.0);
        assert_eq!(*agent.dna.get(0).unwrap(), 1);
        assert_eq!(*agent.dna.get(1).unwrap(), 2);
        assert_eq!(*agent.dna.get(2).unwrap(), 3);
        assert_eq!(agent.fitness, 0.0);
    }
}
