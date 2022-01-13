
pub mod individual {
    use rand::Rng;
    use crate::{TARGET_STRING, random_char};

    #[derive(Clone, Debug)]
    pub struct Individual {
        pub chromosome: String,
        pub fitness: u16,
    }

    impl Individual {
        pub fn new(chromosome: String) -> Individual {
            let mut individual = Individual { chromosome: chromosome, fitness: 0 };
            individual.calculate_fitness();
            individual
        }

        fn calculate_fitness(&mut self) {
            let mut matches: u16 = 0;
            for (index, char) in self.chromosome.chars().enumerate()  {
                if char != TARGET_STRING.chars().nth(index).unwrap() {
                    matches = matches + 1;
                }
            }
            self.fitness = matches;
        }

        pub fn create_gnome() -> String {
            let mut gnome = "".to_owned();
            for _i in 0..TARGET_STRING.len() {
                gnome.push(random_char())
            }
            gnome
        }

        pub fn mate(&self, partner: &Individual) -> Individual {
            let mut child_chromosome = "".to_owned();
            let chromosome_chars = self.chromosome.chars().collect::<Vec<char>>();
            let partner_chromosome_chars = partner.chromosome.chars().collect::<Vec<char>>();

            for i in 0..TARGET_STRING.len() {
                let chance = rand::thread_rng().gen::<f32>();
                if chance < 0.45 {
                    child_chromosome.push(chromosome_chars[i])
                } else if chance < 0.9 {
                    child_chromosome.push(partner_chromosome_chars[i])
                } else {
                    child_chromosome.push(random_char())
                }
            }

            Individual::new(child_chromosome)
        }
    }
}