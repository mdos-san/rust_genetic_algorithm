#[cfg(test)]
mod dna {
    use rand::Rng;
    use rust_genetic_algorithm::dna::Dna;

    #[test]
    fn can_do_a_crossover() {
        // Arrange
        let dna1 = Dna {
            values: vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        };
        let dna2 = Dna {
            values: vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
        };

        let child: Dna<i32> = Dna::crossover(&dna1, &dna2, |v1, v2| {
            match rand::thread_rng().gen_range(0..2) {
                0 => return *v1,
                1 => return *v2,
                _ => return *v1,
            };
        });

        let mut has_one = false;
        let mut has_two = false;
        for v in &child.values {
            if *v == 1 {
                has_one = true;
            }

            if *v == 2 {
                has_two = true;
            }
        }

        assert!(has_one);
        assert!(has_two);
    }

    #[test]
    fn can_do_a_mutation() {
        // Arrange
        let mut dna1 = Dna {
            values: vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        };

        Dna::mutate(&mut dna1, &mut |&current_value| {
            return current_value * 2;
        });

        let mut has_two = false;

        for v in &dna1.values {
            if *v == 2 {
                has_two = true;
            }
        }

        assert!(has_two);
    }
}
