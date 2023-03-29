#[cfg(test)]
pub mod functional_tests {
    use rand::Rng;
    use rust_genetic_algorithm::{agent::Agent, GeneticAlgorithm};

    struct TestContext {}

    fn generate_dna() -> Vec<i32> {
        let mut rng = rand::thread_rng();

        return vec![
            rng.gen_range(0..5), // We don't provide all possible genes, to test mutations
            rng.gen_range(0..5),
            rng.gen_range(0..5),
            rng.gen_range(0..5),
            rng.gen_range(0..5),
            rng.gen_range(0..5),
            rng.gen_range(0..5),
            rng.gen_range(0..5),
            rng.gen_range(0..5),
            rng.gen_range(0..5),
        ];
    }

    fn evaluate_agent(agent: &mut Agent<i32>, _: &mut TestContext) {
        let mut fitness = 0;

        for index in 0..10 {
            let value = *agent.dna.get(index).unwrap();
            fitness -= i32::abs(value - index as i32);
        }

        agent.fitness = fitness.into();
    }

    fn mutate_agent(agent: &mut Agent<i32>) {
        if agent.fitness != 0.0 {
            return;
        }

        let mut rng = rand::thread_rng();
        for i in 0..agent.dna.len() {
            let should_mutate = rng.gen_bool(0.1);
            if !should_mutate {
                continue;
            }

            let add = rng.gen_bool(0.5);

            if add {
                *agent.dna.get_mut(i).unwrap() += 1;
            } else {
                *agent.dna.get_mut(i).unwrap() -= 1;
            }

            let v = *agent.dna.get(i).unwrap();
            *agent.dna.get_mut(i).unwrap() = v.clamp(0, 9);
        }
    }

    #[test]
    pub fn can_generate_an_ordered_dna_sequence() {
        let mut ga = GeneticAlgorithm::new(1024, generate_dna, evaluate_agent, mutate_agent);

        for _ in 0..100 {
            ga.compute_next_generation(&mut TestContext {});
        }

        let best_agent = ga.population.agents.get(0).unwrap();
        print!("{:?}", best_agent.dna);
        assert_eq!(best_agent.fitness, 0.0);
        assert_eq!(*best_agent.dna.get(0).unwrap(), 0);
        assert_eq!(*best_agent.dna.get(1).unwrap(), 1);
        assert_eq!(*best_agent.dna.get(2).unwrap(), 2);
        assert_eq!(*best_agent.dna.get(3).unwrap(), 3);
        assert_eq!(*best_agent.dna.get(4).unwrap(), 4);
        assert_eq!(*best_agent.dna.get(5).unwrap(), 5);
        assert_eq!(*best_agent.dna.get(6).unwrap(), 6);
        assert_eq!(*best_agent.dna.get(7).unwrap(), 7);
        assert_eq!(*best_agent.dna.get(8).unwrap(), 8);
        assert_eq!(*best_agent.dna.get(9).unwrap(), 9);
    }
}
