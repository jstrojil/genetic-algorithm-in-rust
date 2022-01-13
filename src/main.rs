use rand::Rng;

mod individual;
use crate::individual::individual::Individual;

pub static POPULATION_SIZE: i32 = 50;
pub static TARGET_STRING: &str = "hrvfecpimildfyzarmmvhkwhrjupgzcpmkjyssznskg";

pub fn chars() -> Vec<char> {
    let mut chars: Vec<char> = ('a'..='z').collect();
    chars.push(' ');
    chars
}

fn random_char() -> char {
    chars()[rand::thread_rng().gen_range(0..chars().len())]
}

fn main() {
    let mut generation = 0;
    let mut population: Vec<Individual> = Vec::new();

    for _i in 0..POPULATION_SIZE {
        population.push(Individual::new(Individual::create_gnome()))
    }

    while true {
        population.sort_by(|a,b| a.fitness.cmp(&b.fitness));

        if population[0].fitness <= 0 {
            break;
        }

        let mut new_generation: Vec<Individual> = Vec::new();
        let s: usize = ((10*POPULATION_SIZE) / 100).try_into().unwrap();
        
        for i in 0..s {
            new_generation.push(population[i].clone());
        }

        let s: usize = ((90*POPULATION_SIZE) / 100).try_into().unwrap();

        for _i in 0..s {
            let parent1 = &population[rand::thread_rng().gen_range(0..10)];
            let parent2 = &population[rand::thread_rng().gen_range(0..10)];
            new_generation.push(parent1.mate(parent2));
        }
        population = new_generation.clone();

        println!("Generation = {}", generation);
        println!("String = {}", population[0].chromosome);
        println!("Fitness = {}", population[0].fitness);
        generation += 1;
    }
    println!("Generation = {}", generation);
    println!("String = {}", population[0].chromosome);
    println!("Fitness = {}", population[0].fitness);
}
