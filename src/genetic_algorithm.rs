use rand::prelude::*;

use crate::population::Population;
use crate::trates::Chromosome;

pub struct GeneticAlgorithm {
    rng: ThreadRng,
    uniform_rate: f64,
    mutation_rate: f64,
    torunament_size: i32,
    elitism: bool,
}

impl GeneticAlgorithm {
    pub fn new(uniform_rate: f64, mutation_rate: f64, torunament_size: i32, elitism: bool) -> Self {
        Self {
            rng: thread_rng(),
            uniform_rate,
            mutation_rate,
            torunament_size,
            elitism,
        }
    }

    pub fn evolve_population<T>(&mut self, pop: &mut Population<T>) -> Population<T>
    where
        T: Chromosome,
    {
        let mut new_population = Population::new(self.uniform_rate, self.mutation_rate);
        let elitism_offset = if self.elitism { 1 } else { 0 };

        if self.elitism {
            let fittest = pop.get_fittest();
            new_population.chromosomes.push(fittest);
        }

        for _ in elitism_offset..pop.chromosomes.len() {
            let chromosome1 = self.tournament_selection(pop);
            let chromosome2 = self.tournament_selection(pop);
            let new_chromosome = chromosome1.crossover(&chromosome2);
            new_population.chromosomes.push(new_chromosome);
        }

        new_population
            .chromosomes
            .iter_mut()
            .for_each(|c| c.mutate());

        new_population
    }

    fn tournament_selection<T: Chromosome>(&mut self, pop: &mut Population<T>) -> T {
        let mut tournament = Population::<T>::new(self.uniform_rate, self.mutation_rate);
        for _ in 0..self.torunament_size {
            let random_id: usize = self.rng.gen_range(0..pop.size());
            tournament
                .chromosomes
                .push(pop.chromosomes[random_id].clone());
        }
        tournament.get_fittest()
    }
}
